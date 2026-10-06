//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.DiscountCondition.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DiscountCondition；对应 Java com.binarywang.wxjava.store.bean.coupon.DiscountCondition.java。
pub struct DiscountCondition {
    #[serde(rename = "product_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_cnt: Option<i32>,
    #[serde(rename = "product_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_price: Option<i32>,
    #[serde(rename = "product_ids", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
}
