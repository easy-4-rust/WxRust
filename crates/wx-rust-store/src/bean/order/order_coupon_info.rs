//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderCouponInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderCouponInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderCouponInfo.java。
pub struct OrderCouponInfo {
    #[serde(rename = "user_coupon_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_coupon_id: Option<String>,
    #[serde(rename = "coupon_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_type: Option<i32>,
    #[serde(rename = "discounted_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounted_price: Option<i32>,
    #[serde(rename = "coupon_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_id: Option<String>,
}
