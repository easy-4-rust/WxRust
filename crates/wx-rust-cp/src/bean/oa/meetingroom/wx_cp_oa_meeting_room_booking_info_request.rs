//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meetingroom.WxCpOaMeetingRoomBookingInfoRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaMeetingRoomBookingInfoRequest {
    #[serde(rename = "meetingroom_id", default)]
    pub meetingroom_id: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i32,
    #[serde(rename = "end_time", default)]
    pub end_time: i32,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "building", default)]
    pub building: String,
    #[serde(rename = "floor", default)]
    pub floor: String,
}

impl WxCpOaMeetingRoomBookingInfoRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpOaMeetingRoomBookingInfoRequest 序列化失败: {e}"))
    }
}
