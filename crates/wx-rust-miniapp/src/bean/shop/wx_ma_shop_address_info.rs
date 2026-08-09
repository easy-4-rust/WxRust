//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopAddressInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAddressInfo {
    #[serde(rename = "receiver_name", default)]
    pub receiver_name: String,
    #[serde(rename = "detailed_address", default)]
    pub detailed_address: String,
    #[serde(rename = "tel_number", default)]
    pub tel_number: String,
    #[serde(rename = "country", default)]
    pub country: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "town", default)]
    pub town: String,
}
