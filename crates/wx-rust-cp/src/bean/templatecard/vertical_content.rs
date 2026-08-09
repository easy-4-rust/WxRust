//! 对应 Java `me.chanjar.weixin.cp.bean.templatecard.VerticalContent.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VerticalContent {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
}
