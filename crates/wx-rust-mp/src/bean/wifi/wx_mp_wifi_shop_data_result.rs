//! 对应 Java `bean.wifi.WxMpWifiShopDataResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpWifiShopDataResult {
    #[serde(rename = "shop_name", default)]
    pub shop_name: String,
    #[serde(rename = "ssid", default)]
    pub ssid: String,
    #[serde(rename = "ssid_list", default)]
    pub ssid_list: Vec<String>,
    #[serde(rename = "ssid_password_list", default)]
    pub ssid_password_list: Vec<SsidPassword>,
    #[serde(rename = "password", default)]
    pub password: String,
    #[serde(rename = "protocol_type", default)]
    pub protocol_type: i32,
    #[serde(rename = "ap_count", default)]
    pub ap_count: i32,
    #[serde(rename = "template_id", default)]
    pub template_id: i32,
    #[serde(rename = "homepage_url", default)]
    pub homepage_url: String,
    #[serde(rename = "bar_type", default)]
    pub bar_type: i32,
    #[serde(rename = "finishpage_url", default)]
    pub finish_page_url: String,
    #[serde(rename = "sid", default)]
    pub sid: String,
    #[serde(rename = "poi_id", default)]
    pub poi_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SsidPassword {
    #[serde(rename = "ssid", default)]
    pub ssid: String,
    #[serde(rename = "password", default)]
    pub password: String,
}

impl WxMpWifiShopDataResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpWifiShopDataResult 解析失败: {e}"))
    }
}
