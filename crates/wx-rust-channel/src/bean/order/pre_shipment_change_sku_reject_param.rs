//! 对应 Java `me.chanjar.weixin.channel.bean.order.PreShipmentChangeSkuRejectParam.java`。

#[allow(unused_imports)]
use super::*;

/// 拒绝待发货前更换 SKU 请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreShipmentChangeSkuRejectParam {
    /// 订单 ID。
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 拒绝原因。
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
}
