//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.BizBaseInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BizBaseInfo {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "headimg_url", default)]
    pub headimg_url: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
}
