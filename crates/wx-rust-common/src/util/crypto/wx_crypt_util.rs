//! 微信消息加解密工具。
//!
//! 对应 Java `me.chanjar.weixin.common.util.crypto.WxCryptUtil`。
//!
//! 实现公众号/企业微信消息体的 AES-CBC 加解密（微信方案）：
//! - 加密：`randomStr + 网络序长度 + 明文 + appid` 拼接 → PKCS7 填充 → AES-CBC(NoPadding) → Base64
//! - 解密：Base64 解码 → AES-CBC 解密 → 去填充 → 拆解字段 → 校验 appid
//! - 签名：`SHA1(token, timestamp, nonce, encrypt)` 排序拼接

use aes::Aes256;
use cbc::cipher::block_padding::NoPadding;
use cbc::cipher::{BlockModeDecrypt, BlockModeEncrypt, Iv, Key, KeyIvInit};

use super::byte_group::ByteGroup;
use super::pkcs7_encoder::Pkcs7Encoder;
use super::sha1::Sha1;

/// AES-256-CBC 密码（微信使用 NoPadding，填充由 PKCS7Encoder 自定义处理）
type Aes256Cbc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

/// 加密上下文（`encrypt` 的返回值）。
#[derive(Debug, Clone)]
pub struct EncryptContext {
    /// 加密后的消息密文（Base64）
    pub encrypted_xml: String,
    /// 安全签名
    pub signature: String,
    /// 时间戳
    pub timestamp: String,
    /// 随机串
    pub nonce: String,
}

/// 微信消息加解密工具实例。
#[derive(Debug, Clone)]
pub struct WxCryptUtil {
    /// 消息体加密密钥（44 字节 base64 编码的 aesKey，解码后 32 字节）
    aes_key: Vec<u8>,
    /// 令牌（消息校验 token）
    token: String,
    /// 公众号 appid 或企业微信 corpid
    appid_or_corpid: String,
}

impl WxCryptUtil {
    /// 构建加解密工具。
    ///
    /// # 参数
    /// - `token`：消息校验 token（公众号后台配置）
    /// - `aes_key`：消息加密密钥（EncodingAESKey，44 字符 base64）
    /// - `appid_or_corpid`：公众号 appid 或企业微信 corpid
    ///
    /// # 返回
    /// 构建失败（aesKey 解码异常）时返回错误。
    pub fn new(
        token: impl Into<String>,
        aes_key: impl Into<String>,
        appid_or_corpid: impl Into<String>,
    ) -> Result<Self, String> {
        let token = token.into();
        let aes_key_str = aes_key.into();
        // Java Base64.decodeBase64(aesKey) —— Commons Codec 宽松模式：
        // 容忍无 padding、容忍非规范尾字符（如 43 字符的微信 EncodingAESKey）。
        // 实现：先用 STANDARD/NO_PAD 解码；失败则按 4 字符对齐补 'A'（0 值位）
        // 使尾字符通过严格校验，再解码（等价 Java 宽松行为）。
        let aes_key =
            lenient_base64_decode(&aes_key_str).map_err(|e| format!("aesKey 解码失败: {e}"))?;
        if aes_key.len() != 32 {
            return Err(format!("aesKey 解码后长度应为 32，实际 {}", aes_key.len()));
        }
        Ok(Self {
            aes_key,
            token,
            appid_or_corpid: appid_or_corpid.into(),
        })
    }

    /// 将公众平台回复用户的消息加密打包。
    ///
    /// <ol>
    /// <li>对要发送的消息进行 AES-CBC 加密</li>
    /// <li>生成安全签名</li>
    /// <li>将消息密文和安全签名打包成 xml 格式</li>
    /// </ol>
    ///
    /// # 参数
    /// - `plain_text`：公众平台待回复用户的消息（xml 格式字符串）
    ///
    /// # 返回
    /// 加密后的可直接回复用户的密文（含 msg_signature、timestamp、nonce、encrypt 的 xml）。
    pub fn encrypt(&self, plain_text: &str) -> Result<String, String> {
        let ctx = self.encrypt_context(plain_text)?;
        Ok(Self::generate_xml(
            &ctx.encrypted_xml,
            &ctx.signature,
            &ctx.timestamp,
            &ctx.nonce,
        ))
    }

