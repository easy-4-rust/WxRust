//! 可转 JSON 的接口。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.ToJson`。

/// 包含 `to_json()` 方法的接口。
///
/// 用于通用请求发送时把任意请求对象序列化为 JSON 字符串。
pub trait ToJson {
    /// 转换为 JSON 字符串。
    ///
    /// # 返回
    /// JSON 字符串
    fn to_json(&self) -> String;
}

/// 为实现了 `Serialize` 的类型提供默认的 `to_json` 实现。
impl<T: serde::Serialize> ToJson for T {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
