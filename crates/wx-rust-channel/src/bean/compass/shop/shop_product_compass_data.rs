//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.ShopProductCompassData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopProductCompassData {
    #[serde(rename = "pay_gmv", default)]
    pub pay_gmv: String,
    #[serde(rename = "create_gmv", default)]
    pub create_gmv: String,
    #[serde(rename = "create_cnt", default)]
    pub create_cnt: String,
    #[serde(rename = "create_uv", default)]
    pub create_uv: String,
    #[serde(rename = "create_product_cnt", default)]
    pub create_product_cnt: String,
    #[serde(rename = "pay_cnt", default)]
    pub pay_cnt: String,
    #[serde(rename = "pay_uv", default)]
    pub pay_uv: String,
    #[serde(rename = "pay_product_cnt", default)]
    pub pay_product_cnt: String,
    #[serde(rename = "pure_pay_gmv", default)]
    pub pure_pay_gmv: String,
    #[serde(rename = "pay_gmv_per_uv", default)]
    pub pay_gmv_per_uv: String,
    #[serde(rename = "seller_actual_settle_amount", default)]
    pub seller_actual_settle_amount: String,
    #[serde(rename = "platform_actual_commission", default)]
    pub platform_actual_commission: String,
    #[serde(rename = "finderuin_actual_commission", default)]
    pub finderuin_actual_commission: String,
    #[serde(rename = "captain_actual_commission", default)]
    pub captain_actual_commission: String,
    #[serde(rename = "seller_predict_settle_amount", default)]
    pub seller_predict_settle_amount: String,
    #[serde(rename = "platform_predict_commission", default)]
    pub platform_predict_commission: String,
    #[serde(rename = "finderuin_predict_commission", default)]
    pub finderuin_predict_commission: String,
    #[serde(rename = "captain_predict_commission", default)]
    pub captain_predict_commission: String,
    #[serde(rename = "product_click_uv", default)]
    pub product_click_uv: String,
    #[serde(rename = "product_click_cnt", default)]
    pub product_click_cnt: String,
    #[serde(rename = "pay_refund_gmv", default)]
    pub pay_refund_gmv: String,
    #[serde(rename = "pay_refund_uv", default)]
    pub pay_refund_uv: String,
    #[serde(rename = "pay_refund_ratio", default)]
    pub pay_refund_ratio: f64,
    #[serde(rename = "pay_refund_after_send_ratio", default)]
    pub pay_refund_after_send_ratio: f64,
    #[serde(rename = "pay_refund_cnt", default)]
    pub pay_refund_cnt: String,
    #[serde(rename = "pay_refund_product_cnt", default)]
    pub pay_refund_product_cnt: String,
    #[serde(rename = "pay_refund_before_send_ratio", default)]
    pub pay_refund_before_send_ratio: f64,
    #[serde(rename = "refund_gmv", default)]
    pub refund_gmv: String,
    #[serde(rename = "refund_product_cnt", default)]
    pub refund_product_cnt: String,
    #[serde(rename = "refund_cnt", default)]
    pub refund_cnt: String,
    #[serde(rename = "refund_uv", default)]
    pub refund_uv: String,
}
