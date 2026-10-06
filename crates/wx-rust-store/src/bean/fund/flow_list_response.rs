//! 对应 Java `com.binarywang.wxjava.store.bean.fund.FlowListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FlowListResponse；对应 Java com.binarywang.wxjava.store.bean.fund.FlowListResponse.java。
pub struct FlowListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "flow_ids", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_ids: Option<Vec<String>>,
    #[serde(rename = "has_more", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
}
