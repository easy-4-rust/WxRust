//! 平台证书自动更新与验签路由。
//!
//! 对应 Java（weixin-java-pay）`v3/auth/` 包：
//! - [`WxPayCertificatesVerifier`] 对应 `CertificatesVerifier`：`certificateMap`
//!   （HashMap<BigInteger, X509Certificate>，Rust 以序列号十六进制大写字符串为
//!   key）+ `verify(serialNumber, message, signature)` 按序列号路由证书公钥做
//!   SHA256withRSA 验签、`getValidCertificate` 返回首张有效期内证书；
//! - [`WxPayAutoUpdateCertificatesVerifier`] 对应 `AutoUpdateCertificatesVerifier`：
//!   `autoUpdateCert` 下载 `/v3/certificates`（带 v3 Authorization 签名头）→
//!   `deserializeToCerts`（AES-256-GCM 解密 + `PemUtils.loadCertificate` +
//!   `checkValidity()` 跳过失效证书）→ 整体替换证书存储（`CertificatesVerifier`）；
//!   `checkAndAutoUpdateCert` 按 `minutesInterval` 间隔刷新，失败仅 warn 不抛出。
//!
//! ADAPTED 说明（与 Java 的差异，均为接口层面调整、语义保持一致）：
//! - Java `verify()` 内部会同步发起 HTTP 证书刷新；Rust 侧验签接口为同步闭包
//!   （`Fn(&str, &[u8], &str) -> bool`，见 `wx_pay_notify_utils`），HTTP 为异步
//!   reqwest，故网络刷新由门面（P2a）在异步上下文显式调用
//!   [`WxPayAutoUpdateCertificatesVerifier::check_and_auto_update`]（对应 Java
//!   私有方法 `checkAndAutoUpdateCert` 的间隔判断 + tryLock + 失败仅告警语义），
//!   `verify()` 本身不触发网络请求；
//! - Java `autoUpdateCert` 失败时抛 WxRuntimeException（`checkAndAutoUpdateCert`
//!   仅捕获 IOException/GeneralSecurityException 并 `log.warn`）；Rust 统一以
//!   `Result` 返回更新结果（不 panic/不抛出），调用方按 Java 语义将 `Err` 视为
//!   告警并继续使用旧证书。

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};

use reqwest::Client;
use serde::Deserialize;

use wx_rust_common::error::{WxErrorException, WxRuntimeError};

use crate::config::WxPayConfig;
use crate::util::crypto::wx_pay_cert_utils::{
    WxPayCertError, WxPayCertificate, load_certificate_from_pem,
};
use crate::util::crypto::wx_pay_v3_crypto_utils::{
    WxV3CryptoError, aes_gcm_decrypt, create_authorization_header, gen_nonce_str, gen_timestamp,
    verify_sha256_rsa,
};

/// 证书下载地址（对应 Java `AutoUpdateCertificatesVerifier.CERT_DOWNLOAD_PATH`）。
pub const CERT_DOWNLOAD_PATH: &str = "/v3/certificates";

/// 默认证书更新间隔（分钟，对应 Java `TimeInterval.OneHour = 60`）。
pub const DEFAULT_UPDATE_INTERVAL_MINUTES: u64 = 60;

