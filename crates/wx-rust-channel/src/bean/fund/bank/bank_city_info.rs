//! 对应 Java `me.chanjar.weixin.channel.bean.fund.bank.BankCityInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::fund::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankCityInfo {
    #[serde(rename = "city_name", default)]
    pub city_name: String,
    #[serde(rename = "city_code", default)]
    pub city_code: i32,
    #[serde(rename = "bank_address_code", default)]
    pub bank_address_code: String,
}
