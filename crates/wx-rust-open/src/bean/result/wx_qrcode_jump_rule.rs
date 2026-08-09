//! 对应 Java `me.chanjar.weixin.open.bean.result.WxQrcodeJumpRule.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxQrcodeJumpRule {
    #[serde(rename = "prefix", default)]
    pub prefix: String,
    #[serde(rename = "permit_sub_rule", default)]
    pub permit_sub_rule: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "open_version", default)]
    pub open_version: String,
    #[serde(rename = "debug_url", default)]
    pub debug_url: Vec<String>,
    #[serde(rename = "is_edit", default)]
    pub is_edit: String,
    #[serde(rename = "state", default)]
    pub state: String,
}
