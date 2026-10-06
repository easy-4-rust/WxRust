//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.TemplateUpdateRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TemplateUpdateRequest；对应 Java com.binarywang.wxjava.store.bean.ewaybill.TemplateUpdateRequest.java。
pub struct TemplateUpdateRequest {
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称
    #[serde(rename = "template_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}
