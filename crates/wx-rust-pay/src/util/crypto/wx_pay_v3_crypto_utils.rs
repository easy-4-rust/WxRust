//! 微信支付 v3 签名/验签与 AES-GCM 加解密工具。
//!
//! 对应 Java（weixin-java-pay）：
//! - 请求签名与 Authorization 头：`v3/auth/WxPayCredentials`（`getToken`/
//!   `buildMessage`/`getSchema`）、`v3/auth/PrivateKeySigner#sign`
//!   （SHA256withRSA，Base64 输出）、`v3/util/SignUtils#sign`；
//! - 响应/通知验签：`v3/auth/WxPayValidator`（`buildMessage`：
//!   `timestamp\nnonce\nbody\n`）、`v3/auth/PublicCertificateVerifier#verify`；
//! - AES-GCM：`v3/util/AesUtils`（AEAD_AES_256_GCM，apiV3Key，128 位 tag，
//!   ciphertext 为密文+tag 的 Base64）；
//! - 敏感信息 RSA-OAEP：`v3/util/RsaCryptoUtil#encryptOAEP`/`#decryptOAEP`
//!   （`RSA/ECB/OAEPWithSHA-1AndMGF1Padding`）。
//!
//! 全部为无状态纯函数（Java 侧均为静态工具/无状态对象）。

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use rand_core::{OsRng, RngCore};
use rsa::Oaep;
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::sha2::Digest;
use rsa::{RsaPrivateKey, RsaPublicKey};
use wx_rust_common::error::{WxErrorException, WxRuntimeError};

/// Authorization 头 schema（对应 Java `WxPayCredentials.getSchema()`）。
pub const AUTHORIZATION_SCHEMA: &str = "WECHATPAY2-SHA256-RSA2048";

/// nonce 字符集（对应 Java `WxPayCredentials.SYMBOLS`）。
const NONCE_SYMBOLS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// AES-GCM nonce 长度（12 字节，AEAD_AES_256_GCM 规范）。
const GCM_NONCE_LEN: usize = 12;

/// apiV3Key 长度（对应 Java `AesUtils.KEY_LENGTH_BYTE = 32`）。
const AES_KEY_LEN: usize = 32;

/// v3 加密/签名错误。
///
/// 文案对应 Java 各工具类的异常消息（`WxRuntimeException`/
/// `IllegalArgumentException`/`BadPaddingException` 等）。
#[derive(Debug, thiserror::Error)]
pub enum WxV3CryptoError {
    /// apiV3Key 长度错误（对应 Java `AesUtils` 构造器：
    /// "无效的ApiV3Key，长度必须为32个字节"）
    #[error("无效的ApiV3Key，长度必须为32个字节")]
    InvalidApiV3Key,
    /// nonce 长度错误（AEAD_AES_256_GCM 要求 12 字节）
    #[error("无效的nonce，长度必须为12个字节: {0}")]
    InvalidNonce(String),
    /// Base64 解码失败
    #[error("无效的Base64编码: {0}")]
    InvalidBase64(String),
    /// 签名计算失败（对应 Java `PrivateKeySigner`："签名计算失败"）
    #[error("签名计算失败: {0}")]
    SignFailed(String),
    /// 签名验证过程发生错误（对应 Java `PublicCertificateVerifier`：
    /// "签名验证过程发生了错误"）
    #[error("签名验证过程发生了错误: {0}")]
    VerifyFailed(String),
    /// 无效的私钥（对应 Java `PrivateKeySigner`："无效的私钥"）
    #[error("无效的私钥: {0}")]
    InvalidPrivateKey(String),
    /// 无效的证书（对应 Java `PublicCertificateVerifier`："无效的证书"）
    #[error("无效的证书: {0}")]
    InvalidCertificate(String),
    /// AES-GCM 解密失败（对应 Java `AesUtils` 抛出的 JCE 认证失败）
    #[error("解密失败: {0}")]
    DecryptFailed(String),
    /// RSA-OAEP 加密超长（对应 Java `RsaCryptoUtil`：
    /// "加密原串的长度不能超过214字节"）
    #[error("加密原串的长度不能超过214字节: {0}")]
    MessageTooLong(String),
    /// URL 解析失败（对应 Java `WxPayCredentials.buildMessage` 的 URI 解析）
    #[error("无效的URL: {0}")]
    InvalidUrl(String),
}

