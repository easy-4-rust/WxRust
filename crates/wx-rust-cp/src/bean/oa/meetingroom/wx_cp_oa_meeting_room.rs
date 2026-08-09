//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meetingroom.WxCpOaMeetingRoom.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaMeetingRoom {
    #[serde(rename = "meetingroom_id", default)]
    pub meetingroom_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "capacity", default)]
    pub capacity: i32,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "building", default)]
    pub building: String,
    #[serde(rename = "floor", default)]
    pub floor: String,
    #[serde(rename = "equipment", default)]
    pub equipment: Vec<i32>,
    #[serde(rename = "coordinate", default)]
    pub coordinate: Coordinate,
    #[serde(rename = "need_approval", default)]
    pub need_approval: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coordinate {
    #[serde(rename = "latitude", default)]
    pub latitude: String,
    #[serde(rename = "longitude", default)]
    pub longitude: String,
}

impl WxCpOaMeetingRoom {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpOaMeetingRoom 序列化失败: {e}"))
    }
}
