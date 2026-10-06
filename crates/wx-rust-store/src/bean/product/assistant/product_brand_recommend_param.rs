//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.ProductBrandRecommendParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ProductBrandRecommendParam；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ProductBrandRecommendParam.java。
pub struct ProductBrandRecommendParam {
    /// 商品名称
    #[serde(rename = "product_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// 上游字段 cat_id。
    #[serde(rename = "cat_id", skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<i64>,
    /// 上游字段 head_imgs。
    #[serde(rename = "head_imgs", skip_serializing_if = "Option::is_none")]
    pub head_imgs: Option<Vec<String>>,
    /// 上游字段 detail_imgs。
    #[serde(
        rename = "detail_imgs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detail_imgs: Option<Vec<String>>,
    /// 上游字段 title。
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
