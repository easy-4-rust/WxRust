//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.LevelTreeInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::OneLevelTreeNode;

/// 微信小店 LevelTreeInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.tree.LevelTreeInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LevelTreeInfo {
    #[serde(rename = "level_1", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level1: Option<Vec<OneLevelTreeNode>>,
}
