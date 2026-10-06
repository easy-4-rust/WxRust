//! 对应 Java `com.binarywang.wxjava.store.bean.product.stock.StockFlowParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 StockFlowParam；对应 Java com.binarywang.wxjava.store.bean.product.stock.StockFlowParam.java。
pub struct StockFlowParam {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// SKU ID
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    /// 开始时间
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 每页数量
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 stock_type。
    #[serde(
        rename = "stock_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_type: Option<i32>,
    /// 上游字段 finder_id。
    #[serde(rename = "finder_id", skip_serializing_if = "Option::is_none")]
    pub finder_id: Option<String>,
    /// 上游字段 begin_time。
    #[serde(
        rename = "begin_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub begin_time: Option<i64>,
    /// 上游字段 op_type_list。
    #[serde(
        rename = "op_type_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub op_type_list: Option<Vec<i32>>,
    /// 上游字段 stock_type_id。
    #[serde(
        rename = "stock_type_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_type_id: Option<String>,
}
