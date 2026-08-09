//! 对应 Java `me.chanjar.weixin.cp.bean.workbench.WorkBenchList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkBenchList {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "jumpUrl", default)]
    pub jump_url: String,
    #[serde(rename = "pagePath", default)]
    pub page_path: String,
}
