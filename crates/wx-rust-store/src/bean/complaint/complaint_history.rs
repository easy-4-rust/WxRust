//! 对应 Java `com.binarywang.wxjava.store.bean.complaint.ComplaintHistory.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ComplaintHistory；对应 Java com.binarywang.wxjava.store.bean.complaint.ComplaintHistory.java。
pub struct ComplaintHistory {
    #[serde(rename = "item_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<i32>,
    #[serde(rename = "time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "phone_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "content", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "media_id_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
    #[serde(rename = "after_sale_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_type: Option<i32>,
    #[serde(rename = "after_sale_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_reason: Option<i32>,
}
