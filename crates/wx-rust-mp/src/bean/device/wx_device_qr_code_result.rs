//! 对应 Java `bean.device.WxDeviceQrCodeResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDeviceQrCodeResult {
    #[serde(rename = "deviceid", default)]
    pub device_id: String,
    #[serde(rename = "qrticket", default)]
    pub qr_ticket: String,
    #[serde(rename = "devicelicence", default)]
    pub device_licence: String,
    #[serde(rename = "base_resp", default)]
    pub base_resp: BaseResp,
}

impl WxDeviceQrCodeResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxDeviceQrCodeResult 解析失败: {e}"))
    }
}
