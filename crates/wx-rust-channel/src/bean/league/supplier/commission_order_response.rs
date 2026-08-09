//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.CommissionOrderResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommissionOrderResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "commssion_order", default)]
    pub commission_order: CommissionOrder,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommissionOrder {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "order_detail", default)]
    pub order_detail: OrderDetail,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDetail {
    #[serde(rename = "shop_info", default)]
    pub shop_info: BizInfo,
    #[serde(rename = "product_info", default)]
    pub product_info: ProductInfo,
    #[serde(rename = "order_info", default)]
    pub order_info: OrderInfo,
    #[serde(rename = "commission_info", default)]
    pub commission_info: CommissionInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BizInfo {
    #[serde(rename = "appid", default)]
    pub appid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "thumb_img", default)]
    pub thumb_img: String,
    #[serde(rename = "actual_payment", default)]
    pub actual_payment: i32,
    #[serde(rename = "title", default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {
    #[serde(rename = "order_status", default)]
    pub status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommissionInfo {
    #[serde(rename = "finder_info", default)]
    pub finder_info: FinderInfo,
    #[serde(rename = "service_ratio", default)]
    pub service_ratio: i32,
    #[serde(rename = "service_amount", default)]
    pub service_amount: i32,
    #[serde(rename = "profit_sharding_suc_time", default)]
    pub profit_sharding_suc_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderInfo {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "ratio", default)]
    pub ratio: i32,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "openfinderid", default)]
    pub openfinderid: String,
}
