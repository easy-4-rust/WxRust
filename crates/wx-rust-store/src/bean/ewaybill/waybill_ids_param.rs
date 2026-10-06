/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.ewaybill.WaybillIdsParam
/// 微信小店 WaybillIdsParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.ewaybill.WaybillIdsParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaybillIdsParam {
    #[serde(
        rename = "waybill_ids",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub waybill_ids: Option<Vec<String>>,
}
