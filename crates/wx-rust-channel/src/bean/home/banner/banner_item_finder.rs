//! 对应 Java `me.chanjar.weixin.channel.bean.home.banner.BannerItemFinder.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerItemFinder {
    #[serde(rename = "finder_user_name", default)]
    pub finder_user_name: String,
    #[serde(rename = "feed_id", default)]
    pub feed_id: String,
}
