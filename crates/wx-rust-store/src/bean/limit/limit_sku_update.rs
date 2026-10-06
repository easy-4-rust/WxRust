/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.limit.LimitSkuUpdate
/// 微信小店 LimitSkuUpdate 数据类型；对应 Java: com.binarywang.wxjava.store.bean.limit.LimitSkuUpdate。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitSkuUpdate {
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(
        rename = "sale_price",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sale_price: Option<i32>,
    #[serde(
        rename = "sale_stock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sale_stock: Option<i32>,
}
