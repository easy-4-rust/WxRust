//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderCompensationDeliveryParam.java`。

use super::DeliveryInfo;

/// 订单补发货请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderCompensationDeliveryParam {
    /// 订单 ID。
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 物流信息列表。
    #[serde(rename = "delivery_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_list: Option<Vec<DeliveryInfo>>,
}
