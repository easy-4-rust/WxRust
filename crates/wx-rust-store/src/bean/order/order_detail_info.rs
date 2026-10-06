//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderDetailInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{
    OrderAgentInfo, OrderCommissionInfo, OrderCouponInfo, OrderCustomInfo, OrderDeliveryInfo,
    OrderExtInfo, OrderGreetingCardInfo, OrderPayInfo, OrderPriceInfo, OrderProductInfo,
    OrderSettleInfo, OrderSharerInfo, OrderSkuShareInfo, OrderSourceInfo,
};

/// 微信小店 OrderDetailInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderDetailInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDetailInfo {
    #[serde(rename = "product_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_infos: Option<Vec<OrderProductInfo>>,
    #[serde(rename = "pay_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_info: Option<OrderPayInfo>,
    #[serde(rename = "price_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_info: Option<OrderPriceInfo>,
    #[serde(rename = "delivery_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_info: Option<OrderDeliveryInfo>,
    #[serde(rename = "coupon_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_info: Option<OrderCouponInfo>,
    #[serde(rename = "ext_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<OrderExtInfo>,
    #[serde(rename = "commission_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_infos: Option<Vec<OrderCommissionInfo>>,
    #[serde(rename = "sharer_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_info: Option<OrderSharerInfo>,
    #[serde(rename = "settle_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<OrderSettleInfo>,
    #[serde(rename = "sku_sharer_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_sharer_infos: Option<Vec<OrderSkuShareInfo>>,
    #[serde(rename = "agent_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_info: Option<OrderAgentInfo>,
    #[serde(rename = "source_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_infos: Option<Vec<OrderSourceInfo>>,
    #[serde(rename = "refund_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_info: Option<OrderSourceInfo>,
    #[serde(rename = "greeting_card_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greeting_card_info: Option<OrderGreetingCardInfo>,
    #[serde(rename = "custom_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_info: Option<OrderCustomInfo>,
}
