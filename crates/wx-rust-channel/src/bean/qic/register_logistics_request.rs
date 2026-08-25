//! 对应 Java `me.chanjar.weixin.channel.bean.qic.RegisterLogisticsRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisterLogisticsRequest {
    /// 订单号
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    /// 快递单号
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
}
