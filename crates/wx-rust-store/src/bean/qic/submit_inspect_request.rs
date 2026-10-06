//! 对应 Java `com.binarywang.wxjava.store.bean.qic.SubmitInspectRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SubmitInspectRequest；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitInspectRequest.java。
pub struct SubmitInspectRequest {
    /// 订单号
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 质检码
    #[serde(rename = "inspect_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspect_code: Option<String>,
    /// 上游字段 inspect_info。
    #[serde(
        rename = "inspect_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_info: Option<SubmitInspectRequestInspectInfo>,
}

/// 微信小店嵌套数据。对应 Java: qic.SubmitInspectRequest#InspectInfo
/// 微信小店 SubmitInspectRequestInspectInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitInspectRequest.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitInspectRequestInspectInfo {
    /// 上游字段 delivery_id。
    #[serde(
        rename = "delivery_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_id: Option<String>,
    /// 上游字段 backup_delivery_id。
    #[serde(
        rename = "backup_delivery_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_delivery_id: Option<String>,
    /// 上游字段 express_insure。
    #[serde(
        rename = "express_insure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub express_insure: Option<bool>,
    /// 上游字段 express_insure_amount。
    #[serde(
        rename = "express_insure_amount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub express_insure_amount: Option<i64>,
    /// 上游字段 express_merge。
    #[serde(
        rename = "express_merge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub express_merge: Option<bool>,
    /// 上游字段 inspect_org_id。
    #[serde(
        rename = "inspect_org_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_org_id: Option<String>,
    /// 上游字段 refund_intercept。
    #[serde(
        rename = "refund_intercept",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refund_intercept: Option<i32>,
    /// 上游字段 inspect_org_name。
    #[serde(
        rename = "inspect_org_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_org_name: Option<String>,
    /// 上游字段 warehouse_name。
    #[serde(
        rename = "warehouse_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warehouse_name: Option<String>,
    /// 上游字段 warehouse_addr。
    #[serde(
        rename = "warehouse_addr",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warehouse_addr: Option<String>,
    /// 上游字段 delivery_product_id。
    #[serde(
        rename = "delivery_product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_product_id: Option<i64>,
    /// 上游字段 delivery_insure_id。
    #[serde(
        rename = "delivery_insure_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_insure_id: Option<String>,
    /// 上游字段 backup_delivery_product_id。
    #[serde(
        rename = "backup_delivery_product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_delivery_product_id: Option<i64>,
    /// 上游字段 backup_delivery_insure_id。
    #[serde(
        rename = "backup_delivery_insure_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_delivery_insure_id: Option<String>,
    /// 上游字段 backup_express_insure。
    #[serde(
        rename = "backup_express_insure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_express_insure: Option<bool>,
    /// 上游字段 backup_express_insure_amount。
    #[serde(
        rename = "backup_express_insure_amount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_express_insure_amount: Option<i64>,
    /// 上游字段 remark。
    #[serde(rename = "remark", skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 上游字段 agarwood_inspect_org_id。
    #[serde(
        rename = "agarwood_inspect_org_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agarwood_inspect_org_id: Option<String>,
    /// 上游字段 agarwood_inspect_org_name。
    #[serde(
        rename = "agarwood_inspect_org_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agarwood_inspect_org_name: Option<String>,
}
