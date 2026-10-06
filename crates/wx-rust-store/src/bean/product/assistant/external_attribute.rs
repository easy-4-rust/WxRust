/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.product.assistant.ExternalAttribute
/// 微信小店 ExternalAttribute 数据类型；对应 Java: com.binarywang.wxjava.store.bean.product.assistant.ExternalAttribute。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalAttribute {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
