/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.ewaybill.EwaybillOrderIdParam
/// 微信小店 EwaybillOrderIdParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.ewaybill.EwaybillOrderIdParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EwaybillOrderIdParam {
    #[serde(
        rename = "ewaybill_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ewaybill_order_id: Option<String>,
}
