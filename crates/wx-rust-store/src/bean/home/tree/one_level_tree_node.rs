//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.OneLevelTreeNode.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::CatTreeNode;

/// 微信小店 OneLevelTreeNode 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.tree.OneLevelTreeNode.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OneLevelTreeNode {
    #[serde(rename = "id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "is_displayed", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayed: Option<bool>,
    #[serde(rename = "level_2", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2: Option<Vec<CatTreeNode>>,
}
