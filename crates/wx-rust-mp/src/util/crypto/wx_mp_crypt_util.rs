//! 公众号消息加解密。
//!
//! 对应 Java `WxMpCryptUtil`：继承 common `WxCryptUtil`，从 `WxMpConfigStorage`
//! 取 token/aesKey/appid，并去掉 aesKey 中的空格（对齐 Java
//! `StringUtils.remove(encodingAesKey, " ")`）。

use wx_rust_common::util::crypto::WxCryptUtil;

use crate::config::WxMpConfigStorage;

/// 公众号消息加解密工具。
#[derive(Debug, Clone)]
pub struct WxMpCryptUtil {
    inner: WxCryptUtil,
}

impl WxMpCryptUtil {
    /// 从配置存储构建加解密工具。
    ///
    /// # 参数
    /// - `config`：公众号配置存储（token/aesKey/appid）
    pub fn new(config: &dyn WxMpConfigStorage) -> Result<Self, String> {
        // Java: StringUtils.remove(encodingAesKey, " ")——去除全部空格后 base64 解码
        let aes_key = config.aes_key().unwrap_or_default().replace(' ', "");
        let inner = WxCryptUtil::new(config.token().unwrap_or_default(), aes_key, config.app_id())?;
        Ok(Self { inner })
    }

    /// 解密公众号推送的加密消息（xml 格式）。
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

    /// 验证签名后直接解密密文内容（对应 Java `decryptContent`）。
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

    /// 将待回复消息加密打包为带签名的 xml（对应 Java `encrypt`）。
    pub fn encrypt(&self, plain_xml: &str) -> Result<String, String> {
        self.inner.encrypt(plain_xml)
    }

    /// 将待回复消息加密打包，返回加密所需值对象（对应 Java `encryptContext`）。
    pub fn encrypt_context(
        &self,
        plain_xml: &str,
    ) -> Result<wx_rust_common::util::crypto::EncryptContext, String> {
        self.inner.encrypt_context(plain_xml)
    }
}
