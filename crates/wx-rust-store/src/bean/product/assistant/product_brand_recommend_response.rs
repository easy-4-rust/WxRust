//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.ProductBrandRecommendResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ProductBrandRecommendResponse；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ProductBrandRecommendResponse.java。
pub struct ProductBrandRecommendResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 推荐品牌列表
    #[serde(rename = "brand_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_list: Option<Vec<RecommendedBrand>>,
    /// 上游字段 brand_id。
    #[serde(rename = "brand_id", skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<i64>,
    /// 上游字段 brand_name_chinese。
    #[serde(
        rename = "brand_name_chinese",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub brand_name_chinese: Option<String>,
    /// 上游字段 brand_name_english。
    #[serde(
        rename = "brand_name_english",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub brand_name_english: Option<String>,
}

/// 微信小店 RecommendedBrand 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.assistant.ProductBrandRecommendResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecommendedBrand {
    /// 品牌 ID
    #[serde(rename = "brand_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    /// 品牌名称
    #[serde(rename = "brand_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
}
