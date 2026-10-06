//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingNewParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ExternalProductMappingNewParam；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingNewParam.java。
pub struct ExternalProductMappingNewParam {
    /// 外部商品 ID
    #[serde(rename = "out_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_product_id: Option<String>,
    /// 上游字段 cat_id。
    #[serde(rename = "cat_id", skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<i64>,
    /// 上游字段 external_category_name。
    #[serde(
        rename = "external_category_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_category_name: Option<String>,
    /// 上游字段 head_imgs。
    #[serde(rename = "head_imgs", skip_serializing_if = "Option::is_none")]
    pub head_imgs: Option<Vec<String>>,
    /// 上游字段 detail_imgs。
    #[serde(
        rename = "detail_imgs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detail_imgs: Option<Vec<String>>,
    /// 上游字段 title。
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 上游字段 external_attributes。
    #[serde(
        rename = "external_attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_attributes: Option<Vec<ExternalProductMappingNewParamExternalAttribute>>,
}

/// 微信小店嵌套数据。对应 Java: product.ExternalProductMappingNewParam#ExternalAttribute
/// 微信小店 ExternalProductMappingNewParamExternalAttribute 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingNewParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProductMappingNewParamExternalAttribute {
    /// 上游字段 key。
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 上游字段 value。
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
