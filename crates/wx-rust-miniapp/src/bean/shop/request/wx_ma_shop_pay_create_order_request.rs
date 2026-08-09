//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopPayCreateOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopPayCreateOrderRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "combine_trade_no", default)]
    pub combine_trade_no: String,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
    #[serde(rename = "sub_orders", default)]
    pub sub_orders: Vec<SubOrdersDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrdersDTO {
    #[serde(rename = "mchid", default)]
    pub mchid: String,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "trade_no", default)]
    pub trade_no: String,
    #[serde(rename = "description", default)]
    pub description: String,
}
