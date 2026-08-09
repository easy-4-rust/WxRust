//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.CoopProductListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoopProductListParam {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
