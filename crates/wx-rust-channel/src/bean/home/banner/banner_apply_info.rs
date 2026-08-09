//! 对应 Java `me.chanjar.weixin.channel.bean.home.banner.BannerApplyInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerApplyInfo {
    #[serde(rename = "apply_id", default)]
    pub apply_id: i32,
    #[serde(rename = "state", default)]
    pub state: i32,
    #[serde(rename = "scale", default)]
    pub scale: i32,
    #[serde(rename = "banner", default)]
    pub banner: Vec<BannerApplyDetail>,
}
