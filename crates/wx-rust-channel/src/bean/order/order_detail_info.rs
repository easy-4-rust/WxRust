//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderDetailInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDetailInfo {
    #[serde(rename = "product_infos", default)]
    pub product_infos: Vec<OrderProductInfo>,
    #[serde(rename = "pay_info", default)]
    pub pay_info: OrderPayInfo,
    #[serde(rename = "price_info", default)]
    pub price_info: OrderPriceInfo,
    #[serde(rename = "delivery_info", default)]
    pub delivery_info: OrderDeliveryInfo,
    #[serde(rename = "coupon_info", default)]
    pub coupon_info: OrderCouponInfo,
    #[serde(rename = "ext_info", default)]
    pub ext_info: OrderExtInfo,
    #[serde(rename = "commission_infos", default)]
    pub commission_infos: Vec<OrderCommissionInfo>,
    #[serde(rename = "sharer_info", default)]
    pub sharer_info: OrderSharerInfo,
    #[serde(rename = "settle_info", default)]
    pub settle_info: OrderSettleInfo,
    #[serde(rename = "sku_sharer_infos", default)]
    pub sku_sharer_infos: Vec<OrderSkuShareInfo>,
    #[serde(rename = "agent_info", default)]
    pub agent_info: OrderAgentInfo,
    #[serde(rename = "source_infos", default)]
    pub source_infos: Vec<OrderSourceInfo>,
    #[serde(rename = "refund_info", default)]
    pub refund_info: OrderSourceInfo,
    #[serde(rename = "greeting_card_info", default)]
    pub greeting_card_info: OrderGreetingCardInfo,
    #[serde(rename = "custom_info", default)]
    pub custom_info: OrderCustomInfo,
}
