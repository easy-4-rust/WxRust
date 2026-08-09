//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.shipping.WxMaOrderShippingInfoGetListRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::request::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOrderShippingInfoGetListRequest {
    #[serde(rename = "pay_time_range", default)]
    pub pay_time_range: PayTimeRange,
    #[serde(rename = "order_state", default)]
    pub order_state: i32,
    #[serde(rename = "openid", default)]
    pub open_id: String,
    #[serde(rename = "last_index", default)]
    pub last_index: String,
    #[serde(rename = "page_size", default)]
    pub page_size: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayTimeRange {
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
}
