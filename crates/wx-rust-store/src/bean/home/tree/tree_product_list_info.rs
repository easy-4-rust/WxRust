//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.TreeProductListInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TreeProductListInfo；对应 Java com.binarywang.wxjava.store.bean.home.tree.TreeProductListInfo.java。
pub struct TreeProductListInfo {
    #[serde(rename = "level_1_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level1_id: Option<i32>,
    #[serde(rename = "level_2_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2_id: Option<i32>,
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "page_context", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_context: Option<String>,
}
