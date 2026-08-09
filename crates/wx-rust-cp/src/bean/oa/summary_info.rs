//! 对应 Java `me.chanjar.weixin.cp.bean.oa.SummaryInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SummaryInfo {
    #[serde(rename = "summary_info", default)]
    pub summary_info_data: Vec<SummaryInfoData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SummaryInfoData {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "lang", default)]
    pub lang: String,
}
