//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductSaleLimitInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ProductSaleLimitInfo；对应 Java com.binarywang.wxjava.store.bean.product.ProductSaleLimitInfo.java。
pub struct ProductSaleLimitInfo {
    #[serde(rename = "is_limited", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<i32>,
    #[serde(rename = "title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "sub_title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_title: Option<String>,
}
