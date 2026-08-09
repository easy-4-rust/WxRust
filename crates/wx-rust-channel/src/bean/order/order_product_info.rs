//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AttrInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderProductInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "thumb_img", default)]
    pub thumb_img: String,
    #[serde(rename = "sku_cnt", default)]
    pub sku_cnt: i32,
    #[serde(rename = "sale_price", default)]
    pub sale_price: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "on_aftersale_sku_cnt", default)]
    pub on_after_sale_sku_cnt: i32,
    #[serde(rename = "finish_aftersale_sku_cnt", default)]
    pub finish_after_sale_sku_cnt: i32,
    #[serde(rename = "sku_code", default)]
    pub sku_code: String,
    #[serde(rename = "market_price", default)]
    pub market_price: i32,
    #[serde(rename = "sku_attrs", default)]
    pub sku_attrs: Vec<AttrInfo>,
    #[serde(rename = "real_price", default)]
    pub real_price: i32,
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "is_discounted", default)]
    pub is_discounted: bool,
    #[serde(rename = "estimate_price", default)]
    pub estimate_price: i32,
    #[serde(rename = "is_change_price", default)]
    pub change_priced: bool,
    #[serde(rename = "change_price", default)]
    pub change_price: i32,
    #[serde(rename = "out_warehouse_id", default)]
    pub out_warehouse_id: String,
    #[serde(rename = "sku_deliver_info", default)]
    pub sku_deliver_info: OrderSkuDeliverInfo,
    #[serde(rename = "extra_service", default)]
    pub extra_service: OrderProductExtraService,
    #[serde(rename = "use_deduction", default)]
    pub use_deduction: bool,
    #[serde(rename = "deduction_price", default)]
    pub deduction_price: i32,
    #[serde(rename = "order_product_coupon_info_list", default)]
    pub order_product_coupon_info_list: Vec<OrderCouponInfo>,
    #[serde(rename = "delivery_deadline", default)]
    pub delivery_deadline: i64,
    #[serde(rename = "merchant_discounted_price", default)]
    pub merchant_discounted_price: i32,
    #[serde(rename = "finder_discounted_price", default)]
    pub finder_discounted_price: i32,
    #[serde(rename = "is_free_gift", default)]
    pub free_gift: bool,
    #[serde(rename = "vip_discounted_price", default)]
    pub vip_discounted_price: i32,
    #[serde(rename = "product_unique_id", default)]
    pub product_unique_id: String,
    #[serde(rename = "change_sku_info", default)]
    pub change_sku_info: ChangeSkuInfo,
    #[serde(rename = "free_gift_info", default)]
    pub free_gift_info: FreeGiftInfo,
    #[serde(rename = "bulkbuy_discounted_price", default)]
    pub bulkbuy_discounted_price: i32,
    #[serde(rename = "national_subsidy_discounted_price", default)]
    pub national_subsidy_discounted_price: i32,
    #[serde(rename = "dropship_info", default)]
    pub dropship_info: DropshipInfo,
    #[serde(rename = "is_flash_sale", default)]
    pub flash_sale: bool,
    #[serde(rename = "national_subsidy_merchant_discounted_price", default)]
    pub national_subsidy_merchant_discounted_price: i32,
    #[serde(rename = "platform_activity_merchant_discounted_price", default)]
    pub platform_activity_merchant_discounted_price: i32,
    #[serde(rename = "cash_coupon_discounted_price", default)]
    pub cash_coupon_discounted_price: i32,
}
