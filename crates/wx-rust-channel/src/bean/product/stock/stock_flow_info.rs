//! 对应 Java `me.chanjar.weixin.channel.bean.product.stock.StockFlowInfo.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockFlowInfo {
    /// 流水 ID
    #[serde(rename = "flow_id", default)]
    pub flow_id: String,
    /// 流水类型
    #[serde(rename = "flow_type", default)]
    pub flow_type: i32,
    /// 库存变化量
    #[serde(rename = "stock_num", default)]
    pub stock_num: i32,
    /// 创建时间
    #[serde(rename = "create_time", default)]
    pub create_time: String,
}
