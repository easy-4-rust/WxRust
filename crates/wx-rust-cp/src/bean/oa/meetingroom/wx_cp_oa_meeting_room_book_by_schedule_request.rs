//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meetingroom.WxCpOaMeetingRoomBookByScheduleRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaMeetingRoomBookByScheduleRequest {
    #[serde(rename = "meetingroom_id", default)]
    pub meetingroom_id: i32,
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: String,
    #[serde(rename = "booker", default)]
    pub booker: String,
}

impl WxCpOaMeetingRoomBookByScheduleRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpOaMeetingRoomBookByScheduleRequest 序列化失败: {e}"))
    }
}
