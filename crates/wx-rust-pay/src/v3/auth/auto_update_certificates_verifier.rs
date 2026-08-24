//! 平台证书自动更新验签器（v3/auth Java 命名镜像）。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.AutoUpdateCertificatesVerifier`：
//! 持有凭据（`Credentials`）与 APIv3 密钥，`verify` 使用前按
//! `minutesInterval` 间隔自动下载 `/v3/certificates` 刷新平台证书。
//!
//! 引擎复用 [`crate::util::crypto::WxPayAutoUpdateCertificatesVerifier`]
//! （下载/解密/存储替换/间隔判断/tryLock 全量实现，见
//! `tests/wx_pay_cert_verifier_test.rs`）；本文件提供 Java 命名镜像、
//! Java 构造器参数形态（credentials + apiV3Key + 间隔）与
//! [`super::Verifier`] 特性实现。
//!
//! ADAPTED（与 Java 的差异，语义保持一致）：
//! - Java 构造器内同步 `autoUpdateCert()`（失败仅告警、`instant = null`
//!   以便下次重试）；Rust HTTP 为异步 reqwest，构造为同步、证书下载延迟到
//!   首次 [`Self::check_and_auto_update`]（未成功下载前 `verify` 返回
//!   `false`，与 Java "verifier == null 时 warn 并返回 false" 一致）；
//! - Java `verify` 内部触发同步刷新；Rust `verify` 为纯同步查表，刷新由
//!   门面在异步上下文显式调用 [`Self::check_and_auto_update`]（对应 Java
//!   私有 `checkAndAutoUpdateCert` 的间隔判断 + tryLock + 失败仅告警）。

use std::sync::Arc;

use reqwest::Client;

use crate::config::WxPayConfig;
use crate::util::crypto::{
    CertificateUpdateResult, WxPayAutoUpdateCertificatesVerifier, WxPayCertVerifierError,
};

use super::{Verifier, WxPayValidCertificate};
use crate::v3::Credentials;

/// 证书更新间隔枚举（对应 Java
/// `AutoUpdateCertificatesVerifier.TimeInterval`：一小时/六小时/十二小时）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeInterval {
    /// 一小时（60 分钟）。
    OneHour,
    /// 六小时（360 分钟）。
    SixHours,
    /// 十二小时（720 分钟）。
    TwelveHours,
}

impl TimeInterval {
    /// 间隔分钟数（对应 Java `TimeInterval.getMinutes()`）。
    pub fn minutes(self) -> u64 {
        match self {
            Self::OneHour => 60,
            Self::SixHours => 60 * 6,
            Self::TwelveHours => 60 * 12,
        }
    }
}

/// 自动更新平台证书验签器（对应 Java
/// `AutoUpdateCertificatesVerifier implements Verifier`）。
pub struct AutoUpdateCertificatesVerifier {
    /// 下载引擎（证书存储/间隔判断/下载解密，对应 Java `verifier`/
    /// `instant`/`lock`/`autoUpdateCert`）。
    inner: Arc<WxPayAutoUpdateCertificatesVerifier>,
    /// 商户凭据（对应 Java 字段 `credentials`，构造器入参；下载签名材料
    /// 的 Rust 侧来源见 [`Self::check_and_auto_update`] 的 ADAPTED 说明）。
    credentials: Arc<dyn Credentials>,
    /// APIv3 密钥（对应 Java 字段 `apiV3Key`，`/v3/certificates` 解密用）。
    #[allow(dead_code)]
    api_v3_key: String,
}

impl std::fmt::Debug for AutoUpdateCertificatesVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoUpdateCertificatesVerifier")
            .field("serial_numbers", &self.inner.get_serial_numbers())
            .finish_non_exhaustive()
    }
}

