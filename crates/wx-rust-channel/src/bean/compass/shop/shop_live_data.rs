//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.ShopLiveData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopLiveData {
    #[serde(rename = "live_id", default)]
    pub live_id: String,
    #[serde(rename = "live_title", default)]
    pub live_title: String,
    #[serde(rename = "live_time", default)]
    pub live_time: String,
    #[serde(rename = "live_duration", default)]
    pub live_duration: String,
    #[serde(rename = "live_cover_img_url", default)]
    pub live_cover_img_url: String,
}
