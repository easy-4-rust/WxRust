//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meetingroom.WxCpOaMeetingRoomCancelBookRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaMeetingRoomCancelBookRequest {
    #[serde(rename = "booking_id", default)]
    pub booking_id: String,
    #[serde(rename = "keep_schedule", default)]
    pub keep_schedule: i32,
    #[serde(rename = "cancel_date", default)]
    pub cancel_date: i32,
}

impl WxCpOaMeetingRoomCancelBookRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpOaMeetingRoomCancelBookRequest 序列化失败: {e}"))
    }
}