impl From<WxV3CryptoError> for WxErrorException {
    fn from(e: WxV3CryptoError) -> Self {
        // Java 侧这些工具方法抛 WxRuntimeException/IllegalArgumentException，
        // Rust 统一映射为运行时错误
        WxErrorException::Runtime(WxRuntimeError::new(e.to_string()))
    }
}

/// 生成 32 位随机 nonce 串（对应 Java `WxPayCredentials.generateNonceStr()`：
/// 从 `SYMBOLS` 中按 `SecureRandom` 取 32 个字符）。
pub fn gen_nonce_str() -> String {
    let mut rng = OsRng;
    let mut nonce = String::with_capacity(32);
    for _ in 0..32 {
        let idx = (rng.next_u32() as usize) % NONCE_SYMBOLS.len();
        nonce.push(NONCE_SYMBOLS.as_bytes()[idx] as char);
    }
    nonce
}

/// 生成当前 Unix 秒级时间戳（对应 Java `WxPayCredentials.generateTimestamp()`：
/// `System.currentTimeMillis() / 1000`）。
pub fn gen_timestamp() -> i64 {
    // chrono::Utc::now().timestamp() 与 System.currentTimeMillis()/1000 同为
    // Unix 秒；wx-rust-common 提供 RandomUtils 封装，这里直接用标准库时间
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}

