//! 对应 Java `bean.wifi.WxMpWifiShopListResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpWifiShopListResult {
    #[serde(rename = "totalcount", default)]
    pub total_count: i32,
    #[serde(rename = "pageindex", default)]
    pub page_index: i32,
    #[serde(rename = "pagecount", default)]
    pub page_count: i32,
    #[serde(rename = "records", default)]
    pub records: Vec<Record>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Record {
    #[serde(rename = "shop_id", default)]
    pub shop_id: i32,
    #[serde(rename = "shop_name", default)]
    pub shop_name: String,
    #[serde(rename = "ssid", default)]
    pub ssid: String,
    #[serde(rename = "ssid_list", default)]
    pub ssid_list: Vec<String>,
    #[serde(rename = "protocol_type", default)]
    pub protocol_type: i32,
    #[serde(rename = "sid", default)]
    pub sid: String,
    #[serde(rename = "poi_id", default)]
    pub poi_id: String,
}

impl WxMpWifiShopListResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpWifiShopListResult 解析失败: {e}"))
    }
}
