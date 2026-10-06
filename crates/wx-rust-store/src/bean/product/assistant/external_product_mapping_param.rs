//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ExternalProductMappingParam；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingParam.java。
pub struct ExternalProductMappingParam {
    /// 外部商品 ID
    #[serde(rename = "out_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_product_id: Option<String>,
    /// 上游字段 cat_id。
    #[serde(rename = "cat_id", skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<i64>,
    /// 上游字段 external_attribute_name。
    #[serde(
        rename = "external_attribute_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_attribute_name: Option<String>,
    /// 上游字段 external_attribute_value。
    #[serde(
        rename = "external_attribute_value",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_attribute_value: Option<String>,
    /// 上游字段 external_category_name。
    #[serde(
        rename = "external_category_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_category_name: Option<String>,
}
