//! 微信支付 v3 证书加载工具。
//!
//! 对应 Java（weixin-java-pay）：
//! - `com.github.binarywang.wxpay.util.PemUtils`：`loadPrivateKey`（PKCS#8 私钥）、
//!   `loadPublicKey`（X.509 SubjectPublicKeyInfo 公钥）、`loadCertificate`
//!   （X.509 证书 + `checkValidity()` 有效期检查）；
//! - `com.github.binarywang.wxpay.config.WxPayConfig#p12ToPem`：PKCS#12 容器
//!   （apiclient_cert.p12）→ 私钥 + 证书，密码为商户号 mchId；
//! - `com.github.binarywang.wxpay.config.WxPayConfig#initApiV3HttpClient`：
//!   证书序列号提取 `cert.getSerialNumber().toString(16).toUpperCase()`。
//!
//! 选型说明（与 Java 的差异，均为 `ADAPTED`）：
//! - Java 用 JDK `KeyStore.getInstance("PKCS12")` 解析 p12，Rust 选 `p12` crate
//!   （纯 Rust、稳定版；RustCrypto 官方 `pkcs12` 最新版为 0.2.0-pre.0 预发布且依赖
//!   der 0.8-rc/x509-cert 0.3-rc 预发布链，故不用）。`p12` crate 支持微信 p12
//!   常用的 pbeWithSHA1And3KeyTripleDES-CBC（私钥）/pbeWithSHA1And40BitRC2-CBC
//!   （证书）与 SHA1 MAC；PBES2（OpenSSL 3 默认新格式）暂不支持，见模块文档。
//! - Java `X509Certificate` 对应 RustCrypto `x509-cert` crate（der 0.7/spki 0.7
//!   体系，与 rsa 0.9.10 同代）。

use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use wx_rust_common::error::{WxErrorException, WxRuntimeError};
use x509_cert::Certificate;
use x509_cert::der::{Decode, DecodePem, Encode};

/// 证书加载错误。
///
/// 对应 Java `PemUtils` 抛出的 `WxRuntimeException` 文案（"无效的密钥"、
/// "无效的密钥格式"、"无效的证书"、"证书已过期"、"证书尚未生效"）。
#[derive(Debug, thiserror::Error)]
pub enum WxPayCertError {
    /// 私钥解析失败（对应 Java `PemUtils.loadPrivateKey` 的 IOException 分支：
    /// "无效的密钥"）
    #[error("无效的密钥: {0}")]
    InvalidKey(String),
    /// 私钥/公钥格式错误（对应 Java `InvalidKeySpecException` 分支："无效的密钥格式"）
    #[error("无效的密钥格式: {0}")]
    InvalidKeyFormat(String),
    /// 证书解析失败（对应 Java `CertificateException` 分支："无效的证书"）
    #[error("无效的证书: {0}")]
    InvalidCertificate(String),
    /// 证书已过期（对应 Java `CertificateExpiredException`："证书已过期"）
    #[error("证书已过期")]
    CertificateExpired,
    /// 证书尚未生效（对应 Java `CertificateNotYetValidException`："证书尚未生效"）
    #[error("证书尚未生效")]
    CertificateNotYetValid,
    /// p12 容器解析失败（对应 Java `WxPayConfig.p12ToPem` 的
    /// "证书文件有问题，请核实！"）
    #[error("证书文件有问题，请核实！: {0}")]
    P12LoadFailed(String),
}

impl From<WxPayCertError> for WxErrorException {
    fn from(e: WxPayCertError) -> Self {
        // Java 侧 PemUtils 抛 WxRuntimeException、p12 解析抛 WxPayException，
        // Rust 统一映射为运行时错误
        WxErrorException::Runtime(WxRuntimeError::new(e.to_string()))
    }
}

/// X.509 商户/平台证书信息。
///
/// 对应 Java `java.security.cert.X509Certificate`（`PemUtils.loadCertificate`
/// 的返回值与 `p12ToPem` 返回数组的第二个元素）。
#[derive(Debug, Clone)]
pub struct WxPayCertificate {
    cert: Certificate,
    /// 证书序列号（对应 Java `cert.getSerialNumber().toString(16).toUpperCase()`：
    /// 十六进制**大写**、无前导零）
    serial_no: String,
}

impl WxPayCertificate {
    /// 由已解析的 X.509 证书构建（不检查有效期，对应 Java `p12ToPem` 语义）。
    fn from_cert(cert: Certificate) -> Self {
        let serial_no = serial_no_hex_upper(&cert);
        Self { cert, serial_no }
    }

    /// 证书序列号（十六进制大写、无前导零，对应 Java
    /// `cert.getSerialNumber().toString(16).toUpperCase()`）。
    pub fn serial_no(&self) -> &str {
        &self.serial_no
    }

