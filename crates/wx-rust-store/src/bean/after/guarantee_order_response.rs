/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.GuaranteeOrderResponse
/// 微信小店 GuaranteeOrderResponse 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.GuaranteeOrderResponse。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderResponse {
    #[serde(rename = "errcode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<i32>,
    #[serde(rename = "errmsg", skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(
        rename = "guarantee_order",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guarantee_order: Option<serde_json::Value>,
}
