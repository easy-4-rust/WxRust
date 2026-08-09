//! 对应 Java `me.chanjar.weixin.cp.bean.oa.templatedata.TemplateDateRange.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateDateRange {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "official_holiday", default)]
    pub official_holiday: i32,
    #[serde(rename = "perday_duration", default)]
    pub perday_duration: i32,
}
