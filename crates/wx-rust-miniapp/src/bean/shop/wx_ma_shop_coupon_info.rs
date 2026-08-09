//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopCouponInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopCouponInfo {
    #[serde(rename = "out_coupon_id", default)]
    pub out_coupon_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "promote_type", default)]
    pub promote_type: i32,
    #[serde(rename = "coupon_info", default)]
    pub coupon_info: CouponInfo,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "coupon_stock", default)]
    pub coupon_stock: CouponStock,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "promote_info", default)]
    pub promote_info: PromoteInfo,
    #[serde(rename = "discount_info", default)]
    pub discount_info: DiscountInfo,
    #[serde(rename = "receive_info", default)]
    pub receive_info: ReceiveInfo,
    #[serde(rename = "valid_info", default)]
    pub valid_info: ValidInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromoteInfo {
    #[serde(rename = "promote_type", default)]
    pub promote_type: i32,
    #[serde(rename = "finder", default)]
    pub finder: PromoteFinder,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromoteFinder {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountInfo {
    #[serde(rename = "discount_num", default)]
    pub discount_num: i32,
    #[serde(rename = "discount_fee", default)]
    pub discount_fee: i64,
    #[serde(rename = "discount_condition", default)]
    pub discount_condition: DiscountCondition,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountCondition {
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: i32,
    #[serde(rename = "product_price", default)]
    pub product_price: i64,
    #[serde(rename = "out_product_ids", default)]
    pub out_product_ids: Vec<String>,
    #[serde(rename = "tradein_info", default)]
    pub tradein_info: TradeinInfo,
    #[serde(rename = "buyget_info", default)]
    pub buyget_info: BuygetInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeinInfo {
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "price", default)]
    pub price: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuygetInfo {
    #[serde(rename = "buy_out_product_id", default)]
    pub buy_out_product_id: String,
    #[serde(rename = "buy_product_cnt", default)]
    pub buy_product_cnt: i32,
    #[serde(rename = "get_out_product_id", default)]
    pub get_out_product_id: String,
    #[serde(rename = "get_product_cnt", default)]
    pub get_product_cnt: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReceiveInfo {
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "limit_num_one_person", default)]
    pub limit_num_one_person: i32,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ValidInfo {
    #[serde(rename = "valid_type", default)]
    pub valid_type: i32,
    #[serde(rename = "valid_day_num", default)]
    pub valid_day_num: i32,
    #[serde(rename = "valid_second", default)]
    pub valid_second: i64,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponStock {
    #[serde(rename = "out_coupon_id", default)]
    pub out_coupon_id: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "stock_info", default)]
    pub stock_info: StockInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockInfo {
    #[serde(rename = "issued_num", default)]
    pub issued_num: i32,
    #[serde(rename = "receive_num", default)]
    pub receive_num: i32,
}
