//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.SupplierInfo.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SupplierInfo；对应 Java com.binarywang.wxjava.store.bean.supplier.SupplierInfo.java。
pub struct SupplierInfo {
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_id: Option<String>,
    /// 供货商名称
    #[serde(rename = "supplier_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_name: Option<String>,
    /// 上游字段 status。
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
