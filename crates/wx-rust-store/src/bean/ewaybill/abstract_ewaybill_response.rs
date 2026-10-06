/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.ewaybill.AbstractEwaybillResponse
/// 微信小店 AbstractEwaybillResponse 数据类型；对应 Java: com.binarywang.wxjava.store.bean.ewaybill.AbstractEwaybillResponse。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AbstractEwaybillResponse {
    #[serde(rename = "errcode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<i32>,
    #[serde(rename = "errmsg", skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
}
