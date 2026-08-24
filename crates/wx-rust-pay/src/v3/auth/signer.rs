//! v3 签名器接口。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.Signer`：
//!
//! ```java
//! public interface Signer {
//!   SignatureResult sign(byte[] message);
//!   class SignatureResult { String sign; String certificateSerialNumber; }
//! }
//! ```
//!
//! ADAPTED：Java `sign` 抛 unchecked `WxRuntimeException`；Rust 以
//! `Result<SignatureResult, WxV3CryptoError>` 表达同一失败路径
//! （"当前Java环境不支持SHA256withRSA"/"无效的私钥"/"签名计算失败"）。

use crate::util::crypto::WxV3CryptoError;

/// 签名结果（对应 Java `Signer.SignatureResult`）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureResult {
    /// Base64 编码签名（对应 Java 字段 `sign`）。
    pub sign: String,
    /// 签名所用证书/公钥序列号（对应 Java 字段 `certificateSerialNumber`，
    /// 作为 Authorization token 的 `serial_no`）。
    pub certificate_serial_number: String,
}

impl SignatureResult {
    /// 构造签名结果（对应 Java 构造器 `SignatureResult(sign, serialNumber)`）。
    pub fn new(sign: impl Into<String>, serial_number: impl Into<String>) -> Self {
        Self {
            sign: sign.into(),
            certificate_serial_number: serial_number.into(),
        }
    }
}

/// v3 请求签名器（对应 Java `v3/auth/Signer` 接口）。
///
/// 默认实现 [`crate::v3::auth::PrivateKeySigner`]（SHA256withRSA）。
pub trait Signer: Send + Sync {
    /// 对消息签名（对应 Java `sign(byte[] message)`）。
    fn sign(&self, message: &[u8]) -> Result<SignatureResult, WxV3CryptoError>;
}
