//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.SupplierInfoResponse.java`。

#[allow(unused_imports)]
use super::supplier_info::SupplierInfo;

/// 微信小店 SupplierInfoResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.supplier.SupplierInfoResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupplierInfoResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 供货商信息
    #[serde(rename = "supplier_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_info: Option<SupplierInfo>,
}
