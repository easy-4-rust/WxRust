//! 对应 Java `bean.device.WxDeviceBind`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDeviceBind {
    #[serde(rename = "ticket", default)]
    pub ticket: String,
    #[serde(rename = "device_id", default)]
    pub device_id: String,
    #[serde(rename = "openid", default)]
    pub open_id: String,
}
