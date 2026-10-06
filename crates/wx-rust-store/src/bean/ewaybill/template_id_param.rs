/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.ewaybill.TemplateIdParam
/// 微信小店 TemplateIdParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.ewaybill.TemplateIdParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateIdParam {
    #[serde(
        rename = "template_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<String>,
}
