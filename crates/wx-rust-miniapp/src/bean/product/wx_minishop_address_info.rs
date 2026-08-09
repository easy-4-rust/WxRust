//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopAddressInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopAddressInfo {
    #[serde(rename = "user_name", default)]
    pub user_name: String,
    #[serde(rename = "postal_code", default)]
    pub postal_code: String,
    #[serde(rename = "province_name", default)]
    pub province_name: String,
    #[serde(rename = "city_name", default)]
    pub city_name: String,
    #[serde(rename = "county_name", default)]
    pub county_name: String,
    #[serde(rename = "detail_info", default)]
    pub detail_info: String,
    #[serde(rename = "national_code", default)]
    pub national_code: String,
    #[serde(rename = "tel_number", default)]
    pub tel_number: String,
}
