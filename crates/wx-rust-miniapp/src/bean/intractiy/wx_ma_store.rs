//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaStore.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaStore {
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "outStoreId", default)]
    pub out_store_id: String,
    #[serde(rename = "storeName", default)]
    pub store_name: String,
    #[serde(rename = "cityId", default)]
    pub city_id: String,
    #[serde(rename = "orderPattern", default)]
    pub order_pattern: i32,
    #[serde(rename = "ServiceTransPrefer", default)]
    pub service_trans_prefer: String,
    #[serde(rename = "addressInfo", default)]
    pub address_info: AddressInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressInfo {
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "area", default)]
    pub area: String,
    #[serde(rename = "street", default)]
    pub street: String,
    #[serde(rename = "house", default)]
    pub house: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "lat", default)]
    pub lat: f64,
    #[serde(rename = "lng", default)]
    pub lng: f64,
}
