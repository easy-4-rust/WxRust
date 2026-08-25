//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.TemplateCreateRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateCreateRequest {
    /// 标准模板编码
    #[serde(rename = "template_code", default)]
    pub template_code: String,
    /// 商家自定义模板名称
    #[serde(rename = "template_name", default)]
    pub template_name: String,
}
