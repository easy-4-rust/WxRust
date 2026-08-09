//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.MockUpdateOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MockUpdateOrderRequest {
    #[serde(rename = "shopid", default)]
    pub shop_id: String,
    #[serde(rename = "shop_order_id", default)]
    pub shop_order_id: String,
    #[serde(rename = "action_time", default)]
    pub action_time: i64,
    #[serde(rename = "order_status", default)]
    pub order_status: i32,
}
