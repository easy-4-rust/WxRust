//! 对应 Java `me.chanjar.weixin.channel.bean.product.stock.StockFlowParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockFlowParam {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// SKU ID
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    /// 开始时间
    #[serde(rename = "start_time", default)]
    pub start_time: String,
    /// 结束时间
    #[serde(rename = "end_time", default)]
    pub end_time: String,
    /// 每页数量
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
