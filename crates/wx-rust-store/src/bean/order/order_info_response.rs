//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderInfoResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::OrderInfo;

/// 微信小店 OrderInfoResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderInfoResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderInfoResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "order", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderInfo>,
}
