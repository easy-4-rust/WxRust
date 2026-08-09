//! 对应 Java `bean.store.WxMpStoreBaseInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpStoreBaseInfo {
    #[serde(rename = "sid", default)]
    pub sid: String,
    #[serde(rename = "business_name", default)]
    pub business_name: String,
    #[serde(rename = "branch_name", default)]
    pub branch_name: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "district", default)]
    pub district: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "telephone", default)]
    pub telephone: String,
    #[serde(rename = "categories", default)]
    pub categories: Vec<String>,
    #[serde(rename = "offset_type", default)]
    pub offset_type: i32,
    #[serde(rename = "longitude", default)]
    pub longitude: String,
    #[serde(rename = "latitude", default)]
    pub latitude: String,
    #[serde(rename = "photo_list", default)]
    pub photos: Vec<WxMpStorePhoto>,
    #[serde(rename = "recommend", default)]
    pub recommend: String,
    #[serde(rename = "special", default)]
    pub special: String,
    #[serde(rename = "introduction", default)]
    pub introduction: String,
    #[serde(rename = "open_time", default)]
    pub open_time: String,
    #[serde(rename = "avg_price", default)]
    pub avg_price: i32,
    #[serde(rename = "available_state", default)]
    pub available_state: i32,
    #[serde(rename = "update_status", default)]
    pub update_status: i32,
    #[serde(rename = "poi_id", default)]
    pub poi_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpStorePhoto {
    #[serde(rename = "photo_url", default)]
    pub photo_url: String,
}

impl WxMpStoreBaseInfo {
    /// 序列化为 JSON（对应 Java `toJson`：`{"business": {"base_info": {...}}}`）。
    pub fn to_json(&self) -> String {
        serde_json::json!({ "business": { "base_info": self } }).to_string()
    }
}
