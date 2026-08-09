//! 对应 Java `me.chanjar.weixin.channel.bean.complaint.ComplaintHistory.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintHistory {
    #[serde(rename = "item_type", default)]
    pub item_type: i32,
    #[serde(rename = "time", default)]
    pub time: i64,
    #[serde(rename = "phone_number", default)]
    pub phone_number: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "media_id_list", default)]
    pub media_ids: Vec<String>,
    #[serde(rename = "after_sale_type", default)]
    pub after_sale_type: i32,
    #[serde(rename = "after_sale_reason", default)]
    pub after_sale_reason: i32,
}
