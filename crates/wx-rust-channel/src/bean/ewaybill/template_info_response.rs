//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.TemplateInfoResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateInfoResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 模板信息
    #[serde(rename = "template_info", default)]
    pub template_info: TemplateInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateInfo {
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    /// 模板编码
    #[serde(rename = "template_code", default)]
    pub template_code: String,
    /// 模板名称
    #[serde(rename = "template_name", default)]
    pub template_name: String,
}
