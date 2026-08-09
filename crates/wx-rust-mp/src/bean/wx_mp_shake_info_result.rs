//! 对应 Java `bean.WxMpShakeInfoResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpShakeInfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "data", default)]
    pub data: ShakeInfoData,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShakeInfoData {
    #[serde(rename = "page_id", default)]
    pub page_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "poi_id", default)]
    pub poi_id: String,
    #[serde(rename = "brand_userame", default)]
    pub brand_userame: String,
    #[serde(rename = "beacon_info", default)]
    pub beacon_info: BeaconInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BeaconInfo {
    #[serde(rename = "distance", default)]
    pub distance: f64,
    #[serde(rename = "major", default)]
    pub major: i32,
    #[serde(rename = "measure_power", default)]
    pub measure_power: i32,
    #[serde(rename = "minor", default)]
    pub minor: i32,
    #[serde(rename = "rssi", default)]
    pub rssi: i32,
    #[serde(rename = "uuid", default)]
    pub uuid: String,
}

impl WxMpShakeInfoResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpShakeInfoResult 解析失败: {e}"))
    }
}
