//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ExternalProductMappingResponse；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingResponse.java。
pub struct ExternalProductMappingResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 映射结果
    #[serde(rename = "mapping_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_list: Option<Vec<ExternalMappingInfo>>,
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
    /// 上游字段 internal_attribute_name。
    #[serde(
        rename = "internal_attribute_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_attribute_name: Option<String>,
    /// 上游字段 internal_attribute_value。
    #[serde(
        rename = "internal_attribute_value",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_attribute_value: Option<Vec<String>>,
}

/// 微信小店 ExternalMappingInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalMappingInfo {
    /// 外部属性 ID
    #[serde(rename = "out_attr_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_attr_id: Option<String>,
    /// 属性 ID
    #[serde(rename = "attr_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_id: Option<String>,
}
