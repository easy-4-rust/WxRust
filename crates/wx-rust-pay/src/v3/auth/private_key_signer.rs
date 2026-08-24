//! 商户 API 私钥签名器。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.PrivateKeySigner`：
//! 持有商户证书序列号与私钥，`sign(message)` 做
//! `Signature.getInstance("SHA256withRSA")` 签名并 Base64 编码，与序列号
//! 一起构成 `SignatureResult`。
//!
//! 实现复用 [`crate::util::crypto::sign_sha256_rsa`]（P3 已以 openssl golden
//! 验证签名原语），本文件只补 Java 命名镜像与序列号绑定语义。

use rsa::RsaPrivateKey;

use crate::util::crypto::{WxV3CryptoError, sign_sha256_rsa};

use super::{SignatureResult, Signer};

/// 商户 API 私钥签名器（对应 Java `PrivateKeySigner implements Signer`）。
#[derive(Debug, Clone)]
pub struct PrivateKeySigner {
    /// 商户证书/公钥序列号（对应 Java 字段 `certificateSerialNumber`）。
    certificate_serial_number: String,
    /// 商户 API 私钥（对应 Java 字段 `privateKey`，PKCS#8 已解析形态）。
    private_key: RsaPrivateKey,
}

impl PrivateKeySigner {
    /// 构造（对应 Java `PrivateKeySigner(String serialNumber, PrivateKey privateKey)`）。
    pub fn new(serial_number: impl Into<String>, private_key: RsaPrivateKey) -> Self {
        Self {
            certificate_serial_number: serial_number.into(),
            private_key,
        }
    }

    /// 商户证书/公钥序列号。
    pub fn certificate_serial_number(&self) -> &str {
        &self.certificate_serial_number
    }
}

impl Signer for PrivateKeySigner {
    /// SHA256withRSA 签名（对应 Java `sign(byte[] message)`：
    /// `Signature.getInstance("SHA256withRSA")` + Base64）。
    fn sign(&self, message: &[u8]) -> Result<SignatureResult, WxV3CryptoError> {
        let sign = sign_sha256_rsa(&self.private_key, message)?;
        Ok(SignatureResult::new(
            sign,
            self.certificate_serial_number.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 签名/验签往返（离线）：随机 2048 位密钥对，签名可被对应公钥验证。
    #[test]
    fn sign_roundtrip_with_random_keypair() {
        use rand_core::OsRng;
        use rsa::RsaPublicKey;

        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥");
        let public_key = RsaPublicKey::from(&private_key);

        let signer = PrivateKeySigner::new("0123456789ABCDEF", private_key);
        let result = signer
            .sign(b"GET\n/v3/certificates\n1700000000\nnonce\n\n")
            .unwrap();
        assert_eq!(result.certificate_serial_number, "0123456789ABCDEF");
        assert!(!result.sign.is_empty());

        crate::util::crypto::verify_sha256_rsa(
            &public_key,
            b"GET\n/v3/certificates\n1700000000\nnonce\n\n",
            &result.sign,
        )
        .unwrap();
    }
}
