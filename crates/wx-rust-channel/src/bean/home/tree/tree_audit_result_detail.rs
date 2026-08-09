//! 对应 Java `me.chanjar.weixin.channel.bean.home.tree.TreeAuditResultDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TreeAuditResultDetail {
    #[serde(rename = "level_id", default)]
    pub level_id: i32,
    #[serde(rename = "result_code", default)]
    pub result_code: i32,
}
