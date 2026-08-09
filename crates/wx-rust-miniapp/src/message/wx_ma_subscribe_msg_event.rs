//! 订阅消息事件。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaSubscribeMsgEvent`。
//! - XML 侧（XStream）：`SubscribeMsgPopupEvent`/`SubscribeMsgChangeEvent`/
//!   `SubscribeMsgSentEvent` 三个事件容器，事件项位于 `List` 元素下。
//! - JSON 侧（`WxMaSubscribeMsgEventJsonAdapter`）：`List` 可能是对象或数组，
//!   事件类型按字段探测：含 `PopupScene` 为弹窗事件、含 `MsgID`/`ErrorCode`/
//!   `ErrorStatus` 为发送事件，否则为变更事件。

use serde::{Deserialize, Deserializer, Serialize};

/// 弹窗事件项（对应 Java `WxMaSubscribeMsgEvent.PopupEvent`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PopupEvent {
    /// 模板 id。
    #[serde(rename = "TemplateId", default)]
    pub template_id: String,
    /// 订阅结果（accept 接收；reject 拒收）。
    #[serde(rename = "SubscribeStatusString", default)]
    pub subscribe_status_string: String,
    /// 弹框场景，0 代表在小程序页面内。
    #[serde(rename = "PopupScene", default)]
    pub popup_scene: String,
}

/// 变更事件项（对应 Java `WxMaSubscribeMsgEvent.ChangeEvent`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChangeEvent {
    /// 模板 id。
    #[serde(rename = "TemplateId", default)]
    pub template_id: String,
    /// 订阅结果（accept 接收；reject 拒收）。
    #[serde(rename = "SubscribeStatusString", default)]
    pub subscribe_status_string: String,
}

/// 发送事件项（对应 Java `WxMaSubscribeMsgEvent.SentEvent`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SentEvent {
    /// 模板 id。
    #[serde(rename = "TemplateId", default)]
    pub template_id: String,
    /// 消息 id。
    #[serde(rename = "MsgID", default)]
    pub msg_id: String,
    /// 错误码。
    #[serde(rename = "ErrorCode", default)]
    pub error_code: String,
    /// 错误状态。
    #[serde(rename = "ErrorStatus", default)]
    pub error_status: String,
}

/// 订阅消息弹窗事件（对应 Java `WxMaSubscribeMsgEvent.SubscribeMsgPopupEvent`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SubscribeMsgPopupEvent {
    /// 弹窗事件列表。
    #[serde(rename = "List", default)]
    pub list: Vec<PopupEvent>,
}

/// 订阅消息变更事件（对应 Java `WxMaSubscribeMsgEvent.SubscribeMsgChangeEvent`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SubscribeMsgChangeEvent {
    /// 变更事件列表。
    #[serde(rename = "List", default)]
    pub list: Vec<ChangeEvent>,
}

/// 订阅消息发送事件（对应 Java `WxMaSubscribeMsgEvent.SubscribeMsgSentEvent`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SubscribeMsgSentEvent {
    /// 发送事件（Java 为单对象字段）。
    #[serde(rename = "List", default)]
    pub list: Option<SentEvent>,
}

/// JSON 侧事件聚合（对应 Java `WxMaSubscribeMsgEvent.WxMaSubscribeMsgEventJson`，
/// 由 `WxMaSubscribeMsgEventJsonAdapter` 解析 `List` 对象/数组）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaSubscribeMsgEventJson {
    /// 弹窗事件（List 中存在 PopupScene 字段的项）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup_events: Option<SubscribeMsgPopupEvent>,
    /// 变更事件（其余项）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_events: Option<SubscribeMsgChangeEvent>,
    /// 发送事件（含 MsgID/ErrorCode/ErrorStatus 的项，后到者覆盖）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_event: Option<SubscribeMsgSentEvent>,
}

