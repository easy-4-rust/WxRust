//! 对应 Java `com.binarywang.wxjava.store.bean.after.GuaranteeOrderListParam.java`。

/// 保障单列表请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderListParam {
    /// 保障单号列表。
    #[serde(rename = "guarantee_order_id_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_id_list: Option<Vec<String>>,
    /// 订单号列表。
    #[serde(rename = "order_id_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_list: Option<Vec<String>>,
    /// 保障类型。
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_type: Option<i32>,
    /// 开始时间。
    #[serde(rename = "begin_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<i64>,
    /// 结束时间。
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 保障单状态列表（JSON 字符串）。
    #[serde(rename = "status_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_list: Option<String>,
    /// 分页偏移量。
    #[serde(rename = "offset", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页大小。
    #[serde(rename = "limit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
