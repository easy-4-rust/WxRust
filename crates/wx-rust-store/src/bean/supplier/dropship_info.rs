//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.DropshipInfo.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DropshipInfo；对应 Java com.binarywang.wxjava.store.bean.supplier.DropshipInfo.java。
pub struct DropshipInfo {
    /// 订单号
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_id: Option<String>,
    /// 代发单状态
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 上游字段 ds_order_id。
    #[serde(
        rename = "ds_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dropship_id: Option<String>,
    /// 上游字段 create_time。
    #[serde(
        rename = "create_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// 上游字段 update_time。
    #[serde(
        rename = "update_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
}
