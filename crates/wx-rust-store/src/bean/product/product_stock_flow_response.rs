/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.product.ProductStockFlowResponse
/// 微信小店 ProductStockFlowResponse 数据类型；对应 Java: com.binarywang.wxjava.store.bean.product.ProductStockFlowResponse。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductStockFlowResponse {
    #[serde(rename = "errcode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<i32>,
    #[serde(rename = "errmsg", skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<StockFlowData>,
}

/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.product.ProductStockFlowResponse
/// 微信小店 StockFlowData 数据类型；对应 Java: com.binarywang.wxjava.store.bean.product.ProductStockFlowResponse。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockFlowData {
    #[serde(
        rename = "stock_flow_info_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_flow_info_list: Option<Vec<serde_json::Value>>,
    #[serde(rename = "next_key", skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
}
