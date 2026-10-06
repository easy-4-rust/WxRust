//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.TreeShowInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::LevelTreeInfo;

/// 微信小店 TreeShowInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.tree.TreeShowInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TreeShowInfo {
    #[serde(rename = "tree", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree: Option<LevelTreeInfo>,
    #[serde(rename = "version", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "classification_id_deleted", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id_deleted: Option<Vec<String>>,
}
