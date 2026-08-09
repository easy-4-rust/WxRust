//! 对应 Java `cn.binarywang.wx.miniapp.bean.device.WxMaIotGroupDeviceInfoResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaIotGroupDeviceInfoResponse {
    #[serde(rename = "group_name", default)]
    pub group_name: String,
    #[serde(rename = "device_list", default)]
    pub device_list: Vec<WxMaDeviceTicketRequest>,
    #[serde(rename = "model_id", default)]
    pub model_id: String,
    #[serde(rename = "model_type", default)]
    pub model_type: String,
}

impl WxMaIotGroupDeviceInfoResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaIotGroupDeviceInfoResponse 序列化失败: {e}"))
    }
}
