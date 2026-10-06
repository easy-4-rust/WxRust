//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.TemplateInfoResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TemplateInfoResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.TemplateInfoResponse.java。
pub struct TemplateInfoResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 模板信息
    #[serde(rename = "template_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_info: Option<TemplateInfo>,
}

/// 微信小店 TemplateInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.ewaybill.TemplateInfoResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateInfo {
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板编码
    #[serde(rename = "template_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// 模板名称
    #[serde(rename = "template_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}