    /// 证书公钥（对应 Java `cert.getPublicKey()`，签名验证/敏感信息加密使用）。
    pub fn public_key(&self) -> Result<RsaPublicKey, WxPayCertError> {
        let spki_der = self
            .cert
            .tbs_certificate()
            .subject_public_key_info()
            .to_der()
            .map_err(|e| WxPayCertError::InvalidCertificate(e.to_string()))?;
        RsaPublicKey::from_public_key_der(&spki_der)
            .map_err(|e| WxPayCertError::InvalidCertificate(e.to_string()))
    }

    /// 证书 DER 编码（对应 Java `cert.getEncoded()`）。
    pub fn to_der(&self) -> Result<Vec<u8>, WxPayCertError> {
        self.cert
            .to_der()
            .map_err(|e| WxPayCertError::InvalidCertificate(e.to_string()))
    }

    /// 证书是否在有效期内（对应 Java `X509Certificate.checkValidity()`）。
    pub fn check_validity(&self) -> Result<(), WxPayCertError> {
        check_validity(&self.cert)
    }
}

/// p12 容器解析结果（对应 Java `WxPayConfig.p12ToPem` 返回的
/// `Object[]{PrivateKey, X509Certificate}`）。
#[derive(Debug, Clone)]
pub struct WxPayP12Data {
    /// 商户 API 私钥（对应 Java `PrivateKey`）
    pub private_key: RsaPrivateKey,
    /// 商户证书（对应 Java `X509Certificate`）
    pub certificate: WxPayCertificate,
}

/// 加载 RSA 私钥（对应 Java `PemUtils.loadPrivateKey`）。
///
/// 语义与 Java 逐行对齐：去掉 `-----BEGIN PRIVATE KEY-----` /
/// `-----END PRIVATE KEY-----` 标记与全部空白字符，Base64 解码后按
/// PKCS#8 `PrivateKeyInfo` 解析（`rsa::RsaPrivateKey::from_pkcs8_der`）。
/// 与 Java 一致，仅支持 PKCS#8 格式（PKCS#1 `BEGIN RSA PRIVATE KEY` 会报
/// "无效的密钥格式"）。
///
/// # 参数
/// - `pem`：PKCS#8 PEM 文本或去标记的裸 Base64（UTF-8 字节）
pub fn load_private_key_from_pem(pem: &[u8]) -> Result<RsaPrivateKey, WxPayCertError> {
    let b64 = strip_pem_markers(
        pem,
        "-----BEGIN PRIVATE KEY-----",
        "-----END PRIVATE KEY-----",
    );
    let der = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| WxPayCertError::InvalidKey(e.to_string()))?;
    RsaPrivateKey::from_pkcs8_der(&der).map_err(|e| WxPayCertError::InvalidKeyFormat(e.to_string()))
}

/// 加载 RSA 公钥（对应 Java `PemUtils.loadPublicKey`）。
///
/// 语义与 Java 逐行对齐：去掉 `-----BEGIN PUBLIC KEY-----` /
/// `-----END PUBLIC KEY-----` 标记与全部空白字符，Base64 解码后按
/// X.509 `SubjectPublicKeyInfo` 解析。
///
/// # 参数
/// - `pem`：SPKI PEM 文本或去标记的裸 Base64（UTF-8 字节）
pub fn load_public_key_from_pem(pem: &[u8]) -> Result<RsaPublicKey, WxPayCertError> {
    let b64 = strip_pem_markers(
        pem,
        "-----BEGIN PUBLIC KEY-----",
        "-----END PUBLIC KEY-----",
    );
    let der = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| WxPayCertError::InvalidKeyFormat(e.to_string()))?;
    RsaPublicKey::from_public_key_der(&der)
        .map_err(|e| WxPayCertError::InvalidKeyFormat(e.to_string()))
}

/// 加载 X.509 证书（对应 Java `PemUtils.loadCertificate`）。
///
/// 语义与 Java 逐行对齐：按 X.509 解析 PEM 证书，并执行 `checkValidity()`
/// 有效期检查（已过期 → "证书已过期"；尚未生效 → "证书尚未生效"）。
///
/// # 参数
/// - `pem`：PEM 编码的 X.509 证书（UTF-8 字节）
pub fn load_certificate_from_pem(pem: &[u8]) -> Result<WxPayCertificate, WxPayCertError> {
    let cert = Certificate::from_pem(pem)
        .map_err(|e| WxPayCertError::InvalidCertificate(e.to_string()))?;
    check_validity(&cert)?;
    Ok(WxPayCertificate::from_cert(cert))
}

