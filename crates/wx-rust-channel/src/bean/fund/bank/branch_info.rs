//! 对应 Java `me.chanjar.weixin.channel.bean.fund.bank.BranchInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::fund::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchInfo {
    #[serde(rename = "branch_id", default)]
    pub branch_id: i32,
    #[serde(rename = "branch_name", default)]
    pub branch_name: String,
}
