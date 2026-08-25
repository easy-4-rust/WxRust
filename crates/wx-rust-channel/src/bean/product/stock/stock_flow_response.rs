//! 对应 Java `me.chanjar.weixin.channel.bean.product.stock.StockFlowResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

use super::stock_flow_info::StockFlowInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockFlowResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 库存流水列表
    #[serde(rename = "flow_list", default)]
    pub flow_list: Vec<StockFlowInfo>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
