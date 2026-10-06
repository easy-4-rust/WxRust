//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductSchemeParam.java`。

/// 获取商品移动应用跳转 scheme 码请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductSchemeParam {
    /// 商品 ID。
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 来源 appid。
    #[serde(rename = "from_appid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_appid: Option<String>,
    /// 过期时间。
    #[serde(rename = "expire", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i32>,
    /// 扩展信息。
    #[serde(rename = "ext_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<String>,
}
