//! 对应 Java `me.chanjar.weixin.channel.bean.category.ShopCategory.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopCategory {
    #[serde(rename = "cat_id", default)]
    pub id: String,
    #[serde(rename = "f_cat_id", default)]
    pub parent_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "level", default)]
    pub level: i32,
    #[serde(rename = "leaf", default)]
    pub leaf: bool,
}
