//! 对应 Java `me.chanjar.weixin.channel.bean.home.banner.BannerApplyDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerApplyDetail {
    #[serde(rename = "audit_state", default)]
    pub audit_state: i32,
    #[serde(rename = "audit_desc", default)]
    pub audit_desc: String,
    #[serde(rename = "banner", default)]
    pub banner: BannerItem,
}
