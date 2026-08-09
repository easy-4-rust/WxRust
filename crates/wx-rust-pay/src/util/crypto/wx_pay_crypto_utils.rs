//! 支付签名基础算法（MD5/HmacSHA256/SHA1）。
//!
//! 对应 Java 侧：
//! - MD5：`org.apache.commons.codec.digest.DigestUtils.md5Hex`（v2 签名
//!   `SignType.MD5`，最终以大写拼入报文）；
//! - HmacSHA256：`me.chanjar.weixin.common.util.SignUtils.createHmacSha256Sign`
//!   （v2 签名 `SignType.HMAC_SHA256`，十六进制大写）；
//! - SHA1：`DigestUtils.sha1Hex`（支付分/回调解密等场景）。
//!
//! 全部为纯函数，无状态。

use hmac::{Hmac, KeyInit, Mac};
use md5::{Digest as Md5Digest, Md5};
use sha2::Sha256;

/// HmacSHA256 签名器类型。
type HmacSha256 = Hmac<Sha256>;

/// 计算字符串的 MD5 十六进制（小写，对应 Java `DigestUtils.md5Hex`；
/// v2 签名使用时需 `.to_uppercase()`，见 `SignUtils::create_sign`）。
pub fn md5_hex(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(data.as_bytes());
    let digest = hasher.finalize();
    hex::encode(digest)
}

/// 计算字符串的 HmacSHA256 签名（十六进制大写，对应 Java common
/// `SignUtils.createHmacSha256Sign` 的
/// `Hex.encodeHexString(bytes).toUpperCase()`）。
pub fn hmac_sha256_hex(message: &str, key: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC 初始化不应失败");
    mac.update(message.as_bytes());
    let bytes = mac.finalize().into_bytes();
    hex::encode_upper(bytes)
}

/// 计算字符串的 SHA1 十六进制（小写，对应 Java `DigestUtils.sha1Hex`）。
pub fn sha1_hex(data: &str) -> String {
    use sha1::Digest;
    let mut hasher = sha1::Sha1::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

/// 计算字节的 SHA256 十六进制（小写，对应 Java `DigestUtils.sha256Hex`，
/// 媒体上传 `Wechatpay-Sha256` 场景）。
pub fn sha256_hex(data: &[u8]) -> String {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
