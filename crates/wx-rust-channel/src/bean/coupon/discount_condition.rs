//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.DiscountCondition.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountCondition {
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: i32,
    #[serde(rename = "product_price", default)]
    pub product_price: i32,
    #[serde(rename = "product_ids", default)]
    pub product_ids: Vec<String>,
}
