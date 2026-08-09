//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meetingroom.WxCpOaMeetingRoomBookingInfoResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaMeetingRoomBookingInfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "booking_list", default)]
    pub booking_list: Vec<Booking>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Booking {
    #[serde(rename = "meetingroom_id", default)]
    pub meetingroom_id: i32,
    #[serde(rename = "schedule", default)]
    pub schedule:
        Vec<crate::bean::oa::meetingroom::wx_cp_oa_meeting_room_booking_info_result::Schedule>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    #[serde(rename = "booking_id", default)]
    pub booking_id: String,
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: String,
    #[serde(rename = "start_time", default)]
    pub start_time: i32,
    #[serde(rename = "end_time", default)]
    pub end_time: i32,
    #[serde(rename = "booker", default)]
    pub booker: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}

impl WxCpOaMeetingRoomBookingInfoResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpOaMeetingRoomBookingInfoResult 解析失败: {e}"))
    }
}
