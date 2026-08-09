//! 企业微信消息加解密。
//!
//! 对应 Java `me.chanjar.weixin.cp.util.crypto.WxCpCryptUtil`（继承
//! `me.chanjar.weixin.common.util.crypto.WxCryptUtil`）：
//!
//! - 消息收发加解密（AES-256-CBC + PKCS7 + Base64 + SHA1 签名，签名用
//!   排序后 `&` 连接即 `Sha1::digest_with_amp`）：包装 common `WxCryptUtil`，
//!   从 `WxCpConfigStorage` 取 token/aesKey/corpId，aesKey 去掉全部空格
//!   （对齐 Java `StringUtils.remove(encodingAesKey, " ")`）；
//! - 会话存档私钥解密（`decryptPriKey`/`decryptPriKeyByPKCS8`/
//!   `decryptPriKeyByPKCS1`，RSA PKCS1 v1.5 填充）与消息整体解密
//!   （`decryptChatData`：RSA 解出 AES 密钥 → AES-256-CBC 解出消息明文，
//!   对应 Java `WxCpMsgAuditServiceImpl.decryptChatData`，其中
//!   `Finance.DecryptData` 官方 native SDK 部分以 Rust 纯实现替代）。
//!
//! 说明：Java 的 `WxCryptUtil` 构造参数为 `(token, encodingAesKey,
//! appidOrCorpid)`，企业微信侧 appidOrCorpid 即 corpid（`getCorpId`，
//! Rust `WxConfigStorage::app_id()`）。

use base64::Engine;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;

use wx_rust_common::util::crypto::{EncryptContext, WxCryptUtil};

use crate::config::WxCpConfigStorage;

/// 企业微信消息加解密工具。
#[derive(Debug, Clone)]
pub struct WxCpCryptUtils {
    inner: WxCryptUtil,
}

