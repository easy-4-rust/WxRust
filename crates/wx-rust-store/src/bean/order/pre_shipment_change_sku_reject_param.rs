//! 对应 Java `com.binarywang.wxjava.store.bean.order.PreShipmentChangeSkuRejectParam.java`。

/// 拒绝待发货前更换 SKU 请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreShipmentChangeSkuRejectParam {
    /// 订单 ID。
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 拒绝原因。
    #[serde(rename = "reject_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
}
