//! 对应 Java `me.chanjar.weixin.channel.bean.limit.LimitTaskUpdateParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitTaskUpdateParam {
    /// 限时抢购任务 ID
    #[serde(rename = "task_id", default)]
    pub task_id: String,
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// 开始时间
    #[serde(rename = "start_time", default)]
    pub start_time: String,
    /// 结束时间
    #[serde(rename = "end_time", default)]
    pub end_time: String,
    /// 限时抢购 SKU 信息
    #[serde(rename = "limited_discount_skus", default)]
    pub skus: Vec<LimitSku>,
}