/// 证书验签/自动更新错误。
///
/// 文案对应 Java 各路径的异常消息（`WxRuntimeException`/`NoSuchElementException`
/// /`IllegalArgumentException` 等）。
#[derive(Debug, thiserror::Error)]
pub enum WxPayCertVerifierError {
    /// 配置缺失/非法（对应 Java `WxPayCredentials` 构造器对 mchId/私钥的校验）
    #[error("无效的配置: {0}")]
    InvalidConfig(String),
    /// 证书下载失败（对应 Java `autoUpdateCert` 非 200 时
    /// `throw new WxRuntimeException(getErrorMsg(body))`，文案取响应体
    /// `message` 字段，缺省 "update cert failed"）
    #[error("证书下载失败: {0}")]
    DownloadFailed(String),
    /// 下载响应中的证书列表为空（对应 Java
    /// `throw new WxRuntimeException("Cert list is empty")`）
    #[error("Cert list is empty")]
    CertListEmpty,
    /// `/v3/certificates` 响应体 JSON 解析失败
    #[error("证书响应解析失败: {0}")]
    CertResponseParse(String),
    /// 解密后的证书 PEM 解析失败（对应 Java `PemUtils.loadCertificate` 的
    /// `CertificateException` 分支，会使整个更新失败并保留旧证书）
    #[error("证书解析失败: {0}")]
    CertParse(String),
    /// HTTP 请求执行失败（网络/超时，对应 Java `IOException` 分支）
    #[error("HTTP请求失败: {0}")]
    Http(String),
    /// 无任何证书（对应 Java `AutoUpdateCertificatesVerifier.getValidCertificate`：
    /// `throw new WxRuntimeException("No valid certificate available, please check
    /// your configuration or use fullPublicKeyModel mode")`）
    #[error("没有有效的证书可用，请检查配置或使用公钥模式")]
    NoCertificates,
    /// 存储中无有效期内证书（对应 Java `CertificatesVerifier.getValidCertificate`：
    /// `throw new NoSuchElementException("没有有效的微信支付平台证书")`）
    #[error("没有有效的微信支付平台证书")]
    NoValidCertificate,
    /// 加解密/签名错误（复用 `wx_pay_v3_crypto_utils` 的错误）
    #[error("{0}")]
    Crypto(#[from] WxV3CryptoError),
    /// 证书加载错误（复用 `wx_pay_cert_utils` 的错误）
    #[error("{0}")]
    Cert(#[from] WxPayCertError),
}

impl From<WxPayCertVerifierError> for WxErrorException {
    fn from(e: WxPayCertVerifierError) -> Self {
        // Java 侧抛 WxRuntimeException/NoSuchElementException，统一映射为运行时错误
        WxErrorException::Runtime(WxRuntimeError::new(e.to_string()))
    }
}

/// 证书自动更新结果（Java `autoUpdateCert` 无返回值，Rust 以结果对象表达
/// 更新状态，便于门面记录与测试断言）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateUpdateResult {
    /// 本次是否真正执行了证书下载（false = 间隔未到/并发更新被跳过）
    pub refreshed: bool,
    /// 本次下载并写入存储的证书数量（对应 Java `newCertList.size()`）
    pub updated_cert_count: usize,
    /// 更新后证书存储中的序列号列表（对应 Java `certificateMap.keySet()`）
    pub serial_numbers: Vec<String>,
}

impl CertificateUpdateResult {
    /// 未触发下载的跳过结果（对应 Java `checkAndAutoUpdateCert` 中
    /// 间隔未到或 `lock.tryLock()` 失败时直接使用现有证书）。
    fn skipped(serial_numbers: Vec<String>) -> Self {
        Self {
            refreshed: false,
            updated_cert_count: 0,
            serial_numbers,
        }
    }
}

/// 平台证书存储 + 按序列号路由的验签器（对应 Java `CertificatesVerifier`）。
///
/// `certificateMap` 以证书序列号（十六进制大写、无前导零，同
/// [`WxPayCertificate::serial_no`] 的格式）为 key。
#[derive(Debug, Clone, Default)]
pub struct WxPayCertificatesVerifier {
    certificates: HashMap<String, WxPayCertificate>,
}

impl WxPayCertificatesVerifier {
    /// 空证书存储（对应 Java 无参构造后的空 map）。
    pub fn new() -> Self {
        Self::default()
    }

    /// 由证书列表构建（对应 Java `CertificatesVerifier(List<X509Certificate>)`
    /// 构造：逐个 `certificates.put(cert.getSerialNumber(), cert)`）。
    pub fn with_certificates(certificates: Vec<WxPayCertificate>) -> Self {
        let mut verifier = Self::new();
        verifier.set_certificates(certificates);
        verifier
    }

    /// 按序列号取证书（对应 Java `certificateMap.get(serial)` 的 Rust 表达；
    /// 序列号比较大小写不敏感，与 Java `new BigInteger(serialNumber, 16)`
    /// 的 key 语义一致）。
    pub fn get_certificate(&self, serial_no: &str) -> Option<&WxPayCertificate> {
        self.certificates.get(&serial_no.to_ascii_uppercase())
    }

