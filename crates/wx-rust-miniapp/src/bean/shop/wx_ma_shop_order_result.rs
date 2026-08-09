//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopOrderResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopOrderResult {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "out_user_id", default)]
    pub out_user_id: String,
    #[serde(rename = "order_detail", default)]
    pub order_detail: WxMaShopOrderDetail,
}
