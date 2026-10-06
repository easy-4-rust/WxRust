//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.DropshipSearchRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DropshipSearchRequest；对应 Java com.binarywang.wxjava.store.bean.supplier.DropshipSearchRequest.java。
pub struct DropshipSearchRequest {
    /// 每页数量
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 搜索关键词
    #[serde(rename = "keyword", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// 上游字段 supplier_id。
    #[serde(
        rename = "supplier_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplier_id: Option<String>,
    /// 上游字段 status。
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 上游字段 create_time_start。
    #[serde(
        rename = "create_time_start",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time_start: Option<i64>,
    /// 上游字段 create_time_end。
    #[serde(
        rename = "create_time_end",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time_end: Option<i64>,
    /// 上游字段 order_id。
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 dropship_id。
    #[serde(
        rename = "dropship_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dropship_id: Option<String>,
}
