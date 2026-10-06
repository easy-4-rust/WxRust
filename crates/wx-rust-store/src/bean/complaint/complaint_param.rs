//! 对应 Java `com.binarywang.wxjava.store.bean.complaint.ComplaintParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ComplaintParam；对应 Java com.binarywang.wxjava.store.bean.complaint.ComplaintParam.java。
pub struct ComplaintParam {
    #[serde(rename = "complaint_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint_id: Option<String>,
    #[serde(rename = "content", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "media_id_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
}
