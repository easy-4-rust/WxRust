//! 对应 Java `bean.device.WxDeviceBindDeviceResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDeviceBindDeviceResult {
    #[serde(rename = "resp_msg", default)]
    pub resp_msg: RespMsg,
    #[serde(rename = "openid", default)]
    pub open_id: String,
    #[serde(rename = "device_list", default)]
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Device {
    #[serde(rename = "device_type", default)]
    pub device_type: String,
    #[serde(rename = "device_id", default)]
    pub device_id: String,
}

impl WxDeviceBindDeviceResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxDeviceBindDeviceResult 解析失败: {e}"))
    }
}
