/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.product.ProductTimingSaleParam
/// 微信小店 ProductTimingSaleParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.product.ProductTimingSaleParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductTimingSaleParam {
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
    #[serde(rename = "task_id", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
}
