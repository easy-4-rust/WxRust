//! 对应 Java `com.binarywang.wxjava.store.bean.category.PassCategoryInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 PassCategoryInfo；对应 Java com.binarywang.wxjava.store.bean.category.PassCategoryInfo.java。
pub struct PassCategoryInfo {
    #[serde(rename = "cat_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<String>,
    #[serde(rename = "qua_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qua_id: Option<String>,
}
