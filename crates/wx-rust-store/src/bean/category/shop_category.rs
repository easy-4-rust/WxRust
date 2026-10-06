//! 对应 Java `com.binarywang.wxjava.store.bean.category.ShopCategory.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ShopCategory；对应 Java com.binarywang.wxjava.store.bean.category.ShopCategory.java。
pub struct ShopCategory {
    #[serde(rename = "cat_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "f_cat_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "level", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "leaf", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<bool>,
}
