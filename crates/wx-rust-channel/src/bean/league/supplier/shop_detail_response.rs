//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.ShopDetailResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopDetailResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "shop_detail", default)]
    pub shop_detail: ShopDetail,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopDetail {
    #[serde(rename = "base_info", default)]
    pub base_info: BizBaseInfo,
    #[serde(rename = "data_info", default)]
    pub data_info: ShopDataInfo,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "approved_time", default)]
    pub approved_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopDataInfo {
    #[serde(rename = "gmv", default)]
    pub gmv: i32,
    #[serde(rename = "product_number", default)]
    pub product_number: i32,
    #[serde(rename = "settle_amount", default)]
    pub settle_amount: i32,
    #[serde(rename = "unsettle_amount", default)]
    pub unsettle_amount: i32,
    #[serde(rename = "product_number_today", default)]
    pub product_number_today: i32,
    #[serde(rename = "product_number_sold_today", default)]
    pub product_number_sold_today: i32,
}
