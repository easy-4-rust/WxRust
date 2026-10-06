/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleRefundPriceDiffParam
/// 微信小店 AfterSaleRefundPriceDiffParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleRefundPriceDiffParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleRefundPriceDiffParam {
    #[serde(
        rename = "request_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_id: Option<String>,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}
