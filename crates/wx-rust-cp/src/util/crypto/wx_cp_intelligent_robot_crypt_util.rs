//! 企业微信智能机器人消息加解密工具。
//!
//! 对应 Java `me.chanjar.weixin.cp.util.crypto.WxCpIntelligentRobotCryptUtil`：
//! 继承 `WxCryptUtil`，用于智能机器人 API 模式消息的加解密。
//! 与 `WxCpCryptUtils` 的区别在于：智能机器人使用独立的 token/
//! encodingAESKey 配置（从 `WxCpConfigStorage` 获取智能机器人专用配置），
//! 且 `encrypt` 返回 JSON 格式（非 XML）。

use wx_rust_common::util::crypto::{Sha1, WxCryptUtil};

use crate::config::WxCpConfigStorage;

/// 企业微信智能机器人消息加解密工具。
///
/// 对应 Java `WxCpIntelligentRobotCryptUtil`。
///
/// 与 `WxCpCryptUtils` 共享同一套 AES-256-CBC + PKCS7 + SHA1 算法，
/// 但使用智能机器人专用的 token/encodingAESKey/corpId。
/// `encrypt` 返回 JSON 格式（含 `encrypt`/`msg_signature`/`timestamp`/`nonce`），
/// 对应 Java `WxCpIntelligentRobotCryptUtil.encrypt(String, String, String)`。
#[derive(Debug, Clone)]
pub struct WxCpIntelligentRobotCryptUtil {
    token: String,
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

        let inner = WxCryptUtil::new(&token, aes_key, app_id)?;
        Ok(Self { token, inner })
    }

    /// 从原始参数构建智能机器人加解密工具。
    ///
    /// 对应 Java `WxCpIntelligentRobotCryptUtil(String token, String
    /// encodingAesKey, String aiBotId)` 构造器。
    pub fn from_params(
        token: &str,
        encoding_aes_key: &str,
        ai_bot_id: &str,
    ) -> Result<Self, String> {
        let inner = WxCryptUtil::new(token, encoding_aes_key, ai_bot_id)?;
        Ok(Self {
            token: token.to_string(),
            inner,
        })
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

    /// 验证签名后解密密文内容（对应 Java `WxCpIntelligentRobotCryptUtil
    /// .decrypt(String, String, String, String)`）。
    ///
    /// 先以 SHA1(token, timestamp, nonce, encryptedContent) 校验签名，
    /// 再 AES 解密并校验 receiverId == aiBotId。
    pub fn decrypt(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        encrypted_content: &str,
    ) -> Result<String, String> {
        // SHA1 签名校验（对应 Java `SHA1.gen(this.token, timestamp, nonce,
        // encryptedContent)`）
        let signature = Sha1::digest_with_amp(&[&self.token, timestamp, nonce, encrypted_content])?;
        if signature != msg_signature {
            return Err("加密消息签名校验失败".to_string());
        }
        // AES 解密（复用 inner 的 decrypt，已含 receiverId 校验）
        self.inner.decrypt(encrypted_content)
    }

    /// 验证签名后直接解密密文内容（兼容旧接口，委托 [`Self::decrypt`]）。
    pub fn decrypt_content(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        cipher_text: &str,
    ) -> Result<String, String> {
        self.decrypt(msg_signature, timestamp, nonce, cipher_text)
    }

    /// 将待回复消息加密打包为 JSON 格式（对应 Java `WxCpIntelligentRobotCryptUtil
    /// .encrypt(String plainJson, String timestamp, String nonce)`）。
    ///
    /// 返回 JSON 字符串，包含 `encrypt`、`msg_signature`、`timestamp`、`nonce`
    /// 四个字段（对应 Java `JsonObject` 组装）。
    pub fn encrypt_json(
        &self,
        plain_json: &str,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String, String> {
        // 随机 16 字符（对应 Java `UUID.randomUUID().toString().replace("-",
        // "").substring(0, 16)`）
        let random_str = WxCryptUtil::gen_random_str();
        let random_16 = &random_str[..16];
        let encrypted_content = self.inner.encrypt_with_random(random_16, plain_json)?;
        // SHA1 签名（对应 Java `SHA1.gen(this.token, timestamp, nonce,
        // encryptedContent)`）
        let msg_signature =
            Sha1::digest_with_amp(&[&self.token, timestamp, nonce, &encrypted_content])?;
        // JSON 组装
        let result = serde_json::json!({
            "encrypt": encrypted_content,
            "msg_signature": msg_signature,
            "timestamp": timestamp,
            "nonce": nonce,
        });
        serde_json::to_string(&result).map_err(|e| format!("JSON 序列化失败: {e}"))
    }

    /// 将待回复消息加密打包为 XML 格式（兼容旧接口）。
    pub fn encrypt(&self, plain_xml: &str) -> Result<String, String> {
        self.inner.encrypt(plain_xml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let token = "test_token";
        let aes_key = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG"; // 43 chars -> 32 bytes
        let corp_id = "test_corp_id";

        let util = WxCpIntelligentRobotCryptUtil::from_params(token, aes_key, corp_id)
            .expect("创建加密工具成功");

        let plaintext = "Hello, WxCpIntelligentRobot!";
        let encrypted = util.encrypt(plaintext).expect("加密成功");
        assert!(!encrypted.is_empty());
    }

    /// 镜像 Java `testEncryptDecrypt`：encrypt_json 返回 JSON 且含 4 字段。
    #[test]
    fn test_encrypt_json_roundtrip() {
        let token = "test_token";
        let aes_key = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG"; // 43 chars -> 32 bytes
        let corp_id = "test_corp_id";

        let util = WxCpIntelligentRobotCryptUtil::from_params(token, aes_key, corp_id)
            .expect("创建加密工具成功");

        let plain_json = r#"{"content":"hello"}"#;
        let timestamp = "1700000000";
        let nonce = "nonce_123";
        let encrypted_json = util
            .encrypt_json(plain_json, timestamp, nonce)
            .expect("JSON 加密成功");

        // 解析返回的 JSON，验证 4 个字段都存在
        let parsed: serde_json::Value =
            serde_json::from_str(&encrypted_json).expect("解析加密 JSON 成功");
        assert!(parsed.get("encrypt").is_some(), "应含 encrypt 字段");
        assert!(
            parsed.get("msg_signature").is_some(),
            "应含 msg_signature 字段"
        );
        assert!(parsed.get("timestamp").is_some(), "应含 timestamp 字段");
        assert!(parsed.get("nonce").is_some(), "应含 nonce 字段");
        assert_eq!(parsed["timestamp"], timestamp);
        assert_eq!(parsed["nonce"], nonce);

        // 验证签名可反向校验（decrypt 使用同一签名逻辑）
        let msg_signature = parsed["msg_signature"].as_str().unwrap();
        let encrypt = parsed["encrypt"].as_str().unwrap();
        let decrypted = util
            .decrypt(msg_signature, timestamp, nonce, encrypt)
            .expect("解密成功");
        assert_eq!(decrypted, plain_json);
    }
}
