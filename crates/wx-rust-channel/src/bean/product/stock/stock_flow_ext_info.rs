//! 对应 Java `me.chanjar.weixin.channel.bean.product.stock.StockFlowExtInfo.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockFlowExtInfo {
    /// 扩展信息
    #[serde(rename = "ext_info", default)]
    pub ext_info: String,
}
