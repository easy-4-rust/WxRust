//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.ProductDistributeRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ProductDistributeRequest；对应 Java com.binarywang.wxjava.store.bean.supplier.ProductDistributeRequest.java。
pub struct ProductDistributeRequest {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_id: Option<String>,
    /// 上游字段 product_id_list。
    #[serde(
        rename = "product_id_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id_list: Option<Vec<String>>,
}
