/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.ewaybill.PrintContentParam
/// 微信小店 PrintContentParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.ewaybill.PrintContentParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrintContentParam {
    #[serde(
        rename = "ewaybill_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ewaybill_order_id: Option<String>,
    #[serde(
        rename = "template_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<String>,
}
