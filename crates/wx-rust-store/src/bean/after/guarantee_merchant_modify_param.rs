/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.GuaranteeMerchantModifyParam
/// 微信小店 GuaranteeMerchantModifyParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.GuaranteeMerchantModifyParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeMerchantModifyParam {
    #[serde(
        rename = "guarantee_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guarantee_order_id: Option<String>,
    #[serde(rename = "bad_level", skip_serializing_if = "Option::is_none")]
    pub bad_level: Option<i32>,
    #[serde(
        rename = "merchant_remark",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merchant_remark: Option<String>,
}
