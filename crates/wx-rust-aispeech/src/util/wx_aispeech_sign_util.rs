//! 签名与加解密工具。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.util.WxAispeechSignUtil`：
//! - `calcDialogSign`：对话 API 请求签名，MD5 链式摘要（token + 时间戳 +
//!   nonce + 请求体 MD5）；
//! - `calcKnowledgeSignature`：知识库 API 请求签名，HmacSHA256（小写十六
//!   进制），载荷为 `timestamp\nnonce\nrequestId\nbody`；
//! - `encrypt/decryptAesCbc`：对话查询报文的 AES-CBC 加解密，密钥为
//!   base64 解码后的字节（IV 取密钥前 16 字节），PKCS7（Java PKCS5 别名）
//!   填充。

use std::fmt::Write as _;

use base64::Engine as _;
use hmac::{KeyInit, Mac};

/// 签名与加解密工具（对应 Java `WxAispeechSignUtil`，静态方法）。
pub struct WxAispeechSignUtil;

impl WxAispeechSignUtil {
    /// 计算对话 API 请求签名（对应 Java `calcDialogSign`）。
    ///
    /// `md5Hex(token + timestamp + nonce + md5Hex(body))`，token/body 为
    /// null 时按空串处理（对应 Java `defaultString`）。
    pub fn calc_dialog_sign(
        token: Option<&str>,
        timestamp: i64,
        nonce: &str,
        body: &str,
    ) -> String {
        let body_md5 = md5_hex(body);
        let source = format!("{}{timestamp}{nonce}{body_md5}", default_string(token));
        md5_hex(&source)
    }

    /// 计算知识库 API 请求签名（对应 Java `calcKnowledgeSignature`）。
    ///
    /// HmacSHA256(`secretKey`, `timestamp\nnonce\nrequestId\nbody`) 的小写
    /// 十六进制摘要。
    pub fn calc_knowledge_signature(
        secret_key: Option<&str>,
        timestamp: i64,
        nonce: &str,
        request_id: &str,
        request_body: &str,
    ) -> String {
        let payload = format!("{timestamp}\n{nonce}\n{request_id}\n{request_body}");
        let mut mac =
            hmac::Hmac::<sha2::Sha256>::new_from_slice(default_string(secret_key).as_bytes())
                .expect("HMAC 初始化不应失败");
        mac.update(payload.as_bytes());
        let bytes = mac.finalize().into_bytes();
        bytes_to_hex(&bytes)
    }

    /// AES-CBC 加密并输出 base64（对应 Java `encryptAesCbcToBase64`）。
    ///
    /// 密钥为 base64 解码字节（Java `decodeAesKey` 对输入补一个 `=` 后
    /// 解码），IV 取密钥前 16 字节，PKCS7 填充（与 Java PKCS5Padding 等价）。
    pub fn encrypt_aes_cbc_to_base64(plain_text: &str, aes_key: &str) -> Result<String, String> {
        let key = decode_aes_key(aes_key)?;
        let iv = &key[..16.min(key.len())];
        let encrypted = aes_cbc_encrypt(&key, iv, plain_text.as_bytes())?;
        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted))
    }

    /// base64 密文解密（对应 Java `decryptAesCbcFromBase64`）。
    pub fn decrypt_aes_cbc_from_base64(
        cipher_text_base64: &str,
        aes_key: &str,
    ) -> Result<String, String> {
        let key = decode_aes_key(aes_key)?;
        let iv = &key[..16.min(key.len())];
        let encrypted = base64::engine::general_purpose::STANDARD
            .decode(cipher_text_base64)
            .map_err(|e| format!("base64 解码失败: {e}"))?;
        let plain = aes_cbc_decrypt(&key, iv, &encrypted)?;
        String::from_utf8(plain).map_err(|e| format!("解密结果非法 UTF-8: {e}"))
    }
}

/// 标准 base64 字母表（与 base64 crate STANDARD 一致）。
const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 标准 base64 字符 → 6 位索引。
fn b64_index(c: u8) -> Option<u8> {
    B64_CHARS.iter().position(|&b| b == c).map(|i| i as u8)
}

/// 解码 AES 密钥（对应 Java `decodeAesKey`：输入补 `=` 后 base64 解码）。
///
/// Java commons-codec 为宽松解码（容忍无填充/尾部不完整量子，多余位丢弃）；
/// Rust 以「补 `=` → 去尾部填充 → 宽松标准解码」表达同一语义
/// （如 43 位密钥解码为 32 字节 → AES-256）。
fn decode_aes_key(aes_key: &str) -> Result<Vec<u8>, String> {
    let padded = format!("{aes_key}=");
    let trimmed = padded.trim_end_matches('=');
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    // 对应 Java commons-codec lenient 解码：尾部不完整量子的多余位直接丢弃
    // （base64 严格解码要求尾部位为 0，此处将最后一个符号的丢弃位清零）
    let mut s = trimmed.to_string();
    match s.len() % 4 {
        // 单个多余字符（6 位凑不满一个字节）直接丢弃
        1 => {
            s.pop();
        }
        // 两字符量子：第二字符低 4 位被丢弃
        2 => {
            if let Some(c) = s.chars().last()
                && let Some(idx) = b64_index(c as u8)
            {
                s.pop();
                s.push(B64_CHARS[(idx & !0b1111) as usize] as char);
            }
        }
        // 三字符量子：第三字符低 2 位被丢弃
        3 => {
            if let Some(c) = s.chars().last()
                && let Some(idx) = b64_index(c as u8)
            {
                s.pop();
                s.push(B64_CHARS[(idx & !0b11) as usize] as char);
            }
        }
        _ => {}
    }
    base64::engine::general_purpose::STANDARD_PAD_INDIFFERENT
        .decode(s)
        .map_err(|e| format!("AES 密钥 base64 解码失败: {e}"))
}

