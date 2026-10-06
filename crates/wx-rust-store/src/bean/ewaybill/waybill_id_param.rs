/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.ewaybill.WaybillIdParam
/// 微信小店 WaybillIdParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.ewaybill.WaybillIdParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaybillIdParam {
    #[serde(
        rename = "waybill_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub waybill_id: Option<String>,
}
