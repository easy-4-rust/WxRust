//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxOpenMaCategory.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaCategory {
    #[serde(rename = "first_class", default)]
    pub first_class: String,
    #[serde(rename = "second_class", default)]
    pub second_class: String,
    #[serde(rename = "third_class", default)]
    pub third_class: String,
    #[serde(rename = "first_id", default)]
    pub first_id: i32,
    #[serde(rename = "second_id", default)]
    pub second_id: i32,
    #[serde(rename = "third_id", default)]
    pub third_id: i32,
}
