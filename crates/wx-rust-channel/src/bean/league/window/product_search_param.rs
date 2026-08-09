//! 对应 Java `me.chanjar.weixin.channel.bean.league.window.ProductSearchParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductSearchParam {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "openfinderid", default)]
    pub openfinderid: String,
    #[serde(rename = "offset", default)]
    pub offset: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "need_total_num", default)]
    pub need_total_num: bool,
}
