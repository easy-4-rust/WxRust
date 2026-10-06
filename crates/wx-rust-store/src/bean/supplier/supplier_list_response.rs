//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.SupplierListResponse.java`。

#[allow(unused_imports)]
use super::supplier_info::SupplierInfo;

/// 微信小店 SupplierListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.supplier.SupplierListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupplierListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 供货商列表
    #[serde(rename = "supplier_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_list: Option<Vec<SupplierInfo>>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 has_more。
    #[serde(rename = "has_more", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
