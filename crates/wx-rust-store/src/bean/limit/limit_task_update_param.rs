//! 对应 Java `com.binarywang.wxjava.store.bean.limit.LimitTaskUpdateParam.java`。

use super::LimitSkuUpdate;

/// 微信小店 LimitTaskUpdateParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.limit.LimitTaskUpdateParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitTaskUpdateParam {
    /// 限时抢购任务 ID
    #[serde(rename = "task_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 开始时间
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 限时抢购 SKU 信息
    #[serde(rename = "limited_discount_skus", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skus: Option<Vec<LimitSkuUpdate>>,
    /// 上游字段 status。
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 上游字段 title。
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