/// 生成 12 字节随机 nonce（AEAD_AES_256_GCM 加密场景使用，退款回调/商家券
/// 加密需要商户自行生成 nonce）。
pub fn gen_gcm_nonce() -> [u8; GCM_NONCE_LEN] {
    let mut nonce = [0u8; GCM_NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// SHA256withRSA 签名，Base64 输出（对应 Java `PrivateKeySigner#sign` 与
/// `v3/util/SignUtils#sign`）。
///
/// 语义与 Java `Signature.getInstance("SHA256withRSA")` 对齐：先对消息做
/// SHA-256 摘要，再做 PKCS#1 v1.5 填充加密。PKCS#1 v1.5 签名为确定值
/// （盲化只影响中间计算），可用于 golden 断言。
///
/// # 参数
/// - `private_key`：商户 API 私钥
/// - `message`：待签名消息字节（v3 签名串）
///
/// # 返回
/// Base64 编码的 256 字节签名
pub fn sign_sha256_rsa(
    private_key: &RsaPrivateKey,
    message: &[u8],
) -> Result<String, WxV3CryptoError> {
    // rsa 0.9 的 Pkcs1v15Sign::new 要求输入为已哈希摘要
    // （对应 Java Signature.update(message) 后 sign() 的内部哈希步骤）
    let digest = rsa::sha2::Sha256::digest(message);
    let mut rng = OsRng;
    let signature = private_key
        .sign_with_rng(&mut rng, Pkcs1v15Sign::new::<rsa::sha2::Sha256>(), &digest)
        .map_err(|e| WxV3CryptoError::SignFailed(e.to_string()))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(signature))
}

/// SHA256withRSA 验签（对应 Java `PublicCertificateVerifier#verify`）。
///
/// # 参数
/// - `public_key`：平台证书公钥（或微信支付公钥）
/// - `message`：被签名消息字节
/// - `signature_b64`：Base64 编码的签名（`Wechatpay-Signature` 头）
///
/// # 返回
/// `Ok(true)` 验签通过；`Ok(false)` 验签不通过（签名不匹配/格式错误）；
/// Base64 解码失败等返回 `Err`
pub fn verify_sha256_rsa(
    public_key: &RsaPublicKey,
    message: &[u8],
    signature_b64: &str,
) -> Result<bool, WxV3CryptoError> {
    let signature = base64::engine::general_purpose::STANDARD
        .decode(signature_b64)
        .map_err(|e| WxV3CryptoError::InvalidBase64(e.to_string()))?;
    // 与 sign 对应：先做 SHA-256 摘要再验签
    let digest = rsa::sha2::Sha256::digest(message);
    match public_key.verify(
        Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
        &digest,
        &signature,
    ) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// 构造请求签名串（对应 Java `WxPayCredentials.buildMessage`）：
/// `METHOD\ncanonicalUrl\nTIMESTAMP\nNONCE\nBODY\n`。
///
/// `canonicalUrl` 为 URL 的 path（含路径前缀裁剪后）+ `?query`（有则拼）；
/// 空 body（GET）时 body 段为空字符串，但结尾 `\n` 保留。
///
/// # 参数
/// - `method`：HTTP 方法（GET/POST/PUT/PATCH/DELETE，大写）
/// - `canonical_url`：规范化 URL（path + query）
/// - `timestamp`：Unix 秒级时间戳
/// - `nonce`：随机串
/// - `body`：请求体原文（GET 为空字符串）
pub fn build_request_message(
    method: &str,
    canonical_url: &str,
    timestamp: i64,
    nonce: &str,
    body: &str,
) -> String {
    format!("{method}\n{canonical_url}\n{timestamp}\n{nonce}\n{body}\n")
}

/// 拼接规范化 URL：`path` + `?query`（对应 Java `WxPayCredentials.buildMessage`
/// 的 `uri.getRawPath()` + `uri.getRawQuery()`）。
pub fn canonical_url(path: &str, query: Option<&str>) -> String {
    match query {
        Some(q) if !q.is_empty() => format!("{path}?{q}"),
        _ => path.to_string(),
    }
}

/// 从完整 URL 提取规范化 URL（path + query，对应 Java `buildMessage` 对
/// `request.getURI()` 的处理）。
pub fn canonical_url_from_url(url_str: &str) -> Result<String, WxV3CryptoError> {
    let url = url::Url::parse(url_str).map_err(|e| WxV3CryptoError::InvalidUrl(e.to_string()))?;
    // url crate 的 path()/query() 返回原始（percent-encoded）形式，
    // 对应 Java 的 getRawPath()/getRawQuery()
    Ok(canonical_url(url.path(), url.query()))
}

/// 构造 Authorization token 体（对应 Java `WxPayCredentials.getToken` 拼串）：
/// `mchid="..",nonce_str="..",timestamp="..",serial_no="..",signature=".."`
pub fn build_authorization_token(
    mch_id: &str,
    nonce_str: &str,
    timestamp: i64,
    serial_no: &str,
    signature: &str,
) -> String {
    format!(
        "mchid=\"{mch_id}\",nonce_str=\"{nonce_str}\",timestamp=\"{timestamp}\",\
serial_no=\"{serial_no}\",signature=\"{signature}\""
    )
}

/// 构造完整 v3 Authorization 请求头（对应 Java
/// `WxPayV3HttpClientBuilder` + `WxPayCredentials` 全流程）：
/// `WECHATPAY2-SHA256-RSA2048 ` + token。
///
/// # 参数
/// - `mch_id`：商户号
/// - `serial_no`：商户 API 证书序列号
/// - `private_key`：商户 API 私钥
/// - `method`：HTTP 方法（大写）
/// - `canonical_url`：规范化 URL（path + query）
/// - `timestamp`：Unix 秒级时间戳（由调用方生成，便于测试/重放控制）
/// - `nonce`：随机串（由调用方生成）
/// - `body`：请求体原文
pub fn create_authorization_header(
    mch_id: &str,
    serial_no: &str,
    private_key: &RsaPrivateKey,
    method: &str,
    canonical_url: &str,
    timestamp: i64,
    nonce: &str,
    body: &str,
) -> Result<String, WxV3CryptoError> {
    let message = build_request_message(method, canonical_url, timestamp, nonce, body);
    let signature = sign_sha256_rsa(private_key, message.as_bytes())?;
    let token = build_authorization_token(mch_id, nonce, timestamp, serial_no, &signature);
    Ok(format!("{AUTHORIZATION_SCHEMA} {token}"))
}

/// 构造响应验签串（对应 Java `WxPayValidator.buildMessage`）：
/// `timestamp\nnonce\nbody\n`。
pub fn build_response_message(timestamp: &str, nonce: &str, body: &str) -> String {
    format!("{timestamp}\n{nonce}\n{body}\n")
}

/// 响应验签入口（对应 Java `WxPayValidator.validate` + verifier.verify）。
///
/// # 参数
/// - `public_key`：平台证书公钥
/// - `timestamp`：`Wechatpay-Timestamp` 头
/// - `nonce`：`Wechatpay-Nonce` 头
/// - `body`：响应体原文（JSON）
/// - `signature_b64`：`Wechatpay-Signature` 头
pub fn verify_response_signature(
    public_key: &RsaPublicKey,
    timestamp: &str,
    nonce: &str,
    body: &str,
    signature_b64: &str,
) -> Result<bool, WxV3CryptoError> {
    let message = build_response_message(timestamp, nonce, body);
    verify_sha256_rsa(public_key, message.as_bytes(), signature_b64)
}

/// AES-256-GCM 解密为字符串（对应 Java
/// `AesUtils.decryptToString(associatedData, nonce, ciphertext, apiV3Key)`：
/// 通知回调 resource 解密）。
///
/// 语义与 Java 逐行对齐：apiV3Key UTF-8 字节为 AES-256 密钥，nonce UTF-8
/// 字节（12 字节），associatedData 作为 AAD，ciphertext Base64 解码后为
/// 密文+16 字节 tag 的拼接（`AES/GCM/NoPadding`，128 位 tag）。
///
/// # 参数
/// - `api_v3_key`：APIv3 密钥（32 字节）
/// - `associated_data`：附加数据（`resource.associated_data`，可能为空串）
/// - `nonce`：随机串（`resource.nonce`）
/// - `ciphertext_b64`：密文 Base64（`resource.ciphertext`）
pub fn aes_gcm_decrypt(
    api_v3_key: &str,
    associated_data: &str,
    nonce: &str,
    ciphertext_b64: &str,
) -> Result<String, WxV3CryptoError> {
    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(ciphertext_b64)
        .map_err(|e| WxV3CryptoError::InvalidBase64(e.to_string()))?;
    let plaintext = aes_gcm_decrypt_bytes(
        api_v3_key.as_bytes(),
        Some(associated_data.as_bytes()),
        nonce.as_bytes(),
        &ciphertext,
    )?;
    String::from_utf8(plaintext).map_err(|e| WxV3CryptoError::DecryptFailed(e.to_string()))
}

/// AES-256-GCM 解密为字节（对应 Java `AesUtils.decryptToByte(associatedData,
/// nonce, cipherData, key)`；associatedData 为 null 时不传 AAD）。
pub fn aes_gcm_decrypt_bytes(
    api_v3_key: &[u8],
    associated_data: Option<&[u8]>,
    nonce: &[u8],
    ciphertext_with_tag: &[u8],
) -> Result<Vec<u8>, WxV3CryptoError> {
    if api_v3_key.len() != AES_KEY_LEN {
        return Err(WxV3CryptoError::InvalidApiV3Key);
    }
    if nonce.len() != GCM_NONCE_LEN {
        // 先校验长度再构造 Nonce（aead 的 Nonce::from_slice 长度不符会 panic）
        return Err(WxV3CryptoError::InvalidNonce(hex::encode(nonce)));
    }
    // 通过 [u8; 12] 构造 Nonce，避免使用 hybrid-array 已弃用的 from_slice
    let nonce_arr: [u8; GCM_NONCE_LEN] = nonce
        .try_into()
        .map_err(|_| WxV3CryptoError::InvalidNonce(hex::encode(nonce)))?;
    let cipher =
        Aes256Gcm::new_from_slice(api_v3_key).map_err(|_| WxV3CryptoError::InvalidApiV3Key)?;
    let payload = match associated_data {
        Some(aad) => Payload {
            msg: ciphertext_with_tag,
            aad,
        },
        None => Payload {
            msg: ciphertext_with_tag,
            aad: &[],
        },
    };
    cipher
        .decrypt(&Nonce::from(nonce_arr), payload)
        .map_err(|e| WxV3CryptoError::DecryptFailed(e.to_string()))
}

/// AES-256-GCM 加密为 Base64 字符串（退款回调/商家券等加密场景；对应
/// 官方 SDK `AeadAesCipher#encrypt`：Base64(密文 || 16 字节 tag)）。
///
/// # 参数
/// - `api_v3_key`：APIv3 密钥（32 字节）
/// - `associated_data`：附加数据（AAD）
/// - `nonce`：12 字节随机串（可用 [`gen_gcm_nonce`] 生成）
/// - `plaintext`：明文
///
/// # 返回
/// Base64(密文+tag)，可直接作为 `resource.ciphertext` 使用
pub fn aes_gcm_encrypt(
    api_v3_key: &str,
    associated_data: &str,
    nonce: &[u8],
    plaintext: &str,
) -> Result<String, WxV3CryptoError> {
    if api_v3_key.len() != AES_KEY_LEN {
        return Err(WxV3CryptoError::InvalidApiV3Key);
    }
    if nonce.len() != GCM_NONCE_LEN {
        // 先校验长度再构造 Nonce（aead 的 Nonce::from_slice 长度不符会 panic）
        return Err(WxV3CryptoError::InvalidNonce(hex::encode(nonce)));
    }
    // 通过 [u8; 12] 构造 Nonce，避免使用 hybrid-array 已弃用的 from_slice
    let nonce_arr: [u8; GCM_NONCE_LEN] = nonce
        .try_into()
        .map_err(|_| WxV3CryptoError::InvalidNonce(hex::encode(nonce)))?;
    let cipher = Aes256Gcm::new_from_slice(api_v3_key.as_bytes())
        .map_err(|_| WxV3CryptoError::InvalidApiV3Key)?;
    let payload = Payload {
        msg: plaintext.as_bytes(),
        aad: associated_data.as_bytes(),
    };
    let ciphertext = cipher
        .encrypt(&Nonce::from(nonce_arr), payload)
        .map_err(|e| WxV3CryptoError::DecryptFailed(e.to_string()))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(ciphertext))
}

/// RSA-OAEP（SHA-1）加密敏感信息（对应 Java `RsaCryptoUtil.encryptOAEP`：
/// `RSA/ECB/OAEPWithSHA-1AndMGF1Padding`，退款/分账等 `@SpecEncrypt` 字段）。
///
/// 单块 RSA-OAEP-2048 明文上限 214 字节（与 Java 错误文案一致）。
///
/// # 参数
/// - `public_key`：平台证书公钥（对应 Java `certificate.getPublicKey()`）
/// - `message`：明文（如姓名/账号等敏感字段）
///
/// # 返回
/// Base64 编码密文
pub fn rsa_oaep_encrypt(
    public_key: &RsaPublicKey,
    message: &str,
) -> Result<String, WxV3CryptoError> {
    let mut rng = OsRng;
    let ciphertext = public_key
        .encrypt(&mut rng, Oaep::new::<sha1::Sha1>(), message.as_bytes())
        .map_err(|e| WxV3CryptoError::MessageTooLong(e.to_string()))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(ciphertext))
}

