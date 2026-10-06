//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.TemplateIdResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TemplateIdResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.TemplateIdResponse.java。
pub struct TemplateIdResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}