    /// 存储中的全部序列号（对应 Java `certificateMap.keySet()`）。
    pub fn get_serial_numbers(&self) -> Vec<String> {
        self.certificates.keys().cloned().collect()
    }

    /// 存储中的证书数量。
    pub fn len(&self) -> usize {
        self.certificates.len()
    }

    /// 存储是否为空。
    pub fn is_empty(&self) -> bool {
        self.certificates.is_empty()
    }

    /// 整体替换证书存储（对应 Java `CertificatesVerifier` 构造/重新赋值
    /// `certificateMap` 的语义：传入列表整体覆盖旧存储）。
    pub fn set_certificates(&mut self, certificates: Vec<WxPayCertificate>) {
        self.certificates.clear();
        for cert in certificates {
            self.certificates.insert(cert.serial_no().to_string(), cert);
        }
    }

    /// 按序列号路由公钥验签（对应 Java `CertificatesVerifier.verify`：
    /// `certificates.containsKey(val) && verify(cert, message, signature)`）。
    ///
    /// 未知序列号返回 `false`（Java `containsKey` 短路语义，不抛异常）；
    /// Base64 解码失败/验签过程错误同样归为 `false`（Java 侧此类错误抛
    /// unchecked 异常，Rust bool 接口统一按验签失败处理）。
    pub fn verify(&self, serial_no: &str, message: &[u8], signature: &str) -> bool {
        let Some(cert) = self.get_certificate(serial_no) else {
            return false;
        };
        let Ok(public_key) = cert.public_key() else {
            return false;
        };
        verify_sha256_rsa(&public_key, message, signature).unwrap_or(false)
    }

