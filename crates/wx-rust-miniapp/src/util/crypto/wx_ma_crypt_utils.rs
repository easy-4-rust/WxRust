//! 小程序消息加解密。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.util.crypt.WxMaCryptUtils`（继承
//! `me.chanjar.weixin.common.util.crypto.WxCryptUtil`）：
//!
//! - 消息收发加解密（AES-256-CBC + PKCS7 + Base64 + SHA1 签名，签名用
//!   排序后 `&` 连接即 `Sha1::digest_with_amp`）：包装 common `WxCryptUtil`，
//!   从 `WxMaConfig` 取 token/aesKey/appid，aesKey 去掉全部空格（对齐 Java
//!   `StringUtils.remove(encodingAesKey, " ")`）。
//! - 小程序开放数据解密（`session_key`，AES-128-CBC）：对应 Java 静态方法
//!   `decrypt`/`decryptAnotherWay`。
//! - 小程序加密网络通道加解密（encrypt_key/hexIv，AES-128-CBC）：对应 Java
//!   静态方法 `decryptWithEncryptKey`/`encryptWithEncryptKey`。
//!
//! 说明：session_key 解密所需的 AES 原语在依赖清单受限的前提下以纯 Rust
//! （FIPS-197）实现，无第三方依赖、无 unsafe。

use wx_rust_common::util::crypto::{EncryptContext, WxCryptUtil};

use crate::config::WxMaConfig;

/// 小程序消息加解密工具。
#[derive(Debug, Clone)]
pub struct WxMaCryptUtils {
    inner: WxCryptUtil,
}

impl WxMaCryptUtils {
    /// 从配置存储构建加解密工具。
    ///
    /// # 参数
    /// - `config`：小程序配置存储（token/aesKey/appid）
    pub fn new(config: &dyn WxMaConfig) -> Result<Self, String> {
        // Java: StringUtils.remove(encodingAesKey, " ")——去除全部空格后 base64 解码
        let aes_key = config.aes_key().unwrap_or_default().replace(' ', "");
        let inner = WxCryptUtil::new(config.token().unwrap_or_default(), aes_key, config.app_id())?;
        Ok(Self { inner })
    }

