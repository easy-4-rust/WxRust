//! 公钥模式验签器。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.PublicCertificateVerifier`：
//! 以微信支付公钥（`fullPublicKeyModel`，公钥 ID `PUB_KEY_ID_` 前缀）验签；
//! 序列号不含 `PUB_KEY_ID` 且设置了其他（证书）验证器时先尝试证书验签，
//! 失败再以公钥验签兜底（公钥转账等场景）。
//!
//! 签名原语复用 [`crate::util::crypto::verify_sha256_rsa`]。

use std::sync::{Arc, RwLock};

use rsa::RsaPublicKey;

use crate::util::crypto::{WxPayCertVerifierError, verify_sha256_rsa};

use super::{Verifier, WxPayValidCertificate, X509PublicCertificate};

/// 公钥模式验签器（对应 Java `PublicCertificateVerifier implements Verifier`）。
pub struct PublicCertificateVerifier {
    /// 微信支付公钥（对应 Java 字段 `publicKey`）。
    public_key: RsaPublicKey,
    /// 兜底证书验证器（对应 Java 字段 `certificateVerifier`，
    /// `setOtherVerifier` 注入）。
    other_verifier: RwLock<Option<Arc<dyn Verifier>>>,
    /// 公钥承载对象（对应 Java 字段 `publicCertificate`）。
    public_certificate: X509PublicCertificate,
}

impl std::fmt::Debug for PublicCertificateVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_other = self
            .other_verifier
            .read()
            .map(|v| v.is_some())
            .unwrap_or(false);
        f.debug_struct("PublicCertificateVerifier")
            .field("public_id", &self.public_certificate.public_id())
            .field("has_other_verifier", &has_other)
            .finish_non_exhaustive()
    }
}

impl PublicCertificateVerifier {
    /// 构造（对应 Java `PublicCertificateVerifier(PublicKey publicKey,
    /// String publicId)`）。
    pub fn new(public_key: RsaPublicKey, public_id: impl Into<String>) -> Self {
        Self {
            public_certificate: X509PublicCertificate::new(public_key.clone(), public_id),
            public_key,
            other_verifier: RwLock::new(None),
        }
    }

    /// 兜底证书验证器（对应 Java `getCertificateVerifier` 读取语义）。
    pub fn other_verifier(&self) -> Option<Arc<dyn Verifier>> {
        self.other_verifier
            .read()
            .expect("公钥验证器 other 读锁")
            .clone()
    }
}

impl Verifier for PublicCertificateVerifier {
    /// 验签（对应 Java `verify(serialNumber, message, signature)`）：
    /// 1. 序列号非空、不含 `PUB_KEY_ID` 且有兜底验证器 → 先证书验签，
    ///    通过即返回 `true`（Java 捕获异常继续公钥验签，Rust `bool` 接口
    ///    验签失败天然返回 `false` 继续）；
    /// 2. 公钥 SHA256withRSA 验签兜底。
    fn verify(&self, serial_number: &str, message: &[u8], signature: &str) -> bool {
        if !serial_number.contains("PUB_KEY_ID") {
            if let Some(other) = self.other_verifier() {
                if other.verify(serial_number, message, signature) {
                    return true;
                }
            }
        }
        verify_sha256_rsa(&self.public_key, message, signature).unwrap_or(false)
    }

    /// 返回公钥承载对象（对应 Java `getValidCertificate` 返回
    /// `publicCertificate`；公钥模式恒可用，不检查有效期）。
    fn get_valid_certificate(&self) -> Result<WxPayValidCertificate, WxPayCertVerifierError> {
        Ok(WxPayValidCertificate::PublicCertificate(
            self.public_certificate.clone(),
        ))
    }

    /// 注入兜底证书验证器（对应 Java 覆写
    /// `setOtherVerifier(Verifier verifier)`；`None` 清除，Java 无此形态）。
    fn set_other_verifier(&self, verifier: Option<Arc<dyn Verifier>>) {
        *self.other_verifier.write().expect("公钥验证器 other 写锁") = verifier;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rand_core::OsRng;
    use rsa::{RsaPrivateKey, RsaPublicKey};

    use crate::util::crypto::sign_sha256_rsa;
    use crate::v3::auth::CertificatesVerifier;

    use super::*;

    fn random_keypair() -> (RsaPrivateKey, RsaPublicKey) {
        let mut rng = OsRng;
        let private = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥");
        let public = RsaPublicKey::from(&private);
        (private, public)
    }

    /// 公钥验签往返 + PUB_KEY_ID 序列号绕过兜底验证器。
    #[test]
    fn verify_with_public_key_roundtrip() {
        let (private, public) = random_keypair();
        let verifier = PublicCertificateVerifier::new(public, "PUB_KEY_ID_DEADBEEF");
        let message = b"1700000000\nnonce\n{\"code\":\"SUCCESS\"}\n";
        let sign = sign_sha256_rsa(&private, message).unwrap();

        assert!(verifier.verify("PUB_KEY_ID_DEADBEEF", message, &sign));
        assert!(!verifier.verify("PUB_KEY_ID_DEADBEEF", b"tampered", &sign));
        // 未注入兜底验证器时，普通序列号也走公钥验签
        assert!(verifier.verify("1A2B3C", message, &sign));

        let valid = verifier.get_valid_certificate().unwrap();
        assert_eq!(valid.serial_no(), "DEADBEEF");
        assert!(valid.check_validity().is_ok());
    }

    /// 兜底证书验证器优先（对应 Java：序列号不含 PUB_KEY_ID 时先证书验签）。
    #[test]
    fn falls_back_to_public_key_when_cert_verifier_rejects() {
        let (platform_private, platform_public) = random_keypair();
        // 空证书存储的证书验证器：任何验签都失败
        let cert_verifier = Arc::new(CertificatesVerifier::new(Vec::new()));

        let verifier = PublicCertificateVerifier::new(platform_public, "PUB_KEY_ID_CAFE");
        verifier.set_other_verifier(Some(cert_verifier));

        let message = b"msg";
        let sign = sign_sha256_rsa(&platform_private, message).unwrap();
        // 证书验证器失败 → 公钥兜底成功
        assert!(verifier.verify("1A2B3C", message, &sign));
    }
}