    /// 返回存储中首张有效期内证书（对应 Java `CertificatesVerifier.
    /// getValidCertificate`：遍历 `checkValidity()`，全失效时抛
    /// `NoSuchElementException("没有有效的微信支付平台证书")`）。
    pub fn get_valid_certificate(&self) -> Result<&WxPayCertificate, WxPayCertVerifierError> {
        for cert in self.certificates.values() {
            if cert.check_validity().is_ok() {
                return Ok(cert);
            }
        }
        Err(WxPayCertVerifierError::NoValidCertificate)
    }
}

/// `/v3/certificates` 响应体（对应 Java `deserializeToCerts` 中
/// `json.getAsJsonArray("data")` 的解析）。
#[derive(Debug, Deserialize)]
struct CertificatesResponse {
    data: Option<Vec<CertificateItem>>,
}

/// 响应 data[] 单项（对应 Java 遍历 `dataNode` 的 `JsonObject`；`serial_no`/
/// `effective_time`/`expire_time` 字段 Java 侧不参与解密，仅 `encrypt_certificate`
/// 被使用）。
#[derive(Debug, Deserialize)]
struct CertificateItem {
    #[serde(rename = "serial_no")]
    #[allow(dead_code)]
    serial_no: Option<String>,
    #[serde(rename = "effective_time")]
    #[allow(dead_code)]
    effective_time: Option<String>,
    #[serde(rename = "expire_time")]
    #[allow(dead_code)]
    expire_time: Option<String>,
    #[serde(rename = "encrypt_certificate")]
    encrypt_certificate: EncryptCertificate,
}

/// data[] 单项的加密证书对象（对应 Java
/// `encryptCertificateNode`：`algorithm`/`associated_data`/`nonce`/`ciphertext`）。
#[derive(Debug, Deserialize)]
struct EncryptCertificate {
    #[allow(dead_code)]
    algorithm: Option<String>,
    #[serde(rename = "associated_data")]
    associated_data: Option<String>,
    nonce: Option<String>,
    ciphertext: Option<String>,
}

/// 反序列化并解密 `/v3/certificates` 响应（对应 Java
/// `AutoUpdateCertificatesVerifier.deserializeToCerts(apiV3Key, body)`）。
///
/// 语义与 Java 逐行对齐：
/// 1. 解析 JSON，`data` 缺失或为空 → 返回空列表（Java `dataNode == null`
///    返回 `Collections.emptyList()`）；
/// 2. 逐项用 apiV3Key 对 `encrypt_certificate` 做 AES-256-GCM 解密
///    （`AesUtils.decryptToString`，ciphertext 先去除空格，与 Java
///    `StringUtils.remove(ciphertext, " ")` 一致），得到证书 PEM 文本；
/// 3. `PemUtils.loadCertificate` 解析 X.509 证书并 `checkValidity()`；
///    已过期/尚未生效的证书被跳过（Java catch `CertificateExpiredException |
///    CertificateNotYetValidException` 后 `continue`），其余解析错误使整个
///    更新失败（保留旧证书）。
///
/// # 参数
/// - `api_v3_key`：APIv3 密钥（32 字节，对应 Java `AesUtils` 构造参数）
/// - `body`：下载接口响应体原文（JSON）
pub fn deserialize_to_certs(
    api_v3_key: &str,
    body: &str,
) -> Result<Vec<WxPayCertificate>, WxPayCertVerifierError> {
    let response: CertificatesResponse = serde_json::from_str(body)
        .map_err(|e| WxPayCertVerifierError::CertResponseParse(e.to_string()))?;
    let Some(data) = response.data else {
        return Ok(Vec::new());
    };

    let mut certs = Vec::with_capacity(data.len());
    for item in data {
        let enc = item.encrypt_certificate;
        // 对应 Java：associated_data/nonce 缺失时 NPE 使更新失败；Rust 明确报错
        let associated_data = enc.associated_data.ok_or_else(|| {
            WxPayCertVerifierError::CertResponseParse(
                "缺少 encrypt_certificate.associated_data".into(),
            )
        })?;
        let nonce = enc.nonce.ok_or_else(|| {
            WxPayCertVerifierError::CertResponseParse("缺少 encrypt_certificate.nonce".into())
        })?;
        let ciphertext = enc.ciphertext.ok_or_else(|| {
            WxPayCertVerifierError::CertResponseParse("缺少 encrypt_certificate.ciphertext".into())
        })?;
        // 对应 Java `StringUtils.remove(ciphertext, " ")`：去空格后 Base64 解码
        let ciphertext = ciphertext.replace(' ', "");
        let cert_pem = aes_gcm_decrypt(api_v3_key, &associated_data, &nonce, &ciphertext)?;

        // PemUtils.loadCertificate：解析 + checkValidity；失效证书跳过
        let cert = match load_certificate_from_pem(cert_pem.as_bytes()) {
            Ok(cert) => cert,
            Err(WxPayCertError::CertificateExpired | WxPayCertError::CertificateNotYetValid) => {
                continue;
            }
            Err(e) => return Err(WxPayCertVerifierError::Cert(e)),
        };
        certs.push(cert);
    }
    Ok(certs)
}

/// 自动更新验签器（对应 Java `AutoUpdateCertificatesVerifier`）。
///
/// 持有证书存储（内部 [`WxPayCertificatesVerifier`]）、更新间隔与上次更新时间；
/// `verify`/`get_valid_certificate` 语义与 Java 对齐，网络刷新由
/// [`Self::check_and_auto_update`]（对应 Java 私有 `checkAndAutoUpdateCert`）在
/// 异步上下文触发。
pub struct WxPayAutoUpdateCertificatesVerifier {
    inner: RwLock<WxPayCertificatesVerifier>,
    /// 证书更新间隔（分钟，对应 Java `minutesInterval`，默认
    /// `TimeInterval.OneHour`）
    minutes_interval: u64,
    /// 上次更新时间（对应 Java `volatile Instant instant`；`None` 表示
    /// 从未成功更新过，下次使用立即刷新——对应 Java 构造失败时
    /// `instant = null` 的语义）
    last_updated: RwLock<Option<SystemTime>>,
    /// 更新互斥锁（对应 Java `ReentrantLock` 的 `tryLock` 语义；用 tokio 异步
    /// 锁以在 `await` 期间保持互斥——与 Java 持锁完成整个 HTTP 下载一致）
    update_lock: tokio::sync::Mutex<()>,
}

impl std::fmt::Debug for WxPayAutoUpdateCertificatesVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WxPayAutoUpdateCertificatesVerifier")
            .field("minutes_interval", &self.minutes_interval)
            .field("serial_numbers", &self.get_serial_numbers())
            .finish_non_exhaustive()
    }
}

