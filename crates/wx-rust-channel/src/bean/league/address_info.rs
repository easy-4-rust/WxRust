//! 对应 Java `me.chanjar.weixin.channel.bean.league.AddressInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressInfo {
    #[serde(rename = "postal_code", default)]
    pub postal_code: String,
    #[serde(rename = "province_name", default)]
    pub province_name: String,
    #[serde(rename = "city_name", default)]
    pub city_name: String,
    #[serde(rename = "county_name", default)]
    pub county_name: String,
}
