//! 视频号小店消息加解密。
//!
//! 对应 Java `me.chanjar.weixin.channel.util.WxChCryptUtils`（继承
//! `me.chanjar.weixin.common.util.crypto.WxCryptUtil`）：
//!
//! - 消息收发加解密（AES-256-CBC + PKCS7 + Base64 + SHA1 签名，签名用
//!   排序后 `&` 连接即 `Sha1::digest_with_amp`）：包装 common `WxCryptUtil`，
//!   从 `WxChannelConfig` 取 token/aesKey/appid；aesKey 去除首尾空白后补
//!   `=`（对齐 Java `Base64.decodeBase64(StringUtils.trim(config.getAesKey()) + "=")`，
//!   43 字符微信 EncodingAESKey 的宽松解码由 common `WxCryptUtil::new` 承载）；
//! - 用户会话数据解密（[`WxChCryptUtils::decrypt_data`]，对应 Java 静态
//!   `WxChCryptUtils.decrypt(sessionKey, encryptedData, ivStr)`）：
//!   `AES/CBC/NoPadding`（密钥 = Base64 解码的 session_key，16 字节 →
//!   AES-128）+ `PKCS7Encoder.decode` 去填充，与 Java 逐字节同构。

use base64::Engine;
use cbc::cipher::block_padding::NoPadding;
use cbc::cipher::{BlockModeDecrypt, KeyIvInit};

use wx_rust_common::util::crypto::{EncryptContext, Pkcs7Encoder, WxCryptUtil};

use crate::config::WxChannelConfig;

/// 视频号小店消息加解密工具（对应 Java `WxChCryptUtils`）。
#[derive(Debug, Clone)]
pub struct WxChCryptUtils {
    inner: WxCryptUtil,
}

impl WxChCryptUtils {
    /// 从配置存储构建加解密工具。
    ///
    /// # 参数
    /// - `config`：视频号小店配置（token/aesKey/appid）
    pub fn new(config: &dyn WxChannelConfig) -> Result<Self, String> {
        // Java 构造：Base64.decodeBase64(StringUtils.trim(config.getAesKey()) + "=")
        // ——配置存 43 字符 aesKey，trim 后补 "=" 成合法 base64（44 字符 → 32 字节）
        let aes_key = config.aes_key().unwrap_or_default().trim().to_string() + "=";
        let inner = WxCryptUtil::new(config.token().unwrap_or_default(), aes_key, config.app_id())?;
        Ok(Self { inner })
    }

    /// 解密视频号小店推送的加密消息（xml 格式）。
    ///
    /// 先提取 xml 中的 `Encrypt` 密文，验证签名后解密（对应 Java
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

    /// 对密文进行解密（不校验签名），返回明文。
    ///
    /// 对应 Java `WxCryptUtil.decrypt(String cipherText)`（含随机串前缀拆分
    /// 与 appid 校验）。
    pub fn decrypt(&self, cipher_text: &str) -> Result<String, String> {
        self.inner.decrypt(cipher_text)
    }

    /// 将待回复消息加密打包为带签名的 xml（对应 Java `WxCryptUtil.encrypt`）。
    pub fn encrypt(&self, plain_xml: &str) -> Result<String, String> {
        self.inner.encrypt(plain_xml)
    }

    /// 将待回复消息加密打包，返回加密所需值对象（对应 Java
    /// `WxCryptUtil.encryptContext`）。
    pub fn encrypt_context(&self, plain_xml: &str) -> Result<EncryptContext, String> {
        self.inner.encrypt_context(plain_xml)
    }

