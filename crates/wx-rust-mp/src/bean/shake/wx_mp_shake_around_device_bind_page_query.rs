//! 对应 Java `bean.shake.WxMpShakeAroundDeviceBindPageQuery`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpShakeAroundDeviceBindPageQuery {
    #[serde(rename = "deviceIdentifier", default)]
    pub device_identifier: WxMpDeviceIdentifier,
    #[serde(rename = "pageIds", default)]
    pub page_ids: Vec<i32>,
}
