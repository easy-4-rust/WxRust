//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaCancelOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCancelOrderResponse {
    #[serde(rename = "wxOrderId", default)]
    pub wx_order_id: String,
    #[serde(rename = "storeOrderId", default)]
    pub store_order_id: String,
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "orderStatus", default)]
    pub order_status: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "deductfee", default)]
    pub deductfee: i32,
}
