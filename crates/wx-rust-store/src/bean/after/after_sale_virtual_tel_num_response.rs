/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleVirtualTelNumResponse
/// 微信小店 AfterSaleVirtualTelNumResponse 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleVirtualTelNumResponse。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleVirtualTelNumResponse {
    #[serde(rename = "errcode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<i32>,
    #[serde(rename = "errmsg", skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(
        rename = "virtual_tel_number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_tel_number: Option<String>,
    #[serde(
        rename = "virtual_tel_expire_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_tel_expire_time: Option<i64>,
}
