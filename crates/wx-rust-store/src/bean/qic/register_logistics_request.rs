//! 对应 Java `com.binarywang.wxjava.store.bean.qic.RegisterLogisticsRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 RegisterLogisticsRequest；对应 Java com.binarywang.wxjava.store.bean.qic.RegisterLogisticsRequest.java。
pub struct RegisterLogisticsRequest {
    /// 订单号
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// 快递单号
    #[serde(rename = "waybill_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waybill_id: Option<String>,
    /// 上游字段 order_id_list。
    #[serde(
        rename = "order_id_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_id_list: Option<Vec<String>>,
    /// 上游字段 logistics_info。
    #[serde(
        rename = "logistics_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logistics_info: Option<RegisterLogisticsRequestLogisticsInfo>,
}

/// 微信小店嵌套数据。对应 Java: qic.RegisterLogisticsRequest#LogisticsInfo
/// 微信小店 RegisterLogisticsRequestLogisticsInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.RegisterLogisticsRequest.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisterLogisticsRequestLogisticsInfo {
    /// 上游字段 waybill_id。
    #[serde(
        rename = "waybill_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub waybill_id: Option<String>,
    /// 上游字段 delivery_id。
    #[serde(
        rename = "delivery_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_id: Option<String>,
    /// 上游字段 delivery_name。
    #[serde(
        rename = "delivery_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_name: Option<String>,
    /// 上游字段 delivery_type。
    #[serde(
        rename = "delivery_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_type: Option<i32>,
}
