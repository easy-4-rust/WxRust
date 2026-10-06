//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentOrderListParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TalentOrderListParam；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderListParam.java。
pub struct TalentOrderListParam {
    /// 每页数量
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 create_time_gt。
    #[serde(
        rename = "create_time_gt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time_gt: Option<i64>,
    /// 上游字段 create_time_lt。
    #[serde(
        rename = "create_time_lt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time_lt: Option<i64>,
    /// 上游字段 order_id。
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 spu_id。
    #[serde(rename = "spu_id", skip_serializing_if = "Option::is_none")]
    pub spu_id: Option<String>,
    /// 上游字段 update_time_gt。
    #[serde(
        rename = "update_time_gt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time_gt: Option<i64>,
    /// 上游字段 update_time_lt。
    #[serde(
        rename = "update_time_lt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time_lt: Option<i64>,
}
