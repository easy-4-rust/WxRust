//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleRejectReason.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleRejectReason；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleRejectReason.java。
pub struct AfterSaleRejectReason {
    #[serde(rename = "reject_reason_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason_type: Option<i32>,
    #[serde(rename = "reject_reason_type_text", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason_type_text: Option<String>,
    #[serde(rename = "reject_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(rename = "reject_scene", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_scene: Option<i32>,
}
