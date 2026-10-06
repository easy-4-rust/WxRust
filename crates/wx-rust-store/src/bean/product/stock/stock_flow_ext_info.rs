//! 对应 Java `com.binarywang.wxjava.store.bean.product.stock.StockFlowExtInfo.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 StockFlowExtInfo；对应 Java com.binarywang.wxjava.store.bean.product.stock.StockFlowExtInfo.java。
pub struct StockFlowExtInfo {
    /// 扩展信息
    #[serde(rename = "ext_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<String>,
    /// 上游字段 unmove_from_stock_sub_type。
    #[serde(
        rename = "unmove_from_stock_sub_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unmove_from_stock_sub_type: Option<i32>,
    /// 上游字段 move_to_stock_sub_type。
    #[serde(
        rename = "move_to_stock_sub_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub move_to_stock_sub_type: Option<i32>,
    /// 上游字段 upload_source。
    #[serde(
        rename = "upload_source",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upload_source: Option<i32>,
    /// 上游字段 order_id。
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 out_warehouse_id。
    #[serde(
        rename = "out_warehouse_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub out_warehouse_id: Option<String>,
    /// 上游字段 limited_discount_id。
    #[serde(
        rename = "limited_discount_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limited_discount_id: Option<String>,
    /// 上游字段 finder_id。
    #[serde(rename = "finder_id", skip_serializing_if = "Option::is_none")]
    pub finder_id: Option<String>,
}
