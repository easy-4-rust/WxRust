//! 对应 Java `bean.shake.WxMpShakeAroundRelationSearchQuery`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpShakeAroundRelationSearchQuery {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "pageId", default)]
    pub page_id: i32,
    #[serde(rename = "begin", default)]
    pub begin: i32,
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "deviceIdentifier", default)]
    pub device_identifier: WxMpDeviceIdentifier,
}
