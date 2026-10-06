//! 对应 Java `com.binarywang.wxjava.store.bean.product.stock.StockFlowInfo.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 StockFlowInfo；对应 Java com.binarywang.wxjava.store.bean.product.stock.StockFlowInfo.java。
pub struct StockFlowInfo {
    /// 流水 ID
    #[serde(rename = "flow_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// 流水类型
    #[serde(rename = "flow_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<i32>,
    /// 库存变化量
    #[serde(rename = "stock_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_num: Option<i32>,
    /// 创建时间
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 上游字段 amount。
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// 上游字段 beginning_amount。
    #[serde(
        rename = "beginning_amount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub beginning_amount: Option<i32>,
    /// 上游字段 ending_amount。
    #[serde(
        rename = "ending_amount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ending_amount: Option<i32>,
    /// 上游字段 stock_sub_type。
    #[serde(
        rename = "stock_sub_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_sub_type: Option<i32>,
    /// 上游字段 op_type。
    #[serde(rename = "op_type", skip_serializing_if = "Option::is_none")]
    pub op_type: Option<i32>,
    /// 上游字段 update_time。
    #[serde(
        rename = "update_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// 上游字段 ext_info。
    #[serde(rename = "ext_info", skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<super::StockFlowExtInfo>,
}
