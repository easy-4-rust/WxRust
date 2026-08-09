//! 对应 Java `me.chanjar.weixin.channel.bean.image.ChannelImageInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelImageInfo {
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "img_url", default)]
    pub url: String,
    #[serde(rename = "pay_media_id", default)]
    pub pay_media_id: String,
}
