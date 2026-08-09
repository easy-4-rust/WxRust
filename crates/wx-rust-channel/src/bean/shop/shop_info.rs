//! 对应 Java `me.chanjar.weixin.channel.bean.shop.ShopInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopInfo {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "headimg_url", default)]
    pub head_img_url: String,
    #[serde(rename = "subject_type", default)]
    pub subject_type: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "username", default)]
    pub username: String,
}
