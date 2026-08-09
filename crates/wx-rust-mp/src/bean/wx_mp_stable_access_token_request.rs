//! 对应 Java `bean.WxMpStableAccessTokenRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpStableAccessTokenRequest {
    #[serde(rename = "grant_type", default)]
    pub grant_type: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "secret", default)]
    pub secret: String,
    #[serde(rename = "force_refresh", default)]
    pub force_refresh: bool,
}
