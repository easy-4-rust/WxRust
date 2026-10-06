//! 对应 Java `com.binarywang.wxjava.store.bean.product.stock.StockFlowResponse.java`。

#[allow(unused_imports)]
use super::stock_flow_info::StockFlowInfo;

/// 微信小店 StockFlowResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.stock.StockFlowResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(from = "StockFlowWire")]
pub struct StockFlowResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 库存流水列表
    #[serde(rename = "flow_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_list: Option<Vec<StockFlowInfo>>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 stockFlowInfoList。
    #[serde(
        rename = "stockFlowInfoList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_flow_info_list: Option<Vec<StockFlowInfo>>,
}

/// 上游 data 包装的接收模型。对应 Java: StockFlowResponse#unpackData。
#[derive(serde::Deserialize)]
struct StockFlowWire {
    #[serde(default, rename = "errcode")]
    err_code: i32,
    #[serde(default, rename = "errmsg")]
    err_msg: String,
    #[serde(default)]
    data: Option<StockFlowResponseStockFlowData>,
    #[serde(default)]
    flow_list: Vec<StockFlowInfo>,
    #[serde(default)]
    next_key: String,
    #[serde(default, rename = "stockFlowInfoList")]
    stock_flow_info_list: Option<Vec<StockFlowInfo>>,
}

impl From<StockFlowWire> for StockFlowResponse {
    fn from(wire: StockFlowWire) -> Self {
        let (next_key, items) = match wire.data {
            Some(data) => (data.next_key.unwrap_or_default(), data.stock_flow_info_list),
            None => (wire.next_key, wire.stock_flow_info_list),
        };
        Self {
            err_code: wire.err_code,
            err_msg: wire.err_msg,
            flow_list: Some(items.clone().unwrap_or(wire.flow_list)),
            next_key: Some(next_key),
            stock_flow_info_list: items,
        }
    }
}

/// 微信小店嵌套数据。对应 Java: product.stock.StockFlowResponse#StockFlowData
/// 微信小店 StockFlowResponseStockFlowData 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.stock.StockFlowResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockFlowResponseStockFlowData {
    /// 上游字段 next_key。
    #[serde(rename = "next_key", skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 stock_flow_info_list。
    #[serde(
        rename = "stock_flow_info_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_flow_info_list: Option<Vec<StockFlowInfo>>,
}