    /// 解密小程序推送的加密消息（xml 格式）。
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

// ---------------------------------------------------------------------------
// 小程序开放数据解密（session_key）
// ---------------------------------------------------------------------------

/// AES 解密（对应 Java `WxMaCryptUtils.decrypt(sessionKey, encryptedData, ivStr)`）。
///
/// 算法：`AES/CBC/NoPadding`（密钥 = Base64 解码的 session_key，IV = Base64
/// 解码的 ivStr）解密后按 PKCS7 去除填充。
pub fn decrypt(session_key: &str, encrypted_data: &str, iv_str: &str) -> Result<String, String> {
    let key = base64_decode(session_key)?;
    let iv = base64_decode(iv_str)?;
    let data = base64_decode(encrypted_data)?;
    let decrypted = aes_cbc_decrypt(&key, &iv, &data)?;
    // Java `PKCS7Encoder.decode`：按尾部字节截断；非法填充值时 Java 置 0 不截断，
    // 此处以错误返回（ADAPTED：Rust 无 null 表达，显式报错更安全）
    let unpadded = pkcs7_unpad(&decrypted)?;
    String::from_utf8(unpadded).map_err(|e| format!("解密结果不是合法 UTF-8: {e}"))
}

/// AES 解密（对应 Java `WxMaCryptUtils.decryptAnotherWay`）。
///
/// 与 `decrypt` 的区别：密钥先按 16 字节对齐补零（Java `keyBytes % 16 != 0`
/// 时用零填充到 16 的倍数），填充使用标准 PKCS7。
pub fn decrypt_another_way(
    session_key: &str,
    encrypted_data: &str,
    iv_str: &str,
) -> Result<String, String> {
    let mut key = base64_decode(session_key)?;
    // Java: keyBytes 长度按 16 对齐补零（BouncyCastle PKCS7Padding 路径）
    if key.len() % 16 != 0 {
        let groups = key.len() / 16 + 1;
        let mut padded = vec![0u8; groups * 16];
        padded[..key.len()].copy_from_slice(&key);
        key = padded;
    }
    let iv = base64_decode(iv_str)?;
    let data = base64_decode(encrypted_data)?;
    let decrypted = aes_cbc_decrypt(&key, &iv, &data)?;
    let unpadded = pkcs7_unpad(&decrypted)?;
    String::from_utf8(unpadded).map_err(|e| format!("解密结果不是合法 UTF-8: {e}"))
}

// ---------------------------------------------------------------------------
// 小程序加密网络通道（encrypt_key / hexIv）
// ---------------------------------------------------------------------------

/// 使用用户加密 key 对数据进行 AES-128-CBC 解密
/// （对应 Java `WxMaCryptUtils.decryptWithEncryptKey`）。
///
/// # 参数
/// - `encrypt_key`：用户加密 key（Base64 编码，解码后须为 16 字节）
/// - `hex_iv`：加密 iv（Hex 编码，须为 32 位十六进制字符，解码后 16 字节）
/// - `encrypted_data`：加密数据（Base64 编码）
pub fn decrypt_with_encrypt_key(
    encrypt_key: &str,
    hex_iv: &str,
    encrypted_data: &str,
) -> Result<String, String> {
    let key_bytes = base64_decode(encrypt_key)?;
    if key_bytes.len() != 16 {
        return Err(format!(
            "encryptKey 解码后必须为 16 字节（AES-128），实际为 {} 字节",
            key_bytes.len()
        ));
    }
    let iv_bytes = hex_to_bytes(hex_iv)?;
    if iv_bytes.len() != 16 {
        return Err(format!(
            "hexIv 解码后必须为 16 字节（AES-128-CBC），实际为 {} 字节（需 32 位 Hex 字符串）",
            iv_bytes.len()
        ));
    }
    let data_bytes = base64_decode(encrypted_data)?;
    let decrypted = aes_cbc_decrypt(&key_bytes, &iv_bytes, &data_bytes)?;
    let unpadded = pkcs7_unpad(&decrypted)?;
    String::from_utf8(unpadded).map_err(|e| format!("解密结果不是合法 UTF-8: {e}"))
}

/// 使用用户加密 key 对数据进行 AES-128-CBC 加密
/// （对应 Java `WxMaCryptUtils.encryptWithEncryptKey`）。
///
/// # 参数
/// - `encrypt_key`：用户加密 key（Base64 编码，解码后须为 16 字节）
/// - `hex_iv`：加密 iv（Hex 编码，须为 32 位十六进制字符，解码后 16 字节）
/// - `data`：待加密的明文字符串
pub fn encrypt_with_encrypt_key(
    encrypt_key: &str,
    hex_iv: &str,
    data: &str,
) -> Result<String, String> {
    let key_bytes = base64_decode(encrypt_key)?;
    if key_bytes.len() != 16 {
        return Err(format!(
            "encryptKey 解码后必须为 16 字节（AES-128），实际为 {} 字节",
            key_bytes.len()
        ));
    }
    let iv_bytes = hex_to_bytes(hex_iv)?;
    if iv_bytes.len() != 16 {
        return Err(format!(
            "hexIv 解码后必须为 16 字节（AES-128-CBC），实际为 {} 字节（需 32 位 Hex 字符串）",
            iv_bytes.len()
        ));
    }
    let padded = pkcs7_pad(data.as_bytes());
    let encrypted = aes_cbc_encrypt(&key_bytes, &iv_bytes, &padded)?;
    Ok(base64_encode(&encrypted))
}

// ---------------------------------------------------------------------------
// 纯 Rust AES（FIPS-197）实现：支持 128/192/256 位密钥
// ---------------------------------------------------------------------------

/// AES S 盒（FIPS-197 标准 S-box）。
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// 逆 S 盒（由 S 盒推导，`inv_sbox[sbox[i]] = i`）。
fn inv_sbox() -> [u8; 256] {
    let mut inv = [0u8; 256];
    for (i, &v) in SBOX.iter().enumerate() {
        inv[v as usize] = i as u8;
    }
    inv
}

/// AES 密钥扩展，返回 `4 * (Nr + 1)` 个字（字节串）。
///
/// # 参数
/// - `key`：密钥，长度须为 16/24/32 字节
fn key_expansion(key: &[u8]) -> Result<Vec<u8>, String> {
    if !(key.len() == 16 || key.len() == 24 || key.len() == 32) {
        return Err(format!(
            "AES 密钥长度必须为 16/24/32 字节，实际 {}",
            key.len()
        ));
    }
    let nk = key.len() / 4; // 密钥字数
    let nr = nk + 6; // 轮数：10/12/14
    let total_words = 4 * (nr + 1);
    let mut w = vec![0u8; total_words * 4];
    w[..key.len()].copy_from_slice(key);
    // 轮常量（Rcon）
    const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
    let mut i = nk;
    while i < total_words {
        let mut temp = [
            w[(i - 1) * 4],
            w[(i - 1) * 4 + 1],
            w[(i - 1) * 4 + 2],
            w[(i - 1) * 4 + 3],
        ];
        if i % nk == 0 {
            // RotWord
            temp = [temp[1], temp[2], temp[3], temp[0]];
            // SubWord
            for b in temp.iter_mut() {
                *b = SBOX[*b as usize];
            }
            temp[0] ^= RCON[i / nk - 1];
        } else if nk > 6 && i % nk == 4 {
            // 仅 AES-256：每 8 个字中的第 4 个做 SubWord
            for b in temp.iter_mut() {
                *b = SBOX[*b as usize];
            }
        }
        for j in 0..4 {
            w[i * 4 + j] = w[(i - nk) * 4 + j] ^ temp[j];
        }
        i += 1;
    }
    Ok(w)
}

/// 字节代换（SubBytes）。
fn sub_bytes(state: &mut [u8; 16]) {
    for b in state.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

/// 逆字节代换（InvSubBytes）。
fn inv_sub_bytes(state: &mut [u8; 16], inv: &[u8; 256]) {
    for b in state.iter_mut() {
        *b = inv[*b as usize];
    }
}

/// 行移位（ShiftRows）：第 r 行循环左移 r 位。
fn shift_rows(state: &mut [u8; 16]) {
    for r in 1..4 {
        let row = [state[r], state[r + 4], state[r + 8], state[r + 12]];
        for c in 0..4 {
            state[r + 4 * c] = row[(c + r) % 4];
        }
    }
}

/// 逆行移位（InvShiftRows）：第 r 行循环右移 r 位。
fn inv_shift_rows(state: &mut [u8; 16]) {
    for r in 1..4 {
        let row = [state[r], state[r + 4], state[r + 8], state[r + 12]];
        for c in 0..4 {
            state[r + 4 * c] = row[(c + 4 - r) % 4];
        }
    }
}

/// GF(2^8) 乘法（xtime 算法）。
fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

/// 列混合（MixColumns）。
fn mix_columns(state: &mut [u8; 16]) {
    for c in 0..4 {
        let (a0, a1, a2, a3) = (
            state[4 * c],
            state[4 * c + 1],
            state[4 * c + 2],
            state[4 * c + 3],
        );
        state[4 * c] = gmul(a0, 2) ^ gmul(a1, 3) ^ a2 ^ a3;
        state[4 * c + 1] = a0 ^ gmul(a1, 2) ^ gmul(a2, 3) ^ a3;
        state[4 * c + 2] = a0 ^ a1 ^ gmul(a2, 2) ^ gmul(a3, 3);
        state[4 * c + 3] = gmul(a0, 3) ^ a1 ^ a2 ^ gmul(a3, 2);
    }
}

/// 逆列混合（InvMixColumns）。
fn inv_mix_columns(state: &mut [u8; 16]) {
    for c in 0..4 {
        let (a0, a1, a2, a3) = (
            state[4 * c],
            state[4 * c + 1],
            state[4 * c + 2],
            state[4 * c + 3],
        );
        state[4 * c] = gmul(a0, 14) ^ gmul(a1, 11) ^ gmul(a2, 13) ^ gmul(a3, 9);
        state[4 * c + 1] = gmul(a0, 9) ^ gmul(a1, 14) ^ gmul(a2, 11) ^ gmul(a3, 13);
        state[4 * c + 2] = gmul(a0, 13) ^ gmul(a1, 9) ^ gmul(a2, 14) ^ gmul(a3, 11);
        state[4 * c + 3] = gmul(a0, 11) ^ gmul(a1, 13) ^ gmul(a2, 9) ^ gmul(a3, 14);
    }
}

/// 轮密钥加（AddRoundKey）。
fn add_round_key(state: &mut [u8; 16], round_key: &[u8]) {
    for i in 0..16 {
        state[i] ^= round_key[i];
    }
}

/// 加密一个分组。
fn encrypt_block(round_keys: &[u8], nr: usize, block: &mut [u8; 16]) {
    add_round_key(block, &round_keys[0..16]);
    for round in 1..nr {
        sub_bytes(block);
        shift_rows(block);
        mix_columns(block);
        add_round_key(block, &round_keys[round * 16..round * 16 + 16]);
    }
    sub_bytes(block);
    shift_rows(block);
    add_round_key(block, &round_keys[nr * 16..nr * 16 + 16]);
}

/// 解密一个分组（等效逆密码）。
fn decrypt_block(round_keys: &[u8], nr: usize, block: &mut [u8; 16]) {
    let inv = inv_sbox();
    add_round_key(block, &round_keys[nr * 16..nr * 16 + 16]);
    for round in (1..nr).rev() {
        inv_shift_rows(block);
        inv_sub_bytes(block, &inv);
        add_round_key(block, &round_keys[round * 16..round * 16 + 16]);
        inv_mix_columns(block);
    }
    inv_shift_rows(block);
    inv_sub_bytes(block, &inv);
    add_round_key(block, &round_keys[0..16]);
}

/// AES-CBC 解密。
///
/// # 参数
/// - `key`：密钥（16/24/32 字节）
/// - `iv`：初始向量（16 字节）
/// - `data`：密文（16 字节的倍数）
fn aes_cbc_decrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    if iv.len() != 16 {
        return Err(format!("IV 长度必须为 16 字节，实际 {}", iv.len()));
    }
    if data.is_empty() || data.len() % 16 != 0 {
        return Err(format!("密文长度必须是 16 的倍数，实际 {}", data.len()));
    }
    let round_keys = key_expansion(key)?;
    let nr = key.len() / 4 + 6;
    let mut prev: [u8; 16] = iv.try_into().unwrap();
    let mut out = Vec::with_capacity(data.len());
    for chunk in data.chunks(16) {
        let mut block: [u8; 16] = chunk.try_into().unwrap();
        decrypt_block(&round_keys, nr, &mut block);
        for i in 0..16 {
            block[i] ^= prev[i];
        }
        out.extend_from_slice(&block);
        prev.copy_from_slice(chunk);
    }
    Ok(out)
}

/// AES-CBC 加密。
fn aes_cbc_encrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    if iv.len() != 16 {
        return Err(format!("IV 长度必须为 16 字节，实际 {}", iv.len()));
    }
    if data.is_empty() || data.len() % 16 != 0 {
        return Err(format!("明文长度必须是 16 的倍数，实际 {}", data.len()));
    }
    let round_keys = key_expansion(key)?;
    let nr = key.len() / 4 + 6;
    let mut prev: [u8; 16] = iv.try_into().unwrap();
    let mut out = Vec::with_capacity(data.len());
    for chunk in data.chunks(16) {
        let mut block: [u8; 16] = chunk.try_into().unwrap();
        for i in 0..16 {
            block[i] ^= prev[i];
        }
        encrypt_block(&round_keys, nr, &mut block);
        out.extend_from_slice(&block);
        prev = block;
    }
    Ok(out)
}

