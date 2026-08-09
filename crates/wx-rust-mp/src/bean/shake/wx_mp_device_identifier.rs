//! 对应 Java `bean.shake.WxMpDeviceIdentifier`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpDeviceIdentifier {
    #[serde(rename = "device_id", default)]
    pub device_id: i32,
    #[serde(rename = "uuid", default)]
    pub uuid: String,
    #[serde(rename = "page_id", default)]
    pub page_id: i32,
    #[serde(rename = "major", default)]
    pub major: i32,
    #[serde(rename = "minor", default)]
    pub minor: i32,
}
