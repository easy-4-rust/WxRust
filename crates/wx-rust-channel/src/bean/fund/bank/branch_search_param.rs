//! 对应 Java `me.chanjar.weixin.channel.bean.fund.bank.BranchSearchParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::fund::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchSearchParam {
    #[serde(rename = "bank_code", default)]
    pub bank_code: String,
    #[serde(rename = "city_code", default)]
    pub city_code: String,
    #[serde(rename = "offset", default)]
    pub offset: i32,
    #[serde(rename = "limit", default)]
    pub limit: i32,
}
