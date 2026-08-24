//! AES-256-GCM 加解密（v3/util Java 命名镜像）。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.util.AesUtils`：
//! apiV3Key（32 字节）为密钥，`AEAD_AES_256_GCM`，associatedData 为 AAD，
//! ciphertext 为密文+16 字节 tag 的 Base64。`decryptToString`/
//! `decryptToByte` 用于通知回调 `resource.ciphertext` 解密与
//! `/v3/certificates` 平台证书解密。
//!
//! 实现复用 [`crate::util::crypto`] 的 `aes_gcm_encrypt`/`aes_gcm_decrypt`/
//! `aes_gcm_decrypt_bytes`（官方 SDK 向量已验证）。

pub use crate::util::crypto::{
    WxV3CryptoError, aes_gcm_decrypt, aes_gcm_decrypt_bytes, aes_gcm_encrypt,
};

/// 加密为 Base64 密文（对应 Java `AesUtils.encrypt(associatedData, nonce,
/// certificate, apiV3Key)`）。
///
/// # 参数
/// - `api_v3_key`：APIv3 密钥（32 字节）
/// - `associated_data`：附加数据（AAD）
/// - `nonce`：12 字节随机串（可用 [`crate::util::crypto::gen_gcm_nonce`]）
/// - `plaintext`：明文
///
/// # 返回
/// Base64(密文+tag)
pub fn encrypt(
    api_v3_key: &str,
    associated_data: &str,
    nonce: &[u8],
    plaintext: &str,
) -> Result<String, WxV3CryptoError> {
    aes_gcm_encrypt(api_v3_key, associated_data, nonce, plaintext)
}

/// 解密为字符串（对应 Java `AesUtils.decryptToString(associatedData, nonce,
/// ciphertext, apiV3Key)`）。
pub fn decrypt_to_string(
    api_v3_key: &str,
    associated_data: &str,
    nonce: &str,
    ciphertext_b64: &str,
) -> Result<String, WxV3CryptoError> {
    aes_gcm_decrypt(api_v3_key, associated_data, nonce, ciphertext_b64)
}

/// 解密为字节（对应 Java `AesUtils.decryptToByte(associatedData, nonce,
/// cipherData, key)`；`cipherData` 形态为密文+tag 的 Base64，与
/// `decryptToString` 同参，返回 UTF-8 字节）。
pub fn decrypt_to_bytes(
    api_v3_key: &str,
    associated_data: &str,
    nonce: &str,
    ciphertext_b64: &str,
) -> Result<Vec<u8>, WxV3CryptoError> {
    aes_gcm_decrypt(api_v3_key, associated_data, nonce, ciphertext_b64).map(String::into_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 加解密往返（离线，apiV3Key/nonce 取 official SDK TestConfig/官方示例值）。
    #[test]
    fn encrypt_decrypt_roundtrip() {
        let api_v3_key = "a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb";
        let nonce = "61a9c8685a6f"; // 12 字节 ASCII（官方示例 nonce）
        let ciphertext = encrypt(
            api_v3_key,
            "certificate",
            nonce.as_bytes(),
            "{\"serial_no\":1}",
        )
        .unwrap();
        let plaintext = decrypt_to_string(api_v3_key, "certificate", nonce, &ciphertext);
        assert_eq!(plaintext.unwrap(), "{\"serial_no\":1}");
        let bytes = decrypt_to_bytes(api_v3_key, "certificate", nonce, &ciphertext);
        assert_eq!(bytes.unwrap(), b"{\"serial_no\":1}");
    }
}