impl Default for WxPayAutoUpdateCertificatesVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl WxPayAutoUpdateCertificatesVerifier {
    /// 以默认间隔（60 分钟，对应 Java `TimeInterval.OneHour`）构建。
    pub fn new() -> Self {
        Self::with_minutes_interval(DEFAULT_UPDATE_INTERVAL_MINUTES)
    }

    /// 以指定间隔（分钟）构建（对应 Java
    /// `AutoUpdateCertificatesVerifier(credentials, apiV3Key, minutesInterval,
    /// payBaseUrl)`；credentials/密钥在 Rust 由门面经 `check_and_auto_update`
    /// 传入配置，见模块文档 ADAPTED 说明）。
    pub fn with_minutes_interval(minutes_interval: u64) -> Self {
        Self {
            inner: RwLock::new(WxPayCertificatesVerifier::new()),
            minutes_interval,
            last_updated: RwLock::new(None),
            update_lock: tokio::sync::Mutex::new(()),
        }
    }

    /// 手动注入初始证书（对应任务要求的"可注入初始证书"；Java 无直接等价
    /// 方法，语义取 `CertificatesVerifier` 构造的整体替换）。注入成功视为一次
    /// 有效更新，重置上次更新时间（与 Java 构造成功后 `instant = now` 一致）。
    pub fn set_certificates(&self, certificates: Vec<WxPayCertificate>) {
        self.inner
            .write()
            .expect("证书存储写锁")
            .set_certificates(certificates);
        *self.last_updated.write().expect("更新时间写锁") = Some(SystemTime::now());
    }

    /// 按序列号路由验签（对应 Java `AutoUpdateCertificatesVerifier.verify`：
    /// 先 `checkAndAutoUpdateCert` 再委托内部 `CertificatesVerifier.verify`）。
    ///
    /// 存储为空时返回 `false`（对应 Java `verifier == null` 时
    /// `log.warn("No valid certificate available for verification")` 并
    /// 返回 false）。注意：Rust 的 HTTP 刷新为异步（见模块文档），不会在
    /// 本同步方法内触发。
    pub fn verify(&self, serial_no: &str, message: &[u8], signature: &str) -> bool {
        let inner = self.inner.read().expect("证书存储读锁");
        if inner.is_empty() {
            return false;
        }
        inner.verify(serial_no, message, signature)
    }

    /// 按序列号取证书（委托内部存储）。
    pub fn get_certificate(&self, serial_no: &str) -> Option<WxPayCertificate> {
        self.inner
            .read()
            .expect("证书存储读锁")
            .get_certificate(serial_no)
            .cloned()
    }

    /// 存储中的全部序列号。
    pub fn get_serial_numbers(&self) -> Vec<String> {
        self.inner
            .read()
            .expect("证书存储读锁")
            .get_serial_numbers()
    }

    /// 返回首张有效期内证书（对应 Java
    /// `AutoUpdateCertificatesVerifier.getValidCertificate`：先刷新，存储为空时
    /// 抛 "No valid certificate available, please check your configuration or
    /// use fullPublicKeyModel mode"）。
    pub fn get_valid_certificate(&self) -> Result<WxPayCertificate, WxPayCertVerifierError> {
        let inner = self.inner.read().expect("证书存储读锁");
        if inner.is_empty() {
            return Err(WxPayCertVerifierError::NoCertificates);
        }
        inner.get_valid_certificate().cloned()
    }