/// PKCS7 填充（16 字节分组）。
fn pkcs7_pad(data: &[u8]) -> Vec<u8> {
    let pad = 16 - (data.len() % 16);
    let mut out = data.to_vec();
    out.extend(std::iter::repeat_n(pad as u8, pad));
    out
}

/// 去除 PKCS7 填充（16 字节分组）。
fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Err("待去填充数据为空".to_string());
    }
    let pad = data[data.len() - 1] as usize;
    if pad == 0 || pad > 16 || pad > data.len() {
        return Err(format!("非法 PKCS7 填充长度: {pad}"));
    }
    Ok(data[..data.len() - pad].to_vec())
}

// ---------------------------------------------------------------------------
// Base64 / Hex 基础工具（依赖清单受限，纯 Rust 实现）
// ---------------------------------------------------------------------------

/// 标准 Base64 字符表。
const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Base64 编码（带 padding）。
fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
        out.push(BASE64_CHARS[(n >> 18) as usize & 0x3f] as char);
        out.push(BASE64_CHARS[(n >> 12) as usize & 0x3f] as char);
        if chunk.len() > 1 {
            out.push(BASE64_CHARS[(n >> 6) as usize & 0x3f] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(BASE64_CHARS[n as usize & 0x3f] as char);
        } else {
            out.push('=');
        }
    }
    out
}

