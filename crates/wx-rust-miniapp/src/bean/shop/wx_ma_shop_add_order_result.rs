//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopAddOrderResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAddOrderResult {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "ticket", default)]
    pub ticket: String,
    #[serde(rename = "ticket_expire_time", default)]
    pub ticket_expire_time: String,
    #[serde(rename = "final_price", default)]
    pub final_price: i32,
}
