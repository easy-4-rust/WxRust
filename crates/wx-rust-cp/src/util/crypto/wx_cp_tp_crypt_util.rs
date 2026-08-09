//! 企业微信第三方应用（服务商）消息加解密。
//!
//! 对应 Java `me.chanjar.weixin.cp.util.crypto.WxCpTpCryptUtil`（继承
//! common `WxCryptUtil`）：从 `WxCpTpConfigStorage` 取 token/
//! encodingAESKey/corpId，aesKey 去除全部空格（对齐 Java
//! `StringUtils.remove(encodingAesKey, " ")`）后 Base64 解码。
//!
//! 用途：
//! - `getVerifyDecrypt`（验证 URL 回调的 echoStr，对应 Java
//!   `cryptUtil.decrypt(sVerifyEchoStr)`）；
//! - 服务商推送消息 XML 解密（`fromEncryptedXml`，见
//!   `bean::message::WxCpTpXmlMessage::from_encrypted_xml`）。

use base64::Engine;

use wx_rust_common::util::crypto::WxCryptUtil;

use crate::config::WxCpTpConfigStorage;

/// 企业微信第三方应用消息加解密工具。
#[derive(Debug, Clone)]
pub struct WxCpTpCryptUtil {
    inner: WxCryptUtil,
}

impl WxCpTpCryptUtil {
    /// 从第三方应用配置存储构建加解密工具。
    ///
    /// # 参数
    /// - `config`：第三方应用配置存储（token/encodingAESKey/corpId）
    ///
    /// Java 构造语义：
    /// `this.aesKey = Base64.decode(StringUtils.remove(encodingAESKey, " "))`；
    /// `appidOrCorpid` 为服务商企业 ID（`getCorpId`）。
    pub fn new(config: &dyn WxCpTpConfigStorage) -> Result<Self, String> {
        // Java: StringUtils.remove(encodingAesKey, " ")——去除全部空格后
        // base64 解码（WxCryptUtil::new 内部完成解码）
        let aes_key = config
            .encoding_aes_key()
            .unwrap_or_default()
            .replace(' ', "");
        let inner = WxCryptUtil::new(
            config.token().unwrap_or_default(),
            aes_key,
            config.corp_id(),
        )?;
        Ok(Self { inner })
    }

    /// 解密 echoStr/密文（对应 Java `WxCryptUtil.decrypt(String)`：
    /// Base64 解码 → AES-CBC 解密 → PKCS7 去补位 → 拆分随机串/明文/appid
    /// 并校验 appid）。
    pub fn decrypt(&self, cipher_text: &str) -> Result<String, String> {
        self.inner.decrypt(cipher_text)
    }

    /// 验证签名后解密企业微信推送的加密消息（xml 格式，对应 Java
    /// `WxCryptUtil.decryptXml`）。
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

    /// 验证签名后直接解密密文内容（对应 Java `WxCryptUtil.decryptContent`）。
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

    /// 将待回复消息加密打包为带签名的 xml（对应 Java `WxCryptUtil.encrypt`）。
    pub fn encrypt(&self, plain_xml: &str) -> Result<String, String> {
        self.inner.encrypt(plain_xml)
    }

    /// Base64 解码辅助（保留给调用方按需使用；Java 构造时以
    /// `Base64.getDecoder()` 解码 aesKey，由 `WxCryptUtil::new` 内部承载）。
    pub fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
        base64::engine::general_purpose::STANDARD
            .decode(input)
            .map_err(|e| format!("Base64 解码失败: {e}"))
    }
}
