//! 公钥模式的"证书"承载对象。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.X509PublicCertificate`：
//! 继承 `X509Certificate` 但只承载一对公钥 + 公钥 ID（`PUB_KEY_ID_` 前缀），
//! `checkValidity` 为空实现（公钥无有效期概念），
//! `getSerialNumber` 返回 `new BigInteger(publicId.replace("PUB_KEY_ID_", ""), 16)`。
//!
//! 用于微信支付「公钥模式」（新商户号无平台证书，以微信支付公钥验签），
//! 由 [`super::PublicCertificateVerifier`] 的 `getValidCertificate` 返回。

use rsa::RsaPublicKey;

/// 公钥模式的证书承载对象（对应 Java `X509PublicCertificate`）。
#[derive(Debug, Clone)]
pub struct X509PublicCertificate {
    /// 微信支付公钥（对应 Java 字段 `publicKey`）。
    public_key: RsaPublicKey,
    /// 公钥 ID（对应 Java 字段 `publicId`，形如 `PUB_KEY_ID_xxxxxxxx`）。
    public_id: String,
}

/// 公钥 ID 的固定前缀（对应 Java `getSerialNumber` 中移除的
/// `"PUB_KEY_ID_"` 字面量，及 `PublicCertificateVerifier.verify` 的
/// `serialNumber.contains("PUB_KEY_ID")` 判断）。
pub const PUBLIC_KEY_ID_PREFIX: &str = "PUB_KEY_ID_";

impl X509PublicCertificate {
    /// 构造（对应 Java `X509PublicCertificate(PublicKey publicKey, String publicId)`）。
    pub fn new(public_key: RsaPublicKey, public_id: impl Into<String>) -> Self {
        Self {
            public_key,
            public_id: public_id.into(),
        }
    }

    /// 公钥（对应 Java `getPublicKey()`）。
    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    /// 公钥 ID（对应 Java `getPublicId` 语义，原样含 `PUB_KEY_ID_` 前缀）。
    pub fn public_id(&self) -> &str {
        &self.public_id
    }

    /// 序列号（对应 Java `getSerialNumber()`：
    /// `new BigInteger(publicId.replace("PUB_KEY_ID_", ""), 16)`）。
    ///
    /// ADAPTED：Java 返回 `BigInteger`（十六进制解析）；Rust 返回去除前缀
    /// 后的十六进制字符串，与 [`crate::util::crypto::WxPayCertificate::
    /// serial_no`]（证书形态的十六进制大写串）在同一命名空间比较。
    pub fn serial_number_id(&self) -> &str {
        self.public_id
            .strip_prefix(PUBLIC_KEY_ID_PREFIX)
            .unwrap_or(&self.public_id)
    }

    /// 有效期检查（对应 Java `checkValidity` 空实现：公钥无有效期，恒成功）。
    pub fn check_validity(&self) -> Result<(), std::convert::Infallible> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serial_number_strips_pub_key_id_prefix() {
        use rand_core::OsRng;
        let mut rng = OsRng;
        let private_key = rsa::RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥");
        let public_key = rsa::RsaPublicKey::from(&private_key);

        let cert = X509PublicCertificate::new(public_key, "PUB_KEY_ID_1A2B3C");
        assert_eq!(cert.serial_number_id(), "1A2B3C");
        assert_eq!(cert.public_id(), "PUB_KEY_ID_1A2B3C");
        assert!(cert.check_validity().is_ok());
    }
}