impl WxCpCryptUtils {
    /// 从配置存储构建加解密工具。
    ///
    /// # 参数
    /// - `config`：企业微信配置存储（token/aesKey/corpId）
    pub fn new(config: &dyn WxCpConfigStorage) -> Result<Self, String> {
        // Java: StringUtils.remove(encodingAesKey, " ")——去除全部空格后 base64 解码
        let aes_key = config.aes_key().unwrap_or_default().replace(' ', "");
        // Java WxCpCryptUtil 构造：this.appidOrCorpid = corpId
        let inner = WxCryptUtil::new(config.token().unwrap_or_default(), aes_key, config.app_id())?;
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

    /// 将待回复消息加密打包，返回加密所需值对象（对应 Java
    /// `WxCryptUtil.encryptContext`）。
    pub fn encrypt_context(&self, plain_xml: &str) -> Result<EncryptContext, String> {
        self.inner.encrypt_context(plain_xml)
    }
}

// ---------------------------------------------------------------------------
// 会话存档私钥解密（RSA，Wave 0 骨架）
// ---------------------------------------------------------------------------

/// 判断使用 PKCS8 或者 PKCS1 进行解密（对应 Java
/// `WxCpCryptUtil.decryptPriKey(String, String, Integer)`）。
///
/// # 参数
/// - `encrypt_random_key`：使用 `PUBLICKEY_VER` 指定版本的公钥进行非对称
///   加密后 base64 加密的内容
/// - `msg_audit_pri_key`：会话存档私钥
/// - `pkcs1`：使用什么方式进行解密，`Some(1)` 代表使用 PKCS1 进行解密，
///   `Some(2)` 代表 PKCS8 进行解密
pub fn decrypt_pri_key(
    encrypt_random_key: &str,
    msg_audit_pri_key: &str,
    pkcs1: Option<i32>,
) -> Result<String, String> {
    // Java：pkcs1 为 null 时抛 `WxErrorException("请配置会话存档解密方式")`
    let Some(pkcs1) = pkcs1 else {
        return Err("请配置会话存档解密方式".to_string());
    };
    if pkcs1 == 1 {
        return decrypt_pri_key_by_pkcs1(encrypt_random_key, msg_audit_pri_key);
    }
    decrypt_pri_key_by_pkcs8(encrypt_random_key, msg_audit_pri_key)
}

/// PKCS8 私钥解密（对应 Java `WxCpCryptUtil.decryptPriKeyByPKCS8`）。
///
/// 私钥去除 `-----BEGIN/END PRIVATE KEY-----` 头尾与全部空白后 Base64
/// 解码，以 PKCS8 规范加载 RSA 私钥，按 `RSA/ECB/PKCS1Padding`（Java
/// `Cipher.getInstance(keyFactory.getAlgorithm())` 的默认填充，即 PKCS1
/// v1.5）解密 `encrypt_random_key`，结果按 UTF-8 输出（Java
/// `new String(utf8, UTF_8)` 对非法序列以替换符代替，Rust 用 lossy
/// 对齐该语义）。
pub fn decrypt_pri_key_by_pkcs8(
    encrypt_random_key: &str,
    msg_audit_pri_key: &str,
) -> Result<String, String> {
    let normalized = normalize_private_key(
        msg_audit_pri_key,
        "-----BEGIN PRIVATE KEY-----",
        "-----END PRIVATE KEY-----",
    );
    let der = base64::engine::general_purpose::STANDARD
        .decode(&normalized)
        .map_err(|e| format!("会话存档私钥 Base64 解码失败: {e}"))?;
    let private_key =
        rsa::RsaPrivateKey::from_pkcs8_der(&der).map_err(|e| format!("PKCS8 私钥解析失败: {e}"))?;
    decrypt_with_private_key(&private_key, encrypt_random_key)
}

/// PKCS1 私钥解密（对应 Java `WxCpCryptUtil.decryptPriKeyByPKCS1`）。
///
/// 私钥去除 `-----BEGIN/END RSA PRIVATE KEY-----` 头尾与全部空白后
/// Base64 解码，按 PKCS1 规范（Java 以 BouncyCastle `RSAPrivateKey`
/// 解析 ASN.1）加载 RSA 私钥，按 `RSA/ECB/PKCS1Padding` 解密。
pub fn decrypt_pri_key_by_pkcs1(
    encrypt_random_key: &str,
    msg_audit_pri_key: &str,
) -> Result<String, String> {
    let normalized = normalize_private_key(
        msg_audit_pri_key,
        "-----BEGIN RSA PRIVATE KEY-----",
        "-----END RSA PRIVATE KEY-----",
    );
    let der = base64::engine::general_purpose::STANDARD
        .decode(&normalized)
        .map_err(|e| format!("会话存档私钥 Base64 解码失败: {e}"))?;
    let private_key =
        rsa::RsaPrivateKey::from_pkcs1_der(&der).map_err(|e| format!("PKCS1 私钥解析失败: {e}"))?;
    decrypt_with_private_key(&private_key, encrypt_random_key)
}

/// 用 RSA 私钥按 PKCS1 v1.5 填充解密 base64 密文，输出 UTF-8 字符串。
fn decrypt_with_private_key(
    private_key: &rsa::RsaPrivateKey,
    encrypt_random_key: &str,
) -> Result<String, String> {
    let cipher_text = base64::engine::general_purpose::STANDARD
        .decode(encrypt_random_key)
        .map_err(|e| format!("encrypt_random_key Base64 解码失败: {e}"))?;
    let plain = private_key
        .decrypt(rsa::Pkcs1v15Encrypt, &cipher_text)
        .map_err(|e| format!("RSA 私钥解密失败: {e}"))?;
    // Java `new String(utf8, StandardCharsets.UTF_8)`（非法序列替换）
    Ok(String::from_utf8_lossy(&plain).into_owned())
}

/// 去除私钥 PEM 的换行（CRLF/CR/LF/LFCR）、头尾标记与全部空格
/// （对应 Java `replaceAll("(\r\n|\r|\n|\n\r)", "")` + 去标记 + 去空格）。
fn normalize_private_key(key: &str, begin: &str, end: &str) -> String {
    // 单个 replace 去除全部 CR/LF（等价于 Java 的换行正则）
    let step1 = key.replace(['\r', '\n'], "");
    let step2 = step1.replace(begin, "");
    let step3 = step2.replace(end, "");
    step3.replace(' ', "")
}

/// 会话存档消息整体解密（对应 Java `WxCpMsgAuditServiceImpl` 的
/// `decryptChatData` 两段式：先 RSA 解密 `encrypt_random_key` 得到 AES
/// 密钥，再解密 `encrypt_chat_msg`）。
///
/// Java 的第二段（`Finance.DecryptData`）由官方 native SDK 实现，此处以
/// Rust 纯实现替代（ADAPTED）：AES-256-CBC + PKCS7，密钥为 RSA 解密结果
/// 的字节，IV 为密钥前 16 字节（社区对官方 SDK 行为的镜像实现）。
///
/// # 参数
/// - `encrypt_random_key`：RSA 加密的随机密钥（base64）
/// - `encrypt_chat_msg`：消息密文（base64）
/// - `msg_audit_pri_key`：会话存档私钥
/// - `pkcs1`：`Some(1)` PKCS1 / 其他 PKCS8
pub fn decrypt_chat_data(
    encrypt_random_key: &str,
    encrypt_chat_msg: &str,
    msg_audit_pri_key: &str,
    pkcs1: Option<i32>,
) -> Result<String, String> {
    let aes_key = decrypt_pri_key(encrypt_random_key, msg_audit_pri_key, pkcs1)?;
    decrypt_encrypt_chat_msg(&aes_key, encrypt_chat_msg)
}

/// 解密消息密文（对应官方 SDK `Finance.DecryptData` 的纯实现替代）。
///
/// `encrypt_key` 为 RSA 解密出的 AES 密钥（UTF-8 字符串，与 Java 传给
/// `DecryptData` 的 `decryptByPriKey` 一致），取其字节作为 AES-256 密钥，
/// IV 为密钥前 16 字节，AES-256-CBC + PKCS7 解出明文。
pub fn decrypt_encrypt_chat_msg(
    encrypt_key: &str,
    encrypt_chat_msg: &str,
) -> Result<String, String> {
    let key_bytes = encrypt_key.as_bytes();
    if key_bytes.len() < 32 {
        return Err(format!(
            "AES 密钥长度不足：期望至少 32 字节，实际 {} 字节",
            key_bytes.len()
        ));
    }
    let cipher_text = base64::engine::general_purpose::STANDARD
        .decode(encrypt_chat_msg)
        .map_err(|e| format!("encrypt_chat_msg Base64 解码失败: {e}"))?;

    use aes::Aes256;
    use cbc::cipher::block_padding::Pkcs7;
    use cbc::cipher::{BlockModeDecrypt, KeyIvInit};
    type Aes256CbcDec = cbc::Decryptor<Aes256>;

    let cipher = Aes256CbcDec::new_from_slices(&key_bytes[..32], &key_bytes[..16])
        .map_err(|e| format!("AES 密钥/IV 初始化失败: {e}"))?;
    let mut buf = vec![0u8; cipher_text.len()];
    let plain = cipher
        .decrypt_padded_b2b::<Pkcs7>(&cipher_text, &mut buf)
        .map_err(|e| format!("AES 解密失败: {e}"))?;
    Ok(String::from_utf8_lossy(plain).into_owned())
}
