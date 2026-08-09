//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopSharerLiveOrderListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSharerLiveOrderListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "orders", default)]
    pub orders: Vec<WxMaShopOrderItem>,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopOrderItem {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "order_detail", default)]
    pub order_detail: WxMaShopOrderDetail,
}
