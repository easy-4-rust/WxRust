//! SHA1 工具。
//!
//! 对应 Java `me.chanjar.weixin.common.util.crypto.SHA1`。

/// SHA1 摘要工具（微信签名校验）。
///
/// 用于消息签名、jsapi 签名等场景。
use sha1::Digest;

pub struct Sha1;

impl Sha1 {
    /// 串接参数（排序后拼接），生成 sha1 摘要（十六进制小写）。
    ///
    /// # 参数
    /// - `arr`：参与签名的参数数组
    ///
    /// # 返回
    /// sha1 十六进制摘要
    ///
    /// # 错误
    /// 任一参数为空时返回 `IllegalArgumentException` 等价错误。
    pub fn digest(arr: &[&str]) -> Result<String, String> {
        if arr.is_empty() || arr.iter().any(|a| a.is_empty()) {
            return Err(format!("非法请求参数，有部分参数为空 : {arr:?}"));
        }
        let mut sorted: Vec<&str> = arr.to_vec();
        sorted.sort_unstable();
        let joined = sorted.concat();
        Ok(hex::encode(sha1::Sha1::digest(joined.as_bytes())))
    }

    /// 用 `&` 串接参数（排序后拼接，参数间加 `&`），生成 sha1 摘要。
    ///
    /// # 参数
    /// - `arr`：参与签名的参数数组
    ///
    /// # 返回
    /// sha1 十六进制摘要
    ///
    /// # 错误
    /// 任一参数为空时返回错误。
    pub fn digest_with_amp(arr: &[&str]) -> Result<String, String> {
        if arr.is_empty() || arr.iter().any(|a| a.is_empty()) {
            return Err(format!("非法请求参数，有部分参数为空 : {arr:?}"));
        }
        let mut sorted: Vec<&str> = arr.to_vec();
        sorted.sort_unstable();
        let joined = sorted.join("&");
        Ok(hex::encode(sha1::Sha1::digest(joined.as_bytes())))
    }
}
