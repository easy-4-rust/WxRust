//! PEM 密钥/证书加载（v3/util Java 命名镜像）。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.util.PemUtils`：
//! - `loadPrivateKey(String)`：PKCS#8 PEM → `PrivateKey`；
//! - `loadPublicKey(String)`：X.509 SubjectPublicKeyInfo PEM → `PublicKey`；
//! - `loadCertificate(String)`：X.509 证书 PEM → `X509Certificate`
//!   （含 `checkValidity()` 有效期检查，对应 Java 调用方语义）。
//!
//! 实现复用 [`crate::util::crypto::wx_pay_cert_utils`]（PEM 提取 +
//! rsa 0.9/x509-cert 0.2 解析，错误文案与 Java 对齐），零重复。

pub use crate::util::crypto::WxPayCertError;
use crate::util::crypto::{
    WxPayCertificate, load_certificate_from_pem, load_private_key_from_pem,
    load_public_key_from_pem,
};
pub use rsa::{RsaPrivateKey, RsaPublicKey};

/// 加载 PKCS#8 私钥（对应 Java `PemUtils.loadPrivateKey(String privateKey)`）。
///
/// # 参数
/// `private_key_pem`：`-----BEGIN PRIVATE KEY-----` PEM 文本
pub fn load_private_key(private_key_pem: &str) -> Result<RsaPrivateKey, WxPayCertError> {
    load_private_key_from_pem(private_key_pem.as_bytes())
}

/// 加载 X.509 SubjectPublicKeyInfo 公钥（对应 Java
/// `PemUtils.loadPublicKey(String publicKey)`，公钥模式
/// `fullPublicKeyModel` 使用）。
pub fn load_public_key(public_key_pem: &str) -> Result<RsaPublicKey, WxPayCertError> {
    load_public_key_from_pem(public_key_pem.as_bytes())
}

/// 加载 X.509 证书（对应 Java `PemUtils.loadCertificate(String certificate)`）。
pub fn load_certificate(certificate_pem: &str) -> Result<WxPayCertificate, WxPayCertError> {
    load_certificate_from_pem(certificate_pem.as_bytes())
}
