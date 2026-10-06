//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingNewResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ExternalProductMappingNewResponse；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingNewResponse.java。
pub struct ExternalProductMappingNewResponse {
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
    /// 推荐列表
    #[serde(rename = "recommend_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_list: Option<Vec<ExternalMappingInfo>>,
    /// 上游字段 attributes。
    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Vec<ExternalProductMappingNewResponseAttribute>>,
}

/// 微信小店 ExternalMappingInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingNewResponse.java。
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

/// 微信小店嵌套数据。对应 Java: product.ExternalProductMappingNewResponse#Attribute
/// 微信小店 ExternalProductMappingNewResponseAttribute 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ExternalProductMappingNewResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProductMappingNewResponseAttribute {
    /// 上游字段 key。
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 上游字段 value。
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
