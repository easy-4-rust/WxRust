//! 开放平台（第三方平台）回调消息加解密工具。
//!
//! 对应 Java `me.chanjar.weixin.open.util.WxOpenCryptUtil`（继承
//! `me.chanjar.weixin.common.util.crypto.WxCryptUtil`）：
//!
//! - 回调推送消息解密（`decrypt_xml`/`decrypt_content`）：SHA1 验签 +
//!   AES-256-CBC 解密（同企业微信 AES-CBC 模式）；
//! - 回复消息加密（`encrypt`/`encrypt_context`）。
//!
//! 与 miniapp 的 `WxMaCryptUtils` 同一包装模式：从 `WxOpenConfigStorage`
//! 取 componentToken/componentAesKey/componentAppId，aesKey 去掉全部空格
//! （对齐 Java `StringUtils.remove(encodingAesKey, " ")`）。

use wx_rust_common::util::crypto::{EncryptContext, WxCryptUtil};

use crate::config::WxOpenConfigStorage;

/// 开放平台（第三方平台）回调消息加解密工具。
#[derive(Debug, Clone)]
pub struct WxOpenCryptUtils {
    inner: WxCryptUtil,
}

impl WxOpenCryptUtils {
    /// 从配置存储构建加解密工具。
    ///
    /// # 参数
    /// - `config`：开放平台配置存储（componentToken/componentAesKey/componentAppId）
    pub fn new(config: &dyn WxOpenConfigStorage) -> Result<Self, String> {
        // Java: StringUtils.remove(encodingAesKey, " ")——去除全部空格后 base64 解码
        let aes_key = config
            .component_aes_key()
            .unwrap_or_default()
            .replace(' ', "");
        let inner = WxCryptUtil::new(
            config.component_token().unwrap_or_default(),
            aes_key,
            config.component_app_id().unwrap_or_default(),
        )?;
        Ok(Self { inner })
    }

    /// 解密第三方平台推送的加密消息（xml 格式）。
    ///
    /// 对应 Java `WxCryptUtil.decryptXml(msgSignature, timestamp, nonce, encryptedXml)`。
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
    pub fn encrypt_context(&self, plain_xml: &str) -> Result<EncryptContext, String> {
        self.inner.encrypt_context(plain_xml)
    }
}
