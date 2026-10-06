//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.ProductListResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ProductListResponse；对应 Java com.binarywang.wxjava.store.bean.supplier.ProductListResponse.java。
pub struct ProductListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 商品列表
    #[serde(rename = "product_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_list: Option<Vec<ProductSupplierInfo>>,
    /// 上游字段 next_key。
    #[serde(rename = "next_key", skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 has_more。
    #[serde(rename = "has_more", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 微信小店 ProductSupplierInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.supplier.ProductListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductSupplierInfo {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_id: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: supplier.ProductListResponse#ProductInfo
/// 微信小店 ProductListResponseProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.supplier.ProductListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductListResponseProductInfo {
    /// 上游字段 product_id。
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
    /// 上游字段 supplier_id。
    #[serde(
        rename = "supplier_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplier_id: Option<String>,
}