    /// 将公众平台回复用户的消息加密打包，返回加密所需的值对象。
    ///
    /// # 参数
    /// - `plain_text`：公众平台待回复用户的消息（xml 格式字符串）
    ///
    /// # 返回
    /// 加密消息所需的值对象。
    pub fn encrypt_context(&self, plain_text: &str) -> Result<EncryptContext, String> {
        // 加密
        let random_str = Self::gen_random_str();
        let encrypted_xml = self.encrypt_with_random(&random_str, plain_text)?;

        // 生成安全签名
        let timestamp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs())
        .to_string();
        let nonce = Self::gen_random_str();

        let signature = Sha1::digest_with_amp(&[&self.token, &timestamp, &nonce, &encrypted_xml])?;
        Ok(EncryptContext {
            encrypted_xml,
            signature,
            timestamp,
            nonce,
        })
    }

    /// 对明文进行加密（不打包 xml）。
    ///
    /// # 参数
    /// - `random_str`：随机字符串
    /// - `plain_text`：需要加密的明文
    ///
    /// # 返回
    /// 加密后 base64 编码的字符串。
    pub fn encrypt_with_random(
        &self,
        random_str: &str,
        plain_text: &str,
    ) -> Result<String, String> {
        let mut collector = ByteGroup::new();
        let random_bytes = random_str.as_bytes();
        let plain_bytes = plain_text.as_bytes();
        let size_bytes = Self::number_2_bytes_in_network_order(plain_bytes.len() as i32);
        let appid_bytes = self.appid_or_corpid.as_bytes();

        // randomStr + networkBytesOrder + text + appid
        collector.add_bytes(random_bytes);
        collector.add_bytes(&size_bytes);
        collector.add_bytes(plain_bytes);
        collector.add_bytes(appid_bytes);

        // ... + pad: 使用自定义的填充方式对明文进行补位填充
        let pad_bytes = Pkcs7Encoder::encode(collector.size());
        collector.add_bytes(&pad_bytes);

        // 获得最终的字节流（未加密）
        let unencrypted = collector.to_bytes();

        // AES-256-CBC，Key=IV=aesKey 前 16 字节
        let mut key = Key::<Aes256Cbc>::default();
        key.clone_from_slice(&self.aes_key);
        let mut iv = Iv::<Aes256Cbc>::default();
        iv.clone_from_slice(&self.aes_key[..16]);
        let cipher = Aes256Cbc::new(&key, &iv);

        // CBC 加密（微信长度是 32 的倍数，NoPadding 不额外填充）
        let mut buf = vec![0u8; unencrypted.len()];
        cipher
            .encrypt_padded_b2b::<NoPadding>(&unencrypted, &mut buf)
            .map_err(|e| format!("加密失败: {e}"))?;

        Ok(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            buf,
        ))
    }

    /// 检验消息的真实性，并且获取解密后的明文。
    ///
    /// <ol>
    /// <li>利用收到的密文生成安全签名，进行签名验证</li>
    /// <li>若验证通过，则提取 xml 中的加密消息</li>
    /// <li>对消息进行解密</li>
    /// </ol>
    ///
    /// # 参数
    /// - `msg_signature`：签名串，对应 URL 参数的 msg_signature
    /// - `timestamp`：时间戳，对应 URL 参数的 timestamp
    /// - `nonce`：随机串，对应 URL 参数的 nonce
    /// - `encrypted_xml`：包含 Encrypt 密文的 xml（POST 请求数据）
    ///
    /// # 返回
    /// 解密后的原文。
    pub fn decrypt_xml(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        encrypted_xml: &str,
    ) -> Result<String, String> {
        // 提取密文
        let cipher_text = Self::extract_encrypt_part(encrypted_xml)?;
        self.decrypt_content(msg_signature, timestamp, nonce, &cipher_text)
    }

    /// 验证签名后解密内容。
    pub fn decrypt_content(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        cipher_text: &str,
    ) -> Result<String, String> {
        // 验证安全签名
        let signature = Sha1::digest_with_amp(&[&self.token, timestamp, nonce, cipher_text])?;
        if signature != msg_signature {
            return Err("签名验证错误".to_string());
        }

        // 解密
        let encrypted =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cipher_text)
                .map_err(|e| format!("base64 解码失败: {e}"))?;
        let mut key = Key::<Aes256CbcDec>::default();
        key.clone_from_slice(&self.aes_key);
        let mut iv = Iv::<Aes256CbcDec>::default();
        iv.clone_from_slice(&self.aes_key[..16]);
        let cipher = Aes256CbcDec::new(&key, &iv);

        // CBC 解密
        let mut decrypted_buf = vec![0u8; encrypted.len()];
        let decrypted_all = cipher
            .decrypt_padded_b2b::<NoPadding>(&encrypted, &mut decrypted_buf)
            .map_err(|e| format!("解密失败: {e}"))?
            .to_vec();

        // 去除补位字符
        let decrypted = Pkcs7Encoder::decode(&decrypted_all);

        // 拆分：16 字节随机串 + 4 字节网络序长度 + 明文 + appid
        if decrypted.len() < 20 {
            return Err("解密后数据长度非法".to_string());
        }
        let len_bytes: [u8; 4] = decrypted[16..20].try_into().unwrap();
        let xml_len = Self::bytes_network_order_2_number(&len_bytes) as usize;
        if 20 + xml_len > decrypted.len() {
            return Err("解密后 xml 长度非法".to_string());
        }
        let xml = String::from_utf8_lossy(&decrypted[20..20 + xml_len]).into_owned();
        let from_appid = String::from_utf8_lossy(&decrypted[20 + xml_len..]).into_owned();

        // appid 校验
        if from_appid != self.appid_or_corpid {
            return Err(format!(
                "appid 校验失败：报文 appid={from_appid}，本地 appid={}",
                self.appid_or_corpid
            ));
        }
        Ok(xml)
    }

    /// 对密文进行解密（不校验签名），返回明文。
    ///
    /// 对应 Java `WxCryptUtil.decrypt(String cipherText)`。
    ///
    /// # 参数
    /// - `cipher_text`：Base64 编码的密文
    ///
    /// # 返回
    /// 解密后的明文（含随机串前缀与 appid 校验）。
    pub fn decrypt(&self, cipher_text: &str) -> Result<String, String> {
        let encrypted =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cipher_text)
                .map_err(|e| format!("base64 解码失败: {e}"))?;
        let key = Key::<Aes256CbcDec>::default();
        let mut key_copy = key;
        key_copy.clone_from_slice(&self.aes_key);
        let mut iv = Iv::<Aes256CbcDec>::default();
        iv.clone_from_slice(&self.aes_key[..16]);
        let cipher = Aes256CbcDec::new(&key_copy, &iv);

        let mut decrypted_buf = vec![0u8; encrypted.len()];
        let decrypted_all = cipher
            .decrypt_padded_b2b::<NoPadding>(&encrypted, &mut decrypted_buf)
            .map_err(|e| format!("解密失败: {e}"))?
            .to_vec();

        // 去除补位字符
        let decrypted = Pkcs7Encoder::decode(&decrypted_all);

        // 拆分：16 字节随机串 + 4 字节网络序长度 + 明文 + appid
        if decrypted.len() < 20 {
            return Err("解密后数据长度非法".to_string());
        }
        let len_bytes: [u8; 4] = decrypted[16..20].try_into().unwrap();
        let xml_len = Self::bytes_network_order_2_number(&len_bytes) as usize;
        if 20 + xml_len > decrypted.len() {
            return Err("解密后 xml 长度非法".to_string());
        }
        let xml = String::from_utf8_lossy(&decrypted[20..20 + xml_len]).into_owned();
        let from_appid = String::from_utf8_lossy(&decrypted[20 + xml_len..]).into_owned();

        // appid 校验
        if from_appid != self.appid_or_corpid {
            return Err(format!(
                "appid 校验失败：报文 appid={from_appid}，本地 appid={}",
                self.appid_or_corpid
            ));
        }
        Ok(xml)
    }

    /// 从 xml 中提取 Encrypt 节点的密文。
    fn extract_encrypt_part(xml: &str) -> Result<String, String> {
        // 微信消息 xml 中 Encrypt 节点为 CDATA 包裹；此处用轻量提取（不引入完整 XML 解析）
        let start = xml.find("<Encrypt>").ok_or("xml 中未找到 <Encrypt>")? + "<Encrypt>".len();
        let end = xml[start..]
            .find("</Encrypt>")
            .ok_or("xml 中未找到 </Encrypt>")?;
        let content = &xml[start..start + end];
        // 去除可能的 CDATA 包裹
        let content = content.trim();
        if let Some(rest) = content.strip_prefix("<![CDATA[") {
            if let Some(v) = rest.strip_suffix("]]>") {
                return Ok(v.to_string());
            }
        }
        Ok(content.to_string())
    }

    /// 生成加密响应 xml。
    fn generate_xml(encrypt: &str, signature: &str, timestamp: &str, nonce: &str) -> String {
        format!(
            "<xml>\n<Encrypt><![CDATA[{encrypt}]]></Encrypt>\n<MsgSignature><![CDATA[{signature}]]></MsgSignature>\n<TimeStamp>{timestamp}</TimeStamp>\n<Nonce><![CDATA[{nonce}]]></Nonce>\n</xml>"
        )
    }

    /// int 转 4 字节网络序（大端）。
    fn number_2_bytes_in_network_order(number: i32) -> [u8; 4] {
        (number as u32).to_be_bytes()
    }

    /// 4 字节网络序转 int。
    fn bytes_network_order_2_number(bytes: &[u8]) -> i32 {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&bytes[..4]);
        i32::from_be_bytes(arr)
    }

    /// 生成 16 位随机字符串（对应 Java `genRandomStr`，基于 UUID）。
    pub fn gen_random_str() -> String {
        const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        (0..16)
            .map(|_| {
                let idx = rand::random_range(0..CHARS.len());
                CHARS[idx] as char
            })
            .collect()
    }
}

