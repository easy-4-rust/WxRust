//! 对应 Java `me.chanjar.weixin.channel.bean.sharer.SharerInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharerInfo {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "unionid", default)]
    pub unionid: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "bind_time", default)]
    pub bind_time: i64,
    #[serde(rename = "sharer_type", default)]
    pub sharer_type: i32,
}
