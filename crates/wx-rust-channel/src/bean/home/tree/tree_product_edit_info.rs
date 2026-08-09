//! 对应 Java `me.chanjar.weixin.channel.bean.home.tree.TreeProductEditInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TreeProductEditInfo {
    #[serde(rename = "level_1_id", default)]
    pub level1_id: i32,
    #[serde(rename = "level_2_id", default)]
    pub level2_id: i32,
    #[serde(rename = "product_ids", default)]
    pub product_ids: Vec<i64>,
}
