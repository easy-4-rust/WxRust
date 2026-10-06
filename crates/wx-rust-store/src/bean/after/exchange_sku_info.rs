/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.ExchangeSkuInfo
/// 微信小店 ExchangeSkuInfo 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.ExchangeSkuInfo。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeSkuInfo {
    #[serde(
        rename = "new_sku_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_sku_id: Option<String>,
}
