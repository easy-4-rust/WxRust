//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopAfterSaleListRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAfterSaleListRequest {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "begin_create_time", default)]
    pub begin_create_time: i64,
    #[serde(rename = "end_create_time", default)]
    pub end_create_time: i64,
    #[serde(rename = "offset", default)]
    pub offset: i64,
    #[serde(rename = "limit", default)]
    pub limit: i32,
}
