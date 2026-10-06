//! 对应 Java `com.binarywang.wxjava.store.bean.audit.CategoryBrand.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CategoryBrand；对应 Java com.binarywang.wxjava.store.bean.audit.CategoryBrand.java。
pub struct CategoryBrand {
    #[serde(rename = "brand_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
}
