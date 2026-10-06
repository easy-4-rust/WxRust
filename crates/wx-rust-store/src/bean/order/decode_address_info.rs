//! 对应 Java `com.binarywang.wxjava.store.bean.order.DecodeAddressInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 DecodeAddressInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.DecodeAddressInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecodeAddressInfo {
    #[serde(rename = "postal_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "province_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_name: Option<String>,
    #[serde(rename = "city_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[serde(rename = "county_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county_name: Option<String>,
    #[serde(rename = "virtual_order_tel_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_order_tel_number: Option<String>,

    #[serde(rename = "user_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "tel_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel_number: Option<String>,

    #[serde(rename = "detail_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_info: Option<String>,

    #[serde(rename = "national_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_code: Option<String>,

    #[serde(rename = "house_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,

    #[serde(rename = "lat", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    #[serde(rename = "lng", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
}
