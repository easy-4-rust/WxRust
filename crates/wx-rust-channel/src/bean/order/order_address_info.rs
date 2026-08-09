//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderAddressInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderAddressInfo {
    #[serde(rename = "postal_code", default)]
    pub postal_code: String,
    #[serde(rename = "province_name", default)]
    pub province_name: String,
    #[serde(rename = "city_name", default)]
    pub city_name: String,
    #[serde(rename = "county_name", default)]
    pub county_name: String,
    #[serde(rename = "virtual_order_tel_number", default)]
    pub virtual_order_tel_number: String,
    #[serde(rename = "tel_number_ext_info", default)]
    pub tel_number_ext_info: TelNumberExtInfo,
    #[serde(rename = "use_tel_number", default)]
    pub use_tel_number: i32,
    #[serde(rename = "hash_code", default)]
    pub hash_code: String,
}
