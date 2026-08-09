//! 视频号 消息（兼容 JSON 和 XML）。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.WxChannelMessage.java`：
//! 回调消息基类，7 个公共字段（`ToUserName`/`FromUserName`/`CreateTime`/
//! `MsgType`/`Event`/`Encrypt`/`MsgId`，XML 中带 CDATA，quick-xml serde
//! 自动合并 Text/CData 事件，无需额外注解）。

use serde::{Deserialize, Serialize};

/// 视频号 消息（对应 Java `WxChannelMessage`）。
///
/// JSON 与 XML 双线格式：字段名与 Java `@JsonProperty`/`@JacksonXmlProperty`
/// 一致；`MsgID` 为 `MsgId` 的兼容别名（对应 Java `msgIdFill` setter）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxChannelMessage {
    /// 开发者微信号（对应 Java `toUser`）。
    #[serde(rename = "ToUserName", default)]
    pub to_user: Option<String>,
    /// 发送方帐号（对应 Java `fromUser`）。
    #[serde(rename = "FromUserName", default)]
    pub from_user: Option<String>,
    /// 消息创建时间（整型，对应 Java `createTime`）。
    #[serde(rename = "CreateTime", default)]
    pub create_time: Option<i64>,
    /// 消息类型（对应 Java `msgType`）。
    #[serde(rename = "MsgType", default)]
    pub msg_type: Option<String>,
    /// 事件类型（对应 Java `event`）。
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 加密字段（对应 Java `encrypt`）。
    #[serde(rename = "Encrypt", default)]
    pub encrypt: Option<String>,
    /// 消息id（对应 Java `msgId`）。
    #[serde(rename = "MsgId", alias = "MsgID", default)]
    pub msg_id: Option<i64>,
}

impl WxChannelMessage {
    /// 序列化为 JSON（对应 Java `toJson()` 与 `toString()`）。
    ///
    /// Jackson `JsonUtils` 全局 `NON_NULL`：null 字段不输出；Rust 序列化后
    /// 递归移除 null 值表达同一语义（ADAPTED：serde 无全局 NON_NULL）。
    pub fn to_json(&self) -> String {
        let value = serde_json::to_value(self).unwrap_or(serde_json::Value::Null);
        serde_json::to_string(&strip_nulls(value)).unwrap_or_default()
    }
}

/// 递归移除 null 值（对应 Jackson `JsonInclude.Include.NON_NULL`）。
fn strip_nulls(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => serde_json::Value::Object(
            map.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, strip_nulls(v)))
                .collect(),
        ),
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(strip_nulls).collect())
        }
        other => other,
    }
}
