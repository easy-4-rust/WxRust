//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.SupplierInfo.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupplierInfo {
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    pub supplier_id: String,
    /// 供货商名称
    #[serde(rename = "supplier_name", default)]
    pub supplier_name: String,
}
