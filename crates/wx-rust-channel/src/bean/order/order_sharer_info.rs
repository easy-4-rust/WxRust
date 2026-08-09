//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderSharerInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderSharerInfo {
    #[serde(rename = "sharer_openid", default)]
    pub sharer_openid: String,
    #[serde(rename = "sharer_unionid", default)]
    pub sharer_unionid: String,
    #[serde(rename = "sharer_type", default)]
    pub sharer_type: i32,
    #[serde(rename = "share_scene", default)]
    pub share_scene: i32,
    #[serde(rename = "handling_progress", default)]
    pub handling_progress: i32,
}
