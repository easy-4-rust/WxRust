//! 群发消息发送结果。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.result.WxMpMassSendResult`。
//! 线格式由 `WxMpMassSendResultAdapter` 决定：`errcode`/`errmsg`/`msg_id`/`msg_data_id`。

#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMpMassSendResult {
    #[serde(
        rename = "errcode",
        default,
        deserialize_with = "deserialize_string_or_int"
    )]
    pub error_code: String,
    #[serde(
        rename = "errmsg",
        default,
        deserialize_with = "deserialize_string_or_int"
    )]
    pub error_msg: String,
    #[serde(
        rename = "msg_id",
        default,
        deserialize_with = "deserialize_string_or_int"
    )]
    pub msg_id: String,
    #[serde(
        rename = "msg_data_id",
        default,
        deserialize_with = "deserialize_string_or_int"
    )]
    pub msg_data_id: String,
}

/// 数字或字符串统一解析为 `String`（对应 Java `GsonHelper.getAsString`）。
fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    Ok(match v {
        serde_json::Value::String(s) => s,
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => String::new(),
    })
}

impl WxMpMassSendResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("群发结果解析失败: {e}"))
    }
}
