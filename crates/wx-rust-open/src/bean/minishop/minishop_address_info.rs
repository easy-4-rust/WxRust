//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopAddressInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopAddressInfo {
    #[serde(rename = "userName", default)]
    pub user_name: String,
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "cityName", default)]
    pub city_name: String,
    #[serde(rename = "countyName", default)]
    pub county_name: String,
    #[serde(rename = "detailInfo", default)]
    pub detail_info: String,
    #[serde(rename = "nationalCode", default)]
    pub national_code: String,
    #[serde(rename = "telNumber", default)]
    pub tel_number: String,
}
