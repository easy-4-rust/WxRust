/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleHandleFastExchangeReceiptParam
/// 微信小店 AfterSaleHandleFastExchangeReceiptParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleHandleFastExchangeReceiptParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleHandleFastExchangeReceiptParam {
    #[serde(
        rename = "after_sale_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_sale_order_id: Option<String>,
    #[serde(rename = "act", skip_serializing_if = "Option::is_none")]
    pub act: Option<i32>,
    #[serde(
        rename = "reject_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<String>,
    #[serde(
        rename = "reject_reason_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason_type: Option<i32>,
    #[serde(
        rename = "merchant_text",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merchant_text: Option<String>,
    #[serde(
        rename = "reject_confirm_exchange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_confirm_exchange: Option<Vec<String>>,
}
