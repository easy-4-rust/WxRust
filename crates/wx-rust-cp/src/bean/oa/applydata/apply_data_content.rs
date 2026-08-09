//! 对应 Java `me.chanjar.weixin.cp.bean.oa.applydata.ApplyDataContent.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplyDataContent {
    #[serde(rename = "control", default)]
    pub control: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "title", default)]
    pub titles: Vec<crate::bean::oa::applydata::content_title::ContentTitle>,
    #[serde(rename = "value", default)]
    pub value: crate::bean::oa::applydata::content_value::ContentValue,
}
