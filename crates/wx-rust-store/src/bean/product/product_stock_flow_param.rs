/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.product.ProductStockFlowParam
/// 微信小店 ProductStockFlowParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.product.ProductStockFlowParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductStockFlowParam {
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(
        rename = "stock_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_type: Option<i32>,
    #[serde(rename = "finder_id", skip_serializing_if = "Option::is_none")]
    pub finder_id: Option<String>,
    #[serde(
        rename = "begin_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub begin_time: Option<i64>,
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(
        rename = "op_type_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub op_type_list: Option<Vec<i32>>,
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "next_key", skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(
        rename = "stock_type_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_type_id: Option<String>,
}
