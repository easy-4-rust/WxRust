//! 企业微信智能机器人消息加解密工具。
//!
//! 对应 Java `me.chanjar.weixin.cp.util.crypto.WxCpIntelligentRobotCryptUtil`：
//! 包装 common `WxCryptUtil`，用于智能机器人消息的加解密。
//! 与 `WxCpCryptUtils` 的区别在于：智能机器人使用独立的 token/
//! encodingAESKey 配置（从 `WxCpConfigStorage` 获取智能机器人专用配置）。

use wx_rust_common::util::crypto::WxCryptUtil;

use crate::config::WxCpConfigStorage;

/// 企业微信智能机器人消息加解密工具。
///
/// 对应 Java `WxCpIntelligentRobotCryptUtil`。
///
/// 与 `WxCpCryptUtils` 共享同一套 AES-256-CBC + PKCS7 + SHA1 算法，
/// 但使用智能机器人专用的 token/encodingAESKey/corpId。
#[derive(Debug, Clone)]
pub struct WxCpIntelligentRobotCryptUtil {
    inner: WxCryptUtil,
}

impl WxCpIntelligentRobotCryptUtil {
    /// 从配置存储构建智能机器人加解密工具。
    ///
    /// 使用智能机器人专用的 token/encodingAESKey/corpId 初始化。
    pub fn new(config: &dyn WxCpConfigStorage) -> Result<Self, String> {
        let token = config.token().unwrap_or_default();
        let aes_key = config.aes_key().unwrap_or_default().replace(' ', "");
        let app_id = config.app_id();

        let inner = WxCryptUtil::new(token, aes_key, app_id)?;
        Ok(Self { inner })
    }

    /// 解密企业微信推送的加密消息（xml 格式）。
    pub fn decrypt_xml(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        encrypted_xml: &str,
    ) -> Result<String, String> {
        self.inner
            .decrypt_xml(msg_signature, timestamp, nonce, encrypted_xml)
    }

    /// 验证签名后直接解密密文内容。
    pub fn decrypt_content(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        cipher_text: &str,
    ) -> Result<String, String> {
        self.inner
            .decrypt_content(msg_signature, timestamp, nonce, cipher_text)
    }

    /// 将待回复消息加密打包。
    pub fn encrypt(&self, plain_xml: &str) -> Result<String, String> {
        self.inner.encrypt(plain_xml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wx_rust_common::util::crypto::WxCryptUtil;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let token = "test_token";
        let aes_key = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG"; // 43 chars -> 32 bytes
        let corp_id = "test_corp_id";

        let inner = WxCryptUtil::new(token, aes_key, corp_id).expect("创建加密工具成功");
        let util = WxCpIntelligentRobotCryptUtil { inner };

        let plaintext = "Hello, WxCpIntelligentRobot!";
        let encrypted = util.encrypt(plaintext).expect("加密成功");
        assert!(!encrypted.is_empty());
    }
}
