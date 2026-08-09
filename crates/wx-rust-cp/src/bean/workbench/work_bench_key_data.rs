//! 对应 Java `me.chanjar.weixin.cp.bean.workbench.WorkBenchKeyData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkBenchKeyData {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "data", default)]
    pub data: String,
    #[serde(rename = "jumpUrl", default)]
    pub jump_url: String,
    #[serde(rename = "pagePath", default)]
    pub page_path: String,
}