/// Base64 解码（标准字符集，容忍尾部换行/空白，拒绝非法字符）。
fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let input = input.trim_end_matches('=');
    if input.len() % 4 == 1 {
        return Err(format!("非法的 Base64 长度: {}", input.len()));
    }
    let mut out = Vec::with_capacity(input.len() / 4 * 3);
    let chars = input.as_bytes();
    let mut buf: u32 = 0;
    let mut bits = 0u32;
    for &c in chars {
        let v = match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a' + 26) as u32,
            b'0'..=b'9' => (c - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => return Err(format!("Base64 包含非法字符: {}", c as char)),
        };
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    Ok(out)
}

/// Hex 字符串转字节数组（对应 Java `hexToBytes`：长度必须为偶数、仅含 0-9/a-f/A-F）。
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("无效的十六进制字符串格式：长度必须为偶数".to_string());
    }
    let mut data = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();
    for i in (0..hex.len()).step_by(2) {
        let high = hex_digit(bytes[i]).ok_or_else(|| {
            format!(
                "无效的十六进制字符串格式：包含非法字符 '{}'",
                bytes[i] as char
            )
        })?;
        let low = hex_digit(bytes[i + 1]).ok_or_else(|| {
            format!(
                "无效的十六进制字符串格式：包含非法字符 '{}'",
                bytes[i + 1] as char
            )
        })?;
        data.push((high << 4) + low);
    }
    Ok(data)
}

