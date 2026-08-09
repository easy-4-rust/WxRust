//! 签名工具类。
//!
//! 对应 Java `me.chanjar.weixin.common.util.SignUtils`。

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

/// HmacSHA256 签名器类型
type HmacSha256 = Hmac<Sha256>;

/// 签名工具。
pub struct SignUtils;

impl SignUtils {
    /// 生成 HmacSHA256 签名（十六进制大写）。
    ///
    /// # 参数
    /// - `message`：签名数据
    /// - `key`：签名密钥
    ///
    /// # 返回
    /// 十六进制大写签名结果
    pub fn create_hmac_sha256_sign(message: &str, key: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC 初始化不应失败");
        mac.update(message.as_bytes());
        let bytes = mac.finalize().into_bytes();
        hex::encode_upper(bytes)
    }
}
