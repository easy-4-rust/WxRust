//! 对应 Java `com.binarywang.wxjava.store.bean.complaint.ComplaintOrderResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::ComplaintHistory;

/// 微信小店 ComplaintOrderResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.complaint.ComplaintOrderResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintOrderResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "after_sale_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_order_id: Option<String>,
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "history", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<ComplaintHistory>>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
