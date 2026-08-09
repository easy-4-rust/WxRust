//! 对应 Java `me.chanjar.weixin.channel.bean.token.StableTokenParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StableTokenParam {
    #[serde(rename = "grant_type", default)]
    pub grant_type: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "secret", default)]
    pub secret: String,
    #[serde(rename = "force_refresh", default)]
    pub force_refresh: bool,
}
