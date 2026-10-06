/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleGenAfterSaleOrderParam
/// 微信小店 AfterSaleGenAfterSaleOrderParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.AfterSaleGenAfterSaleOrderParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleGenAfterSaleOrderParam {
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
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        rename = "address_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_id: Option<String>,
    #[serde(
        rename = "exchange_sku_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_sku_info: Option<super::ExchangeSkuInfo>,
}
