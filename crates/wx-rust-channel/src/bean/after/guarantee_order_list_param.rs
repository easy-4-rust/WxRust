//! 对应 Java `me.chanjar.weixin.channel.bean.after.GuaranteeOrderListParam.java`。

#[allow(unused_imports)]
use super::*;

/// 保障单列表请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderListParam {
    /// 保障单号列表。
    #[serde(rename = "guarantee_order_id_list", default)]
    pub guarantee_order_id_list: Vec<String>,
    /// 订单号列表。
    #[serde(rename = "order_id_list", default)]
    pub order_id_list: Vec<String>,
    /// 保障类型。
    #[serde(rename = "type", default)]
    pub guarantee_type: i32,
    /// 开始时间。
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    /// 结束时间。
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    /// 保障单状态列表（JSON 字符串）。
    #[serde(rename = "status_list", default)]
    pub status_list: String,
    /// 分页偏移量。
    #[serde(rename = "offset", default)]
    pub offset: i32,
    /// 分页大小。
    #[serde(rename = "limit", default)]
    pub limit: i32,
}
