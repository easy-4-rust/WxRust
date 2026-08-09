//! 对应 Java `me.chanjar.weixin.channel.bean.sharer.SharerSearchResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharerSearchResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
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
