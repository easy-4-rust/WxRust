//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.ProductListResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 商品列表
    #[serde(rename = "product_list", default)]
    pub product_list: Vec<ProductSupplierInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductSupplierInfo {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    pub supplier_id: String,
}