/// 宽松 Base64 解码（对齐 Java Commons Codec 语义）。
///
/// 先尝试严格/NO_PAD 引擎；若仅因尾字符非规范填充位失败，
/// 则将最后字符的额外位清零后重试（等价 Java 宽松行为）。
fn lenient_base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let s = input.trim();
    // 尝试严格引擎（带 padding）
    if let Ok(v) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s) {
        return Ok(v);
    }
    // 尝试 NO_PAD（无 padding）
    if let Ok(v) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD_NO_PAD, s) {
        return Ok(v);
    }
    // 宽松模式：长度非 4 倍数时，把尾字符的额外位清零（按 4 对齐到合法输入）
    let rem = s.len() % 4;
    if rem == 1 {
        // 1 个尾字符无法构成字节，Java 亦会丢弃
        let s2 = &s[..s.len() - 1];
        return base64::Engine::decode(&base64::engine::general_purpose::STANDARD_NO_PAD, s2)
            .map_err(|e| e.to_string());
    }
    if rem == 2 || rem == 3 {
        // 尾字符只保留 2 位（rem=2）或 4 位（rem=3）有效数据：
        // 将尾字符替换为合法编码（低 2 位清零）
        if let Some(&last) = s.as_bytes().last() {
            if let Some(idx) = base64_charset_index(last) {
                let masked = idx & !0b11;
                if let Some(ch) = base64_charset_char(masked) {
                    let mut s2 = s[..s.len() - 1].to_string();
                    s2.push(ch as char);
                    return base64::Engine::decode(
                        &base64::engine::general_purpose::STANDARD_NO_PAD,
                        &s2,
                    )
                    .map_err(|e| e.to_string());
                }
            }
        }
    }
    Err("无法解码".to_string())
}

/// 返回 base64 字符在标准字符集中的索引。
fn base64_charset_index(c: u8) -> Option<u8> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    CHARS.iter().position(|&x| x == c).map(|i| i as u8)
}

/// 返回 base64 索引对应的字符。
fn base64_charset_char(idx: u8) -> Option<u8> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    CHARS.get(idx as usize).copied()
}
