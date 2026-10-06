//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderPriceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderPriceInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderPriceInfo.java。
pub struct OrderPriceInfo {
    #[serde(rename = "product_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_price: Option<i32>,
    #[serde(rename = "order_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_price: Option<i32>,
    #[serde(rename = "freight", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight: Option<i32>,
    #[serde(rename = "discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounted_price: Option<i32>,
    #[serde(rename = "is_discounted", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_discounted: Option<bool>,
    #[serde(rename = "original_order_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_order_price: Option<i32>,
    #[serde(rename = "estimate_product_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_product_price: Option<i32>,
    #[serde(rename = "change_down_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_down_price: Option<i32>,
    #[serde(rename = "change_freight", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_freight: Option<i32>,
    #[serde(rename = "is_change_freight", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_freighted: Option<bool>,
    #[serde(rename = "use_deduction", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_deduction: Option<bool>,
    #[serde(rename = "deduction_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduction_price: Option<i32>,
    #[serde(rename = "merchant_receieve_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_receive_price: Option<i32>,
    #[serde(rename = "merchant_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_discounted_price: Option<i32>,
    #[serde(rename = "finder_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_discounted_price: Option<i32>,
    #[serde(rename = "vip_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_discounted_price: Option<i32>,
    #[serde(rename = "bulkbuy_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulkbuy_discounted_price: Option<i32>,
    #[serde(rename = "national_subsidy_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_subsidy_discounted_price: Option<i32>,
    #[serde(rename = "cash_coupon_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_coupon_discounted_price: Option<i32>,
    #[serde(rename = "national_subsidy_merchant_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_subsidy_merchant_discounted_price: Option<i32>,
}
