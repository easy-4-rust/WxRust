//! 对应 Java `com.binarywang.wxjava.store.bean.window.request.GetWindowProductListRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 GetWindowProductListRequest；对应 Java com.binarywang.wxjava.store.bean.window.request.GetWindowProductListRequest.java。
pub struct GetWindowProductListRequest {
    #[serde(rename = "appid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(rename = "branch_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<i32>,
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "page_index", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i32>,
    #[serde(rename = "last_buffer", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_buffer: Option<String>,
    #[serde(rename = "need_total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_total_num: Option<i32>,
}
