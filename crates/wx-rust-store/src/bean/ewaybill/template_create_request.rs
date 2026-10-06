//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.TemplateCreateRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TemplateCreateRequest；对应 Java com.binarywang.wxjava.store.bean.ewaybill.TemplateCreateRequest.java。
pub struct TemplateCreateRequest {
    /// 标准模板编码
    #[serde(rename = "template_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// 商家自定义模板名称
    #[serde(rename = "template_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}
