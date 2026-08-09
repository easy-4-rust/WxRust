//! 对应 Java `me.chanjar.weixin.channel.bean.league.window.AuthInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "auth_url", default)]
    pub auth_url: String,
    #[serde(rename = "auth_wxa_path", default)]
    pub auth_wxa_path: String,
    #[serde(rename = "auth_wxa_appid", default)]
    pub auth_wxa_appid: String,
    #[serde(rename = "auth_wxa_username", default)]
    pub auth_wxa_username: String,
}
