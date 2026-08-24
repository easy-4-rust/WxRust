//! 敏感信息 RSA 加解密（v3/util Java 命名镜像）。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.util.RsaCryptoUtil`：
//! - `encryptOAEP(message, certificate)`：`RSA/ECB/OAEPWithSHA-1AndMGF1Padding`
//!   加密（平台证书公钥），Base64 输出，明文上限 214 字节（RSA-OAEP-2048）；
//! - `decryptOAEP(ciphertext, privateKey)`：对应解密；
//! - `encryptFields(object, certificate)`：反射扫描 `@SpecEncrypt` 字段并
//!   逐字段加密——Java 注解/反射机制专属，Rust 以调用侧**显式**调用
//!   [`encrypt_oaep`]（或实现 [`crate::v3::SpecEncrypt`] 约定）承载，
//!   见 `v3/spec_encrypt.rs` 的 ADAPTED 说明。
//!
//! 实现复用 [`crate::util::crypto`] 的 `rsa_oaep_encrypt`/`rsa_oaep_decrypt`。

pub use crate::util::crypto::WxV3CryptoError;
use crate::util::crypto::{rsa_oaep_decrypt, rsa_oaep_encrypt};
pub use rsa::{RsaPrivateKey, RsaPublicKey};

/// RSA-OAEP（SHA-1）加密敏感信息（对应 Java `RsaCryptoUtil.encryptOAEP`）。
///
/// # 参数
/// - `public_key`：平台证书/公钥模式的公钥（对应 Java `certificate.getPublicKey()`
///   或 `fullPublicKeyModel` 公钥）
/// - `message`：明文（姓名/身份证/银行卡号等，单块上限 214 字节）
///
/// # 返回
/// Base64 编码密文
pub fn encrypt_oaep(public_key: &RsaPublicKey, message: &str) -> Result<String, WxV3CryptoError> {
    rsa_oaep_encrypt(public_key, message)
}

/// RSA-OAEP（SHA-1）解密（对应 Java `RsaCryptoUtil.decryptOAEP`）。
///
/// # 参数
/// - `private_key`：商户 API 私钥
/// - `ciphertext_b64`：Base64 编码密文
pub fn decrypt_oaep(
    private_key: &RsaPrivateKey,
    ciphertext_b64: &str,
) -> Result<String, WxV3CryptoError> {
    rsa_oaep_decrypt(private_key, ciphertext_b64)
}

#[cfg(test)]
mod tests {
    use rand_core::OsRng;

    use super::*;

    /// 加解密往返（离线，随机密钥对）。
    #[test]
    fn oaep_encrypt_decrypt_roundtrip() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥");
        let public_key = RsaPublicKey::from(&private_key);

        let ciphertext = encrypt_oaep(&public_key, "张三#110101199001011234").unwrap();
        let plaintext = decrypt_oaep(&private_key, &ciphertext).unwrap();
        assert_eq!(plaintext, "张三#110101199001011234");
    }
}
