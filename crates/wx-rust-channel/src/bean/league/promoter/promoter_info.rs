//! 对应 Java `me.chanjar.weixin.channel.bean.league.promoter.PromoterInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromoterInfo {
    #[serde(rename = "finder_id", default)]
    pub finder_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "invite_time", default)]
    pub invite_time: i64,
    #[serde(rename = "sale_product_number", default)]
    pub sale_product_number: i32,
    #[serde(rename = "sale_gmv", default)]
    pub sale_gmv: i32,
}
