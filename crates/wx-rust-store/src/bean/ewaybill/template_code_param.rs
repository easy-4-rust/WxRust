/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.ewaybill.TemplateCodeParam
/// 微信小店 TemplateCodeParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.ewaybill.TemplateCodeParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateCodeParam {
    #[serde(
        rename = "template_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_code: Option<String>,
}