    /// 是否需要刷新证书（对应 Java `checkAndAutoUpdateCert` 的间隔判断：
    /// `instant == null || instant.plus(minutesInterval).compareTo(now) <= 0`）。
    pub fn need_update(&self) -> bool {
        // Option<SystemTime>: Copy，直接拷贝出读锁再判断
        let last = *self.last_updated.read().expect("更新时间读锁");
        match last {
            None => true,
            Some(prev) => SystemTime::now()
                .duration_since(prev)
                .map(|elapsed| elapsed >= Duration::from_secs(self.minutes_interval * 60))
                .unwrap_or(true),
        }
    }

    /// 按需刷新证书（对应 Java 私有方法 `checkAndAutoUpdateCert`）：
    /// 1. 间隔未到 → 直接返回跳过结果；
    /// 2. `tryLock` 失败（其他线程正在更新）→ 跳过，使用现有证书；
    /// 3. 执行 [`Self::auto_update_certificates`]，失败返回 `Err`（调用方按
    ///    Java 语义仅告警、继续使用旧证书）。
    pub async fn check_and_auto_update<C: WxPayConfig + ?Sized>(
        &self,
        config: &C,
        http_client: &Client,
    ) -> Result<CertificateUpdateResult, WxPayCertVerifierError> {
        if !self.need_update() {
            return Ok(CertificateUpdateResult::skipped(self.get_serial_numbers()));
        }
        let Ok(_guard) = self.update_lock.try_lock() else {
            // 对应 Java `if (lock.tryLock())` 失败：直接使用现有证书
            return Ok(CertificateUpdateResult::skipped(self.get_serial_numbers()));
        };
        // 双重检查：等待锁期间其他线程可能已完成更新
        if !self.need_update() {
            return Ok(CertificateUpdateResult::skipped(self.get_serial_numbers()));
        }
        self.auto_update_certificates(config, http_client).await
    }

