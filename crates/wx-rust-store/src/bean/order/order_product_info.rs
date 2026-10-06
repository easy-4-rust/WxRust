//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{
    ChangeSkuInfo, DropshipInfo, FreeGiftInfo, OrderCouponInfo, OrderProductExtraService,
    OrderSkuDeliverInfo,
};

#[allow(unused_imports)]
use crate::bean::base::AttrInfo;

/// 微信小店 OrderProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderProductInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderProductInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "thumb_img", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_img: Option<String>,
    #[serde(rename = "sku_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_cnt: Option<i32>,
    #[serde(rename = "sale_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<i32>,
    #[serde(rename = "title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "on_aftersale_sku_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_after_sale_sku_cnt: Option<i32>,
    #[serde(rename = "finish_aftersale_sku_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_after_sale_sku_cnt: Option<i32>,
    #[serde(rename = "sku_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_code: Option<String>,
    #[serde(rename = "market_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_price: Option<i32>,
    #[serde(rename = "sku_attrs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_attrs: Option<Vec<AttrInfo>>,
    #[serde(rename = "real_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_price: Option<i32>,
    #[serde(rename = "out_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_product_id: Option<String>,
    #[serde(rename = "out_sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_sku_id: Option<String>,
    #[serde(rename = "is_discounted", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_discounted: Option<bool>,
    #[serde(rename = "estimate_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_price: Option<i32>,
    #[serde(rename = "is_change_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_priced: Option<bool>,
    #[serde(rename = "change_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_price: Option<i32>,
    #[serde(rename = "out_warehouse_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_warehouse_id: Option<String>,
    #[serde(rename = "sku_deliver_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_deliver_info: Option<OrderSkuDeliverInfo>,
    #[serde(rename = "extra_service", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_service: Option<OrderProductExtraService>,
    #[serde(rename = "use_deduction", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_deduction: Option<bool>,
    #[serde(rename = "deduction_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduction_price: Option<i32>,
    #[serde(rename = "order_product_coupon_info_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_product_coupon_info_list: Option<Vec<OrderCouponInfo>>,
    #[serde(rename = "delivery_deadline", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_deadline: Option<i64>,
    #[serde(rename = "merchant_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_discounted_price: Option<i32>,
    #[serde(rename = "finder_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_discounted_price: Option<i32>,
    #[serde(rename = "is_free_gift", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_gift: Option<bool>,
    #[serde(rename = "vip_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_discounted_price: Option<i32>,
    #[serde(rename = "product_unique_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_unique_id: Option<String>,
    #[serde(rename = "change_sku_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_sku_info: Option<ChangeSkuInfo>,
    #[serde(rename = "free_gift_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_gift_info: Option<FreeGiftInfo>,
    #[serde(rename = "bulkbuy_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulkbuy_discounted_price: Option<i32>,
    #[serde(rename = "national_subsidy_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_subsidy_discounted_price: Option<i32>,
    #[serde(rename = "dropship_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropship_info: Option<DropshipInfo>,
    #[serde(rename = "is_flash_sale", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_sale: Option<bool>,
    #[serde(rename = "national_subsidy_merchant_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_subsidy_merchant_discounted_price: Option<i32>,
    #[serde(rename = "platform_activity_merchant_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_activity_merchant_discounted_price: Option<i32>,
    #[serde(rename = "cash_coupon_discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_coupon_discounted_price: Option<i32>,
}