impl AutoUpdateCertificatesVerifier {
    /// 以凭据 + APIv3 密钥 + 更新间隔构造（对应 Java 构造器
    /// `AutoUpdateCertificatesVerifier(credentials, apiV3Key, minutesInterval,
    /// payBaseUrl, wxPayHttpProxy)`；Rust 的下载 URL/私钥由配置提供，
    /// 代理由 reqwest 客户端承载，故此处只保留语义必需参数）。
    ///
    /// ADAPTED：Java 构造器内同步下载证书（失败仅告警）；Rust HTTP 为异步，
    /// 下载延迟到首次 [`Self::check_and_auto_update`]（引擎从 `config` 取
    /// mchId/certSerialNo/私钥/apiV3Key 构造签名，与 Java
    /// `credentials` + `apiV3Key` 入参等价）。
    pub fn new(credentials: Arc<dyn Credentials>, api_v3_key: &str, minutes_interval: u64) -> Self {
        Self {
            inner: Arc::new(WxPayAutoUpdateCertificatesVerifier::with_minutes_interval(
                minutes_interval,
            )),
            credentials,
            api_v3_key: api_v3_key.to_string(),
        }
    }

    /// 以默认间隔（[`TimeInterval::OneHour`]）构造（对应 Java
    /// `AutoUpdateCertificatesVerifier(credentials, apiV3Key, payBaseUrl)`）。
    pub fn with_default_interval(credentials: Arc<dyn Credentials>, api_v3_key: &str) -> Self {
        Self::new(credentials, api_v3_key, TimeInterval::OneHour.minutes())
    }

    /// 商户凭据（对应 Java 字段 `credentials` 的 getter）。
    pub fn credentials(&self) -> &Arc<dyn Credentials> {
        &self.credentials
    }

    /// APIv3 密钥（对应 Java 字段 `apiV3Key`）。
    pub fn api_v3_key(&self) -> &str {
        &self.api_v3_key
    }

    /// 更新间隔（分钟，对应 Java 字段 `minutesInterval`）。
    pub fn minutes_interval(&self) -> u64 {
        self.inner.minutes_interval()
    }

    /// 手动注入初始证书（Java 无直接等价，语义取整体替换证书存储）。
    pub fn set_certificates(&self, certificates: Vec<crate::util::crypto::WxPayCertificate>) {
        self.inner.set_certificates(certificates);
    }

    /// 按需刷新证书（对应 Java 私有 `checkAndAutoUpdateCert`：间隔未到或
    /// `tryLock` 失败时跳过；失败仅告警继续用旧证书，Rust 以 `Err` 返回、
    /// 不改动现有存储）。
    pub async fn check_and_auto_update<C: WxPayConfig + ?Sized>(
        &self,
        config: &C,
        http_client: &Client,
    ) -> Result<CertificateUpdateResult, WxPayCertVerifierError> {
        self.inner.check_and_auto_update(config, http_client).await
    }

    /// 立即下载并更新证书（对应 Java `autoUpdateCert`）。
    pub async fn auto_update_certificates<C: WxPayConfig + ?Sized>(
        &self,
        config: &C,
        http_client: &Client,
    ) -> Result<CertificateUpdateResult, WxPayCertVerifierError> {
        self.inner
            .auto_update_certificates(config, http_client)
            .await
    }

    /// 是否需要刷新（对应 Java 间隔判断 `instant == null ||
    /// instant.plus(minutesInterval).compareTo(now) <= 0`）。
    pub fn need_update(&self) -> bool {
        self.inner.need_update()
    }

    /// 存储中的全部序列号。
    pub fn get_serial_numbers(&self) -> Vec<String> {
        self.inner.get_serial_numbers()
    }
}

impl Verifier for AutoUpdateCertificatesVerifier {
    /// 按序列号验签（对应 Java `verify`：先 `checkAndAutoUpdateCert` 再委托
    /// 内部 `CertificatesVerifier.verify`；存储为空返回 `false`，对应 Java
    /// `verifier == null` 时 warn 并返回 false）。
    ///
    /// Rust 的网络刷新为异步，不在本同步方法内触发（见模块文档 ADAPTED）。
    fn verify(&self, serial_number: &str, message: &[u8], signature: &str) -> bool {
        self.inner.verify(serial_number, message, signature)
    }

    /// 返回首张有效期内证书（对应 Java `getValidCertificate`；存储为空时
    /// `Err`，对应抛 "No valid certificate available, please check your
    /// configuration or use fullPublicKeyModel mode"）。
    fn get_valid_certificate(&self) -> Result<WxPayValidCertificate, WxPayCertVerifierError> {
        self.inner
            .get_valid_certificate()
            .map(|c| WxPayValidCertificate::Certificate(Box::new(c)))
    }
}
