//! v3 验签器接口。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.Verifier`：
//!
//! ```java
//! public interface Verifier {
//!   boolean verify(String serialNumber, byte[] message, String signature);
//!   X509Certificate getValidCertificate();
//!   default void setOtherVerifier(Verifier verifier) {};
//! }
//! ```
//!
//! ADAPTED：
//! - Java `getValidCertificate` 返回 `X509Certificate`（两个运行时形态：
//!   真实平台证书 / `X509PublicCertificate` 公钥包装）；Rust 以
//!   [`WxPayValidCertificate`] 枚举承载同一多态；
//! - Java `getValidCertificate` 无有效证书时抛 unchecked 异常
//!   （`NoSuchElementException`/`WxRuntimeException`）；Rust 以
//!   `Result` 表达。

use std::sync::Arc;

use rsa::RsaPublicKey;

use crate::util::crypto::{WxPayCertError, WxPayCertVerifierError, WxPayCertificate};

use super::x509_public_certificate::X509PublicCertificate;

/// 验签器返回的"有效证书"（对应 Java `X509Certificate` 的两个运行时形态）。
///
/// `Certificate` 装箱以压缩枚举尺寸（`WxPayCertificate` 内含完整 X.509
/// 结构约 0.5 KiB，clippy large_enum_variant）。
#[derive(Debug, Clone)]
pub enum WxPayValidCertificate {
    /// X.509 平台证书（`CertificatesVerifier`/`AutoUpdateCertificatesVerifier`
    /// 返回，对应 Java 真实 `X509Certificate`）。
    Certificate(Box<WxPayCertificate>),
    /// 公钥模式承载对象（`PublicCertificateVerifier` 返回，对应 Java
    /// `X509PublicCertificate`）。
    PublicCertificate(X509PublicCertificate),
}

impl WxPayValidCertificate {
    /// 证书/公钥序列号（对应 Java `getSerialNumber()`）。
    ///
    /// Certificate 形态为十六进制大写串（[`WxPayCertificate::serial_no`]，
    /// 对应 Java `getSerialNumber().toString(16).toUpperCase()`）；
    /// PublicCertificate 形态去除 `PUB_KEY_ID_` 前缀的公钥 ID。
    pub fn serial_no(&self) -> &str {
        match self {
            Self::Certificate(cert) => cert.serial_no(),
            Self::PublicCertificate(pc) => pc.serial_number_id(),
        }
    }

    /// 公钥（对应 Java `getPublicKey()`，验签/敏感信息加密使用）。
    pub fn public_key(&self) -> Result<RsaPublicKey, WxPayCertError> {
        match self {
            Self::Certificate(cert) => cert.public_key(),
            Self::PublicCertificate(pc) => Ok(pc.public_key().clone()),
        }
    }

    /// 有效期检查（对应 Java `checkValidity()`）。
    ///
    /// 公钥形态恒为 `Ok`（Java `X509PublicCertificate.checkValidity` 为空实现）。
    pub fn check_validity(&self) -> Result<(), WxPayCertError> {
        match self {
            Self::Certificate(cert) => cert.check_validity(),
            Self::PublicCertificate(_) => Ok(()),
        }
    }
}

/// v3 验签器（对应 Java `v3/auth/Verifier` 接口）。
pub trait Verifier: Send + Sync {
    /// 按序列号验签（对应 Java `verify(serialNumber, message, signature)`）。
    ///
    /// 验签失败/未知序列号返回 `false`（与 Java bool 语义一致）。
    fn verify(&self, serial_number: &str, message: &[u8], signature: &str) -> bool;

    /// 返回首张有效期内证书（对应 Java `getValidCertificate()`；无有效证书
    /// 时 `Err`，对应 Java 抛 `NoSuchElementException`/
    /// `WxRuntimeException`）。
    fn get_valid_certificate(&self) -> Result<WxPayValidCertificate, WxPayCertVerifierError>;

    /// 设置兜底验证器（对应 Java `default void setOtherVerifier(Verifier)`
    /// ——默认空实现，仅 [`crate::v3::auth::PublicCertificateVerifier`]
    /// 覆写）。
    fn set_other_verifier(&self, _verifier: Option<Arc<dyn Verifier>>) {}
}
