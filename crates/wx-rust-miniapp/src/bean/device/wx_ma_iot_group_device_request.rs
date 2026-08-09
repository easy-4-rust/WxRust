//! 对应 Java `cn.binarywang.wx.miniapp.bean.device.WxMaIotGroupDeviceRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaIotGroupDeviceRequest {
    #[serde(rename = "group_id", default)]
    pub group_id: String,
    #[serde(rename = "device_list", default)]
    pub device_list: Vec<WxMaDeviceTicketRequest>,
    #[serde(rename = "force_add", default)]
    pub force_add: bool,
}

impl WxMaIotGroupDeviceRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaIotGroupDeviceRequest 序列化失败: {e}"))
    }
}