/// 加载 PKCS#12 容器中的私钥与证书（对应 Java `WxPayConfig.p12ToPem`）。
///
/// 语义与 Java 逐行对齐：
/// 1. `KeyStore.getInstance("PKCS12")` + `load(inputStream, password)`：
///    `p12::PFX::parse` + `verify_mac`（密码错误 → "证书文件有问题，请核实！"）；
/// 2. `alias = keyStore.aliases().nextElement()`：取文件中**第一个**私钥
///    （`key_bags().next()`）与第一个证书（`cert_x509_bags().next()`）。
///
/// 注意（与 Java 一致）：p12 路径**不**做证书有效期检查（Java `p12ToPem`
/// 未调用 `checkValidity`，有效期检查仅发生在 `PemUtils.loadCertificate`）。
///
/// # 参数
/// - `p12_der`：PKCS#12 文件 DER 字节（`WxPayConfig` 中 `keyPath`/`keyString`
///   base64/`keyContent` 三种通道最终得到的二进制内容）
/// - `password`：p12 密码，Java 语义为商户号 `mchId`
pub fn load_private_key_and_cert_from_p12(
    p12_der: &[u8],
    password: &str,
) -> Result<WxPayP12Data, WxPayCertError> {
    let pfx = p12::PFX::parse(p12_der).map_err(|e| WxPayCertError::P12LoadFailed(e.to_string()))?;
    // 对应 Java KeyStore.load 的密码校验（SHA-1 MAC）
    if !pfx.verify_mac(password) {
        return Err(WxPayCertError::P12LoadFailed(
            "p12 密码不正确（Java 语义：商户号 mchId）".to_string(),
        ));
    }
    // 对应 Java keyStore.getKey(alias, password)：第一个私钥（PKCS#8 DER）
    let key_der = pfx
        .key_bags(password)
        .map_err(|e| WxPayCertError::P12LoadFailed(e.to_string()))?
        .into_iter()
        .next()
        .ok_or_else(|| WxPayCertError::P12LoadFailed("p12 中未找到私钥".to_string()))?;
    let private_key = RsaPrivateKey::from_pkcs8_der(&key_der)
        .map_err(|e| WxPayCertError::InvalidKeyFormat(e.to_string()))?;

    // 对应 Java keyStore.getCertificate(alias)：第一个 X.509 证书
    let cert_der = pfx
        .cert_x509_bags(password)
        .map_err(|e| WxPayCertError::P12LoadFailed(e.to_string()))?
        .into_iter()
        .next()
        .ok_or_else(|| WxPayCertError::P12LoadFailed("p12 中未找到证书".to_string()))?;
    let cert = Certificate::from_der(&cert_der)
        .map_err(|e| WxPayCertError::InvalidCertificate(e.to_string()))?;

    Ok(WxPayP12Data {
        private_key,
        certificate: WxPayCertificate::from_cert(cert),
    })
}

/// 去掉 PEM 标记与全部空白字符（对应 Java `PemUtils` 的
/// `replace(...).replaceAll("\\s+", "")`）。
fn strip_pem_markers(pem: &[u8], begin: &str, end: &str) -> String {
    let text = String::from_utf8_lossy(pem);
    text.replace(begin, "")
        .replace(end, "")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// 证书序列号 → 十六进制大写（对应 Java
/// `cert.getSerialNumber().toString(16).toUpperCase()`）。
///
/// `BigInteger.toString(16)` 为无前导零的最小表示；DER INTEGER 对最高位为 1
/// 的正数会前置 `0x00`，故先跳过前导零字节。
fn serial_no_hex_upper(cert: &Certificate) -> String {
    let bytes = cert.tbs_certificate().serial_number().as_bytes();
    let trimmed = bytes
        .iter()
        .skip_while(|&&b| b == 0)
        .copied()
        .collect::<Vec<u8>>();
    let hex = hex::encode_upper(trimmed);
    // BigInteger.ZERO.toString(16) == "0"
    if hex.is_empty() { "0".to_string() } else { hex }
}

/// 证书有效期检查（对应 Java `X509Certificate.checkValidity(new Date())`：
/// 当前时间必须在 [notBefore, notAfter] 区间内）。
fn check_validity(cert: &Certificate) -> Result<(), WxPayCertError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    // x509-cert 的 Time::to_unix_duration（der UtcTime/GeneralizedTime 均支持）
    let not_before = cert
        .tbs_certificate()
        .validity()
        .not_before
        .to_unix_duration()
        .as_secs();
    let not_after = cert
        .tbs_certificate()
        .validity()
        .not_after
        .to_unix_duration()
        .as_secs();
    if now > not_after {
        return Err(WxPayCertError::CertificateExpired);
    }
    if now < not_before {
        return Err(WxPayCertError::CertificateNotYetValid);
    }
    Ok(())
}
