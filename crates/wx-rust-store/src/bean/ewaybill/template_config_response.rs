//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.TemplateConfigResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TemplateConfigResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.TemplateConfigResponse.java。
pub struct TemplateConfigResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 模板配置列表
    #[serde(rename = "template_config_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_config_list: Option<Vec<TemplateConfig>>,
}

/// 微信小店 TemplateConfig 数据类型；对应 Java com.binarywang.wxjava.store.bean.ewaybill.TemplateConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateConfig {
    /// 模板编码
    #[serde(rename = "template_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// 模板名称
    #[serde(rename = "template_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}
