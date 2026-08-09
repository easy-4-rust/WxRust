//! 对应 Java `bean.device.WxDeviceMsg`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDeviceMsg {
    #[serde(rename = "device_type", default)]
    pub device_type: String,
    #[serde(rename = "device_id", default)]
    pub device_id: String,
    #[serde(rename = "open_id", default)]
    pub open_id: String,
    #[serde(rename = "content", default)]
    pub content: String,
}
