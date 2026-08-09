//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderPriceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderPriceInfo {
    #[serde(rename = "product_price", default)]
    pub product_price: i32,
    #[serde(rename = "order_price", default)]
    pub order_price: i32,
    #[serde(rename = "freight", default)]
    pub freight: i32,
    #[serde(rename = "discounted_price", default)]
    pub discounted_price: i32,
    #[serde(rename = "is_discounted", default)]
    pub is_discounted: bool,
    #[serde(rename = "original_order_price", default)]
    pub original_order_price: i32,
    #[serde(rename = "estimate_product_price", default)]
    pub estimate_product_price: i32,
    #[serde(rename = "change_down_price", default)]
    pub change_down_price: i32,
    #[serde(rename = "change_freight", default)]
    pub change_freight: i32,
    #[serde(rename = "is_change_freight", default)]
    pub change_freighted: bool,
    #[serde(rename = "use_deduction", default)]
    pub use_deduction: bool,
    #[serde(rename = "deduction_price", default)]
    pub deduction_price: i32,
    #[serde(rename = "merchant_receieve_price", default)]
    pub merchant_receive_price: i32,
    #[serde(rename = "merchant_discounted_price", default)]
    pub merchant_discounted_price: i32,
    #[serde(rename = "finder_discounted_price", default)]
    pub finder_discounted_price: i32,
    #[serde(rename = "vip_discounted_price", default)]
    pub vip_discounted_price: i32,
    #[serde(rename = "bulkbuy_discounted_price", default)]
    pub bulkbuy_discounted_price: i32,
    #[serde(rename = "national_subsidy_discounted_price", default)]
    pub national_subsidy_discounted_price: i32,
    #[serde(rename = "cash_coupon_discounted_price", default)]
    pub cash_coupon_discounted_price: i32,
    #[serde(rename = "national_subsidy_merchant_discounted_price", default)]
    pub national_subsidy_merchant_discounted_price: i32,
}