/// 单个十六进制字符的值。
fn hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wx_rust_common::util::crypto::Sha1;

    /// 官方测试向量（微信开放文档示例，WxJava `WxMaCryptUtilsTest` 同源）：
    /// `decrypt` 与 `decryptAnotherWay` 应得到一致结果。
    #[test]
    fn test_decrypt_matches_decrypt_another_way() {
        let session_key = "tiihtNczf5v6AKRyjwEUhQ==";
        let encrypted_data = "CiyLU1Aw2KjvrjMdj8YKliAjtP4gsMZMQmRzooG2xrDcvSnxIMXFufNstNGTyaGS9uT5geRa0W4oTOb1WT7fJlAC+oNPdbB+3hVbJSRgv+4lGOETKUQz6OYStslQ142dNCuabNPGBzlooOmB231qMM85d2/fV6ChevvXvQP8Hkue1poOFtnEtpyxVLW1zAo6/1Xx1COxFvrc2d7UL/lmHInNlxuacJXwu0fjpXfz/YqYzBIBzD6WUfTIF9GRHpOn/Hz7saL8xz+W//FRAUid1OksQaQx4CMs8LOddcQhULW4ucetDf96JcR3g0gfRK4PC7E/r7Z6xNrXd2UIeorGj5Ef7b1pJAYB6Y5anaHqZ9J6nKEBvB4DnNLIVWSgARns/8wR2SiRS7MNACwTyrGvt9ts8p12PKFdlqYTopNHR1Vf7XjfhQlVsAJdNiKdYmYVoKlaRv85IfVunYzO0IKXsyl7JCUjCpoG20f0a04COwfneQAGGwd5oa+T8yO5hzuyDb/XcxxmK01EpqOyuxINew==";
        let iv = "r7BXXKkLb8qrSNn05n0qiA==";

        let d1 = decrypt(session_key, encrypted_data, iv).expect("decrypt 应成功");
        let d2 =
            decrypt_another_way(session_key, encrypted_data, iv).expect("decryptAnotherWay 应成功");
        assert_eq!(d1, d2);
        // 官方示例期望的解密结果（WxJava test 同源数据）
        assert!(d1.contains("oGZUI0egBJY1zhBYw2KhdUfwVJJE"));
        assert!(d1.contains("\"watermark\""));
    }

    /// 损坏向量的行为说明（WxJava `WxMaCryptUtilsTest.testDecrypt` 的第二个
    /// 样例：`sessionKey=7MG7jbTToVVRWRXVA885rg==`）。该密文与密钥/IV 不匹配
    /// （openssl AES-128-CBC 独立验证结果与本实现逐字节一致），Java 侧仅打印
    /// 乱码不断言；本实现（严格模式）对非法 PKCS7 填充返回错误。
    #[test]
    fn test_decrypt_corrupt_vector_returns_err() {
        let session_key = "7MG7jbTToVVRWRXVA885rg==";
        let encrypted_data = "BY6VOgcWbwGcyrunK0ECWI8rnDsT69DucZ+M78tc1aL9aM/3bEAHFYd4fu7kRjWhD4YfjObw44T9vUqKyHIjbKs6hvtEasZZEIW35x4a91xVgN48ZqZ7MTQqUlP13kDUlkuwYh+/8g8yceu4kNbjowYrhihx+SV7CfjKCveJ7TSepr5Z7aLv1o+rfeelfOwn++WN/YoQsuZ6S3L4fWlWe5DAAUnFUI6cJvxxCohVzbrVXhyH2AqQdSjH2WnMYFeaGFIbcoxMznlk7oEwFn+hBj63dyT/swdYQfEdzuyCBmKXy8d6l1RKVX6Y65coTD8kIlbr+FKsqYrXVUIUBSwehqYuOdhYWZ9Bntl5DWU1oqzAPCnMn2cAIoQpQPKP7IGSxMOvCNAMhVXbE7BvnWuVuGF+AM5tXAa9IVUhcMImGwLQqm4iV5uBd+5OcFObh3A4VJk9iBCBWSkBHa/rV9CVoY0bFv2F9/2Hv82++Ybl274=";
        let iv = "TarMFjnzHVxy8pdS93wQbw==";
        assert!(decrypt(session_key, encrypted_data, iv).is_err());
    }

    /// 基础工具自检：base64 往返。
    #[test]
    fn test_base64_round_trip() {
        for data in [
            &b"hello"[..],
            b"",
            b"a",
            b"ab",
            b"abc",
            b"hello miniprogram encrypted data!",
        ] {
            let enc = base64_encode(data);
            let dec = base64_decode(&enc).unwrap();
            assert_eq!(dec, data);
        }
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
    }

    /// 小程序加密网络通道加解密对称性（WxJava `testEncryptAndDecryptWithEncryptKey`）。
    #[test]
    fn test_encrypt_key_round_trip() {
        let encrypt_key = "VI6BpyrK9XH4i4AIGe86tg==";
        let hex_iv = "6003f73ec441c3866003f73ec441c386";
        let plain1 = "{\"userId\":\"12345\",\"amount\":100}";
        let plain2 = "hello miniprogram";

        for plain in [plain1, plain2] {
            let encrypted = encrypt_with_encrypt_key(encrypt_key, hex_iv, plain).unwrap();
            let decrypted = decrypt_with_encrypt_key(encrypt_key, hex_iv, &encrypted).unwrap();
            assert_eq!(decrypted, plain);
        }
    }

    /// 非法参数校验（对应 Java 抛 IllegalArgumentException 的场景）。
    #[test]
    fn test_invalid_arguments() {
        let encrypt_key = "VI6BpyrK9XH4i4AIGe86tg==";
        // hexIv 奇数长度
        assert!(encrypt_with_encrypt_key(encrypt_key, "abc", "data").is_err());
        // hexIv 含非法字符
        assert!(
            encrypt_with_encrypt_key(encrypt_key, "6003f73ec441c3866003f73ec441z386", "data")
                .is_err()
        );
        // hexIv 解码后不足 16 字节
        assert!(encrypt_with_encrypt_key(encrypt_key, "6003f73ec441c386", "data").is_err());
        // encryptKey 解码后不足 16 字节
        assert!(
            encrypt_with_encrypt_key("AAAAAAAAAAA=", "6003f73ec441c3866003f73ec441c386", "data")
                .is_err()
        );
    }

    /// 消息加解密签名：SHA1（排序后 `&` 连接）。
    #[test]
    fn test_sha1_digest_with_amp() {
        // 与 common WxCryptUtil 消息签名同一语义：token/timestamp/nonce/encrypt 排序 `&` 拼接
        let sig =
            Sha1::digest_with_amp(&["encrypt_1", "nonce_1", "1234567890", "token_1"]).unwrap();
        assert_eq!(sig.len(), 40);
        // 排序不敏感性：换序结果一致
        let sig2 =
            Sha1::digest_with_amp(&["token_1", "encrypt_1", "1234567890", "nonce_1"]).unwrap();
        assert_eq!(sig, sig2);
    }
}