/// AES-CBC 加密（按密钥长度选择 AES-128/192/256，对应 Java SecretKeySpec）。
///
/// 注：cbc 0.2.1 + cipher 0.5 组合下 `new_from_slices` 产生错误密文
/// （经 openssl 独立向量验证），故按 `wx-rust-common::util::crypto` 的
/// 既有模式使用 `Key`/`Iv` + `KeyIvInit::new`（与 mp/miniapp 加解密同一
/// 验证过的用法）。
fn aes_cbc_encrypt(key: &[u8], iv: &[u8], plain: &[u8]) -> Result<Vec<u8>, String> {
    use cbc::cipher::block_padding::Pkcs7;
    use cbc::cipher::{BlockModeEncrypt, KeyIvInit};
    // PKCS7 填充后最多多出一个分块（16 字节）
    let mut buf = vec![0u8; plain.len() + 16];
    match key.len() {
        32 => {
            let mut k = cbc::cipher::Key::<cbc::Encryptor<aes::Aes256>>::default();
            k.clone_from_slice(key);
            let mut i = cbc::cipher::Iv::<cbc::Encryptor<aes::Aes256>>::default();
            i.clone_from_slice(iv);
            cbc::Encryptor::<aes::Aes256>::new(&k, &i)
                .encrypt_padded_b2b::<Pkcs7>(plain, &mut buf)
                .map(|ct| ct.to_vec())
                .map_err(|e| format!("AES 加密失败: {e}"))
        }
        24 => {
            let mut k = cbc::cipher::Key::<cbc::Encryptor<aes::Aes192>>::default();
            k.clone_from_slice(key);
            let mut i = cbc::cipher::Iv::<cbc::Encryptor<aes::Aes192>>::default();
            i.clone_from_slice(iv);
            cbc::Encryptor::<aes::Aes192>::new(&k, &i)
                .encrypt_padded_b2b::<Pkcs7>(plain, &mut buf)
                .map(|ct| ct.to_vec())
                .map_err(|e| format!("AES 加密失败: {e}"))
        }
        _ => {
            let mut k = cbc::cipher::Key::<cbc::Encryptor<aes::Aes128>>::default();
            k.clone_from_slice(key);
            let mut i = cbc::cipher::Iv::<cbc::Encryptor<aes::Aes128>>::default();
            i.clone_from_slice(iv);
            cbc::Encryptor::<aes::Aes128>::new(&k, &i)
                .encrypt_padded_b2b::<Pkcs7>(plain, &mut buf)
                .map(|ct| ct.to_vec())
                .map_err(|e| format!("AES 加密失败: {e}"))
        }
    }
}

/// AES-CBC 解密（对应 Java `Cipher.doFinal`，PKCS7 去填充）。
fn aes_cbc_decrypt(key: &[u8], iv: &[u8], cipher_text: &[u8]) -> Result<Vec<u8>, String> {
    use cbc::cipher::block_padding::Pkcs7;
    use cbc::cipher::{BlockModeDecrypt, KeyIvInit};
    let mut buf = cipher_text.to_vec();
    match key.len() {
        32 => {
            let mut k = cbc::cipher::Key::<cbc::Decryptor<aes::Aes256>>::default();
            k.clone_from_slice(key);
            let mut i = cbc::cipher::Iv::<cbc::Decryptor<aes::Aes256>>::default();
            i.clone_from_slice(iv);
            cbc::Decryptor::<aes::Aes256>::new(&k, &i)
                .decrypt_padded::<Pkcs7>(&mut buf)
                .map(|pt| pt.to_vec())
                .map_err(|e| format!("AES 解密失败: {e}"))
        }
        24 => {
            let mut k = cbc::cipher::Key::<cbc::Decryptor<aes::Aes192>>::default();
            k.clone_from_slice(key);
            let mut i = cbc::cipher::Iv::<cbc::Decryptor<aes::Aes192>>::default();
            i.clone_from_slice(iv);
            cbc::Decryptor::<aes::Aes192>::new(&k, &i)
                .decrypt_padded::<Pkcs7>(&mut buf)
                .map(|pt| pt.to_vec())
                .map_err(|e| format!("AES 解密失败: {e}"))
        }
        _ => {
            let mut k = cbc::cipher::Key::<cbc::Decryptor<aes::Aes128>>::default();
            k.clone_from_slice(key);
            let mut i = cbc::cipher::Iv::<cbc::Decryptor<aes::Aes128>>::default();
            i.clone_from_slice(iv);
            cbc::Decryptor::<aes::Aes128>::new(&k, &i)
                .decrypt_padded::<Pkcs7>(&mut buf)
                .map(|pt| pt.to_vec())
                .map_err(|e| format!("AES 解密失败: {e}"))
        }
    }
}

/// MD5 十六进制（小写，对应 Java `DigestUtils.md5Hex`）。
fn md5_hex(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{digest:x}")
}

/// null → 空串（对应 Java `defaultString`）。
fn default_string(value: Option<&str>) -> &str {
    value.unwrap_or("")
}

/// 字节转小写十六进制（对应 Java `bytesToHex`）。
fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}
