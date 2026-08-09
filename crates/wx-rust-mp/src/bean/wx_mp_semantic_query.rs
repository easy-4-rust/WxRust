//! 对应 Java `bean.WxMpSemanticQuery`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpSemanticQuery {
    #[serde(rename = "query", default)]
    pub query: String,
    #[serde(rename = "category", default)]
    pub category: String,
    #[serde(rename = "latitude", default)]
    pub latitude: f32,
    #[serde(rename = "longitude", default)]
    pub longitude: f32,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "region", default)]
    pub region: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "uid", default)]
    pub uid: String,
}
