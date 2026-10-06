/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.GuaranteeMerchantProofParam
/// 微信小店 GuaranteeMerchantProofParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.GuaranteeMerchantProofParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeMerchantProofParam {
    #[serde(
        rename = "guarantee_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guarantee_order_id: Option<String>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "pic_list", skip_serializing_if = "Option::is_none")]
    pub pic_list: Option<Vec<String>>,
}