/// RSA/ECB/PKCS1Padding 加密（Base64，对应 Java `RsaCryptoUtil` 的
/// `RSA/ECB/PKCS1Padding` 通道，海关报关 verifyCertificate 场景）。
pub fn rsa_pkcs1_encrypt(
    public_key: &RsaPublicKey,
    message: &str,
) -> Result<String, WxV3CryptoError> {
    let mut rng = OsRng;
    let ciphertext = public_key
        .encrypt(&mut rng, rsa::pkcs1v15::Pkcs1v15Encrypt, message.as_bytes())
        .map_err(|e| WxV3CryptoError::MessageTooLong(e.to_string()))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(ciphertext))
}

/// RSA-OAEP（SHA-1）解密（对应 Java `RsaCryptoUtil.decryptOAEP`）。
///
/// # 参数
/// - `private_key`：商户 API 私钥
/// - `ciphertext_b64`：Base64 编码密文
///
/// # 返回
/// 明文字符串
pub fn rsa_oaep_decrypt(
    private_key: &RsaPrivateKey,
    ciphertext_b64: &str,
) -> Result<String, WxV3CryptoError> {
    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(ciphertext_b64)
        .map_err(|e| WxV3CryptoError::InvalidBase64(e.to_string()))?;
    let plaintext = private_key
        .decrypt(Oaep::new::<sha1::Sha1>(), &ciphertext)
        .map_err(|e| WxV3CryptoError::DecryptFailed(e.to_string()))?;
    String::from_utf8(plaintext).map_err(|e| WxV3CryptoError::DecryptFailed(e.to_string()))
}