impl WxMaSubscribeMsgEventJson {
    /// 从 `List` 元素（对象或数组）解析事件聚合（对应 Java adapter 的
    /// `deserialize`：数组取首项探测类型后逐项归集）。
    pub fn from_list_value(value: &serde_json::Value) -> Result<Self, String> {
        let mut result = WxMaSubscribeMsgEventJson::default();
        let items: Vec<&serde_json::Value> = match value {
            serde_json::Value::Array(arr) => arr.iter().collect(),
            serde_json::Value::Object(_) => vec![value],
            other => {
                return Err(format!("List 应为对象或数组，实际为: {other}"));
            }
        };
        if items.is_empty() {
            return Ok(result);
        }
        let event_type = detect_msg_event_type(items[0]);
        for item in items {
            set_field(&mut result, event_type, item)?;
        }
        Ok(result)
    }
}

/// 事件类型（对应 Java `MsgEventTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MsgEventType {
    Popup,
    Change,
    Sent,
}

/// 探测事件类型（对应 Java `detectMsgEventType`）。
fn detect_msg_event_type(obj: &serde_json::Value) -> MsgEventType {
    if let Some(obj) = obj.as_object() {
        if obj.contains_key("PopupScene") {
            return MsgEventType::Popup;
        }
        if obj.contains_key("MsgID")
            || obj.contains_key("ErrorCode")
            || obj.contains_key("ErrorStatus")
        {
            return MsgEventType::Sent;
        }
    }
    MsgEventType::Change
}

/// 按事件类型归集一项（对应 Java `setField`）。
fn set_field(
    target: &mut WxMaSubscribeMsgEventJson,
    event_type: MsgEventType,
    json: &serde_json::Value,
) -> Result<(), String> {
    let obj = json
        .as_object()
        .ok_or_else(|| "List 项应为 JSON 对象".to_string())?;
    let get = |k: &str| {
        obj.get(k)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    match event_type {
        MsgEventType::Popup => {
            let popup_event = PopupEvent {
                template_id: get("TemplateId"),
                subscribe_status_string: get("SubscribeStatusString"),
                popup_scene: get("PopupScene"),
            };
            let holder = target
                .popup_events
                .get_or_insert_with(SubscribeMsgPopupEvent::default);
            holder.list.push(popup_event);
        }
        MsgEventType::Change => {
            let change_event = ChangeEvent {
                template_id: get("TemplateId"),
                subscribe_status_string: get("SubscribeStatusString"),
            };
            let holder = target
                .change_events
                .get_or_insert_with(SubscribeMsgChangeEvent::default);
            holder.list.push(change_event);
        }
        MsgEventType::Sent => {
            let sent_event = SentEvent {
                template_id: get("TemplateId"),
                msg_id: get("MsgID"),
                error_code: get("ErrorCode"),
                error_status: get("ErrorStatus"),
            };
            let holder = target
                .sent_event
                .get_or_insert_with(SubscribeMsgSentEvent::default);
            holder.list = Some(sent_event);
        }
    }
    Ok(())
}

/// 从 JSON 构建事件聚合（`{"List": ...}`，对应 Java adapter 的注册场景）。
pub fn from_json(json: &str) -> Result<WxMaSubscribeMsgEventJson, String> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("订阅消息事件解析失败: {e}"))?;
    let list = value
        .get("List")
        .ok_or_else(|| "缺少 List 字段".to_string())?;
    WxMaSubscribeMsgEventJson::from_list_value(list)
}

// ---------------------------------------------------------------------------
// serde 支持：事件项直接序列化；聚合体以 `List` 键承载（XML/JSON 双格式）
// ---------------------------------------------------------------------------

/// 订阅消息事件聚合（对外类型，序列化为 `{"List": ...}`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct WxMaSubscribeMsgEvent {
    /// 事件聚合（popup/change/sent）。
    #[serde(flatten)]
    pub inner: WxMaSubscribeMsgEventJson,
}

impl<'de> Deserialize<'de> for WxMaSubscribeMsgEvent {
    /// 接受 `{"List": 对象|数组}` 线格式（对应
    /// `WxMaSubscribeMsgEventJsonAdapter` 注册于 Gson 的解析语义）。
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        let inner = WxMaSubscribeMsgEventJson::from_list_value(
            value
                .get("List")
                .ok_or_else(|| serde::de::Error::custom("缺少 List 字段"))?,
        )
        .map_err(serde::de::Error::custom)?;
        Ok(WxMaSubscribeMsgEvent { inner })
    }
}
