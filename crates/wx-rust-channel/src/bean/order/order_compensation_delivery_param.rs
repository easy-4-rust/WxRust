//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderCompensationDeliveryParam.java`。

#[allow(unused_imports)]
use super::*;

/// 订单补发货请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderCompensationDeliveryParam {
    /// 订单 ID。
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 物流信息列表。
    #[serde(rename = "delivery_list", default)]
    pub delivery_list: Vec<DeliveryInfo>,
}
