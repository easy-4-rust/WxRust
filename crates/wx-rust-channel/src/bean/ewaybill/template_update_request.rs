//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.TemplateUpdateRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateUpdateRequest {
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    /// 模板名称
    #[serde(rename = "template_name", default)]
    pub template_name: String,
}
