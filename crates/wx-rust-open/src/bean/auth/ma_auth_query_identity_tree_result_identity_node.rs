//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthQueryIdentityTreeResultIdentityNode.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthQueryIdentityTreeResultIdentityNode {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "node_id", default)]
    pub node_id: i32,
    #[serde(rename = "leaf_info", default)]
    pub leaf_info: MaAuthQueryIdentityTreeResultIdentityLeaf,
    #[serde(rename = "node_list", default)]
    pub node_list: Vec<MaAuthQueryIdentityTreeResultIdentityNode>,
}