    /// 用户会话数据解密（对应 Java 静态
    /// `WxChCryptUtils.decrypt(String sessionKey, String encryptedData, String ivStr)`）。
    ///
    /// 算法：`AES/CBC/NoPadding`（session_key/iv/密文均 Base64 解码，
    /// session_key 解码后 16 字节 → AES-128）解密后按
    /// `PKCS7Encoder.decode` 去除填充，结果按 UTF-8 输出（Java
    /// `new String(..., UTF_8)` 对非法序列以替换符代替，Rust 用 lossy 对齐）。
    pub fn decrypt_data(
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<String, String> {
        let key = base64::engine::general_purpose::STANDARD
            .decode(session_key)
            .map_err(|e| format!("session_key Base64 解码失败: {e}"))?;
        let iv = base64::engine::general_purpose::STANDARD
            .decode(iv_str)
            .map_err(|e| format!("iv Base64 解码失败: {e}"))?;
        let cipher_text = base64::engine::general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|e| format!("encrypted_data Base64 解码失败: {e}"))?;

        // Java: Cipher.getInstance("AES/CBC/NoPadding")，密钥为 sessionKey 解码字节
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
        let cipher = Aes128CbcDec::new_from_slices(&key, &iv)
            .map_err(|e| format!("AES 密钥/IV 初始化失败: {e}"))?;
        let mut buf = vec![0u8; cipher_text.len()];
        let plain_all = cipher
            .decrypt_padded_b2b::<NoPadding>(&cipher_text, &mut buf)
            .map_err(|e| format!("AES 解密失败: {e}"))?;
        // Java: PKCS7Encoder.decode(cipher.doFinal(...))——去除 PKCS7 填充
        let plain = Pkcs7Encoder::decode(plain_all);
        Ok(String::from_utf8_lossy(&plain).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cbc::cipher::block_padding::Pkcs7;
    use cbc::cipher::{BlockModeEncrypt, KeyIvInit};

    use crate::config::r#impl::WxChannelDefaultConfig;

    /// 官方向量（微信官方文档 getUserInfo 解密示例；Java 同款夹具见
    /// weixin-java-miniapp `WxMaCryptUtilsTest.testDecryptAnotherWay`）。
    const OFFICIAL_SESSION_KEY: &str = "tiihtNczf5v6AKRyjwEUhQ==";
    const OFFICIAL_IV: &str = "r7BXXKkLb8qrSNn05n0qiA==";
    const OFFICIAL_ENCRYPTED: &str = "CiyLU1Aw2KjvrjMdj8YKliAjtP4gsMZMQmRzooG2xrDcvSnxIMXFufNstNGTyaGS9uT5geRa0W4oTOb1WT7fJlAC+oNPdbB+3hVbJSRgv+4lGOETKUQz6OYStslQ142dNCuabNPGBzlooOmB231qMM85d2/fV6ChevvXvQP8Hkue1poOFtnEtpyxVLW1zAo6/1Xx1COxFvrc2d7UL/lmHInNlxuacJXwu0fjpXfz/YqYzBIBzD6WUfTIF9GRHpOn/Hz7saL8xz+W//FRAUid1OksQaQx4CMs8LOddcQhULW4ucetDf96JcR3g0gfRK4PC7E/r7Z6xNrXd2UIeorGj5Ef7b1pJAYB6Y5anaHqZ9J6nKEBvB4DnNLIVWSgARns/8wR2SiRS7MNACwTyrGvt9ts8p12PKFdlqYTopNHR1Vf7XjfhQlVsAJdNiKdYmYVoKlaRv85IfVunYzO0IKXsyl7JCUjCpoG20f0a04COwfneQAGGwd5oa+T8yO5hzuyDb/XcxxmK01EpqOyuxINew==";

    /// 官方向量解密（AES-128-CBC + PKCS7）。
    #[test]
    fn decrypt_data_official_vector() {
        let plain =
            WxChCryptUtils::decrypt_data(OFFICIAL_SESSION_KEY, OFFICIAL_ENCRYPTED, OFFICIAL_IV)
                .expect("官方向量解密成功");
        let v: serde_json::Value =
            serde_json::from_str(&plain).expect("解密结果为合法 JSON（官方 getUserInfo）");
        assert_eq!(v["openId"], "oGZUI0egBJY1zhBYw2KhdUfwVJJE");
        assert_eq!(v["unionId"], "ocMvos6NjeKLIBqg5Mr9QjxrP1FA");
        assert_eq!(v["watermark"]["appid"], "wx4f4bc4dec97d474b");
        assert_eq!(v["watermark"]["timestamp"], 1477314187);
    }

    /// 往返：AES-128-CBC + PKCS7 加密 → `decrypt_data` 解密（含 16 倍数
    /// 明文满块填充边界）。
    #[test]
    fn decrypt_data_round_trip() {
        type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

        let key_bytes = [0x2a; 16];
        let iv_bytes = [0x7b; 16];
        let session_key = base64::engine::general_purpose::STANDARD.encode(key_bytes);
        let iv_str = base64::engine::general_purpose::STANDARD.encode(iv_bytes);

        for plain in [
            r#"{"nickName":"Band","openId":"oGZUI0egBJY1zhBYw2KhdUfwVJJE"}"#,
            // 明文长度为 16 的倍数：PKCS7 追加整块填充（Java 同款边界）
            "0123456789abcdef0123456789abcdef",
            "",
        ] {
            let cipher = Aes128CbcEnc::new_from_slices(&key_bytes, &iv_bytes).unwrap();
            let mut buf = vec![0u8; plain.len() + 32];
            let enc = cipher
                .encrypt_padded_b2b::<Pkcs7>(plain.as_bytes(), &mut buf)
                .expect("加密成功");
            let encrypted_data = base64::engine::general_purpose::STANDARD.encode(enc);
            let decrypted = WxChCryptUtils::decrypt_data(&session_key, &encrypted_data, &iv_str)
                .expect("解密成功");
            assert_eq!(decrypted, plain);
        }
    }

    /// 消息加解密往返：`encrypt_context` 组装完整 xml → `decrypt_xml`；
    /// `encrypt` 打包 → `decrypt` 直解（对应 Java `WxCryptUtil` 同语义）。
    #[test]
    fn message_crypto_round_trip() {
        // 微信 EncodingAESKey：43 字符 base64（构造器 trim 后补 "=" → 32 字节）
        let mut config = WxChannelDefaultConfig::new("wxappid123", "secret");
        config
            .set_token("test-token")
            .set_aes_key("kvuO9BLIAs5iFlXRfwOJXjh3z7O1psxaY6jY1pnFUBQ");
        let crypt = WxChCryptUtils::new(&config).expect("构建加解密工具");

        let plain = "<xml><ToUserName><![CDATA[gh_*]]></ToUserName><Content><![CDATA[你好微信]]></Content></xml>";
        let ctx = crypt.encrypt_context(plain).expect("加密上下文");
        let full_xml = format!(
            "<xml>\n<Encrypt><![CDATA[{}]]></Encrypt>\n<MsgSignature><![CDATA[{}]]></MsgSignature>\n<TimeStamp>{}</TimeStamp>\n<Nonce><![CDATA[{}]]></Nonce>\n</xml>",
            ctx.encrypted_xml, ctx.signature, ctx.timestamp, ctx.nonce
        );
        let decrypted = crypt
            .decrypt_xml(&ctx.signature, &ctx.timestamp, &ctx.nonce, &full_xml)
            .expect("解密成功");
        assert_eq!(decrypted, plain);

        // encrypt 打包 → 1 参 decrypt 直解（Java WxCryptUtil.decrypt(String)）
        let xml = crypt.encrypt(plain).expect("加密");
        assert!(xml.contains("<Encrypt>"));
        assert!(xml.contains("<MsgSignature>"));
        let via_ctx = crypt.encrypt_context(plain).expect("加密上下文 2");
        let direct = crypt.decrypt(&via_ctx.encrypted_xml).expect("直解成功");
        assert_eq!(direct, plain);

        // 错误签名必须拒绝（Java 抛 WxErrorException 同语义）
        let wrong = crypt.decrypt_xml("deadbeef", &ctx.timestamp, &ctx.nonce, &full_xml);
        assert!(wrong.is_err());
    }
}