    /// 下载并更新平台证书（对应 Java `AutoUpdateCertificatesVerifier.autoUpdateCert`）。
    ///
    /// 流程与 Java 逐行对齐：
    /// 1. 用商户 API 私钥构造 v3 Authorization 头（`WxPayCredentials.getToken`：
    ///    `WECHATPAY2-SHA256-RSA2048 mchid="..",nonce_str="..",timestamp="..",
    ///    serial_no="..",signature=".."`，签名串 `GET\n/v3/certificates\nTS\n
    ///    NONCE\n\n`）；
    /// 2. GET `payBaseUrl + CERT_DOWNLOAD_PATH`（`payBaseUrl` 为
    ///    `getApiHostWithPathPrefix()`，即 host + 路径前缀），头带
    ///    `Accept: application/json`（`strictlyNeedWechatPaySerial` 开启时
    ///    附加 `Wechatpay-Serial`）；
    /// 3. 非 200 → `Err(DownloadFailed)`（消息取响应体 `message` 字段，
    ///    对应 Java `getErrorMsg`，缺省 "update cert failed"）；
    /// 4. 200 → `deserialize_to_certs` 解密，列表为空 → `Err(CertListEmpty)`
    ///    （对应 Java `throw new WxRuntimeException("Cert list is empty")`）；
    /// 5. 整体替换证书存储（对应 Java `this.verifier = new CertificatesVerifier(
    ///    newCertList)`）并记录上次更新时间。
    ///
    /// 下载失败不抛出（Java 语义：仅 warn 并保留旧证书），以 `Result` 返回
    /// 更新结果；本方法失败不会改动现有证书存储。
    ///
    /// # 参数
    /// - `config`：微信支付配置（mchId/certSerialNo/apiV3Key/私钥 PEM，
    ///   对应 Java `WxPayCredentials` + `WxPayConfig`）
    /// - `http_client`：复用门面的 reqwest 客户端（对应 Java
    ///   `WxPayV3HttpClientBuilder.build()`）
    pub async fn auto_update_certificates<C: WxPayConfig + ?Sized>(
        &self,
        config: &C,
        http_client: &Client,
    ) -> Result<CertificateUpdateResult, WxPayCertVerifierError> {
        // ---- 1. 商户凭据与 Authorization 头（对应 Java WxPayCredentials + PrivateKeySigner）----
        let mch_id = config
            .mch_id()
            .ok_or_else(|| WxPayCertVerifierError::InvalidConfig("缺少商户号 mchId".into()))?;
        let cert_serial_no = config.cert_serial_no().ok_or_else(|| {
            WxPayCertVerifierError::InvalidConfig("缺少商户证书序列号 certSerialNo".into())
        })?;
        let private_key_pem = config.private_key().ok_or_else(|| {
            WxPayCertVerifierError::InvalidConfig("缺少商户 API 私钥 privateKey".into())
        })?;
        let api_v3_key = config.api_v3_key().ok_or_else(|| {
            WxPayCertVerifierError::InvalidConfig("缺少 APIv3 密钥 apiV3Key".into())
        })?;
        let private_key = load_private_key(private_key_pem)?;

        // 对应 Java `payBaseUrl + CERT_DOWNLOAD_PATH`（payBaseUrl = getApiHostWithPathPrefix）
        let base_url = config.api_host_with_path_prefix();
        let url = format!("{base_url}{CERT_DOWNLOAD_PATH}");
        // 签名用规范化 URL：Java `WxPayCredentials.buildMessage` 对请求 URI 的
        // rawPath 调用 `stripPathPrefix`（VerifierBuilder 将 apiHostUrlPath 作为
        // signUriStripPrefix 传入，签名串中剥离开头路径前缀），故此处恒为
        // `/v3/certificates`（不含路径前缀），实际请求 URL 仍带前缀
        let canonical_url = CERT_DOWNLOAD_PATH;

        let timestamp = gen_timestamp();
        let nonce = gen_nonce_str();
        let authorization = create_authorization_header(
            mch_id,
            cert_serial_no,
            &private_key,
            "GET",
            canonical_url,
            timestamp,
            &nonce,
            "",
        )?;

        // ---- 2. 执行下载（对应 Java `httpGet` + `Accept: application/json`）----
        let mut request = http_client.get(&url).header("Accept", "application/json");
        // 对应 Java SignatureExec 的严格模式：v3 请求附加 Wechatpay-Serial 头
        if config.strictly_need_wechat_pay_serial() {
            request = request.header("Wechatpay-Serial", cert_serial_no);
        }
        let response = request
            .header("Authorization", authorization)
            .send()
            .await
            .map_err(|e| WxPayCertVerifierError::Http(e.to_string()))?;
        let status_code = response.status().as_u16() as i32;
        let body = response
            .text()
            .await
            .map_err(|e| WxPayCertVerifierError::Http(e.to_string()))?;

        // ---- 3/4. 状态码与解密（对应 Java autoUpdateCert 的 200 分支）----
        if status_code != 200 {
            return Err(WxPayCertVerifierError::DownloadFailed(error_msg_from_body(
                &body,
            )));
        }
        let new_certs = deserialize_to_certs(api_v3_key, &body)?;
        if new_certs.is_empty() {
            return Err(WxPayCertVerifierError::CertListEmpty);
        }

        // ---- 5. 整体替换存储并记录更新时间 ----
        let updated_cert_count = new_certs.len();
        self.inner
            .write()
            .expect("证书存储写锁")
            .set_certificates(new_certs);
        *self.last_updated.write().expect("更新时间写锁") = Some(SystemTime::now());

        Ok(CertificateUpdateResult {
            refreshed: true,
            updated_cert_count,
            serial_numbers: self.get_serial_numbers(),
        })
    }
}

/// 解析商户 API 私钥 PEM（对应 Java `PrivateKeySigner` 构造时
/// `config.getPrivateKey()` 已解析的 `PrivateKey`；Rust 按 PEM 文本解析）。
fn load_private_key(pem: &str) -> Result<rsa::RsaPrivateKey, WxPayCertVerifierError> {
    crate::util::crypto::wx_pay_cert_utils::load_private_key_from_pem(pem.as_bytes())
        .map_err(WxPayCertVerifierError::from)
}

/// 提取下载失败的错误消息（对应 Java `AutoUpdateCertificatesVerifier.getErrorMsg`：
/// 解析响应体 JSON 的 `message` 字段，缺省 "update cert failed"）。
fn error_msg_from_body(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.get("message")
                .and_then(|m| m.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "update cert failed".to_string())
}
