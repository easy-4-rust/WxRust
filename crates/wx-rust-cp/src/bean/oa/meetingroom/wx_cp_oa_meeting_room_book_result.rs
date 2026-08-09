//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meetingroom.WxCpOaMeetingRoomBookResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaMeetingRoomBookResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "booking_id", default)]
    pub booking_id: String,
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: String,
    #[serde(rename = "conflict_date", default)]
    pub conflict_date: Vec<i32>,
}

impl WxCpOaMeetingRoomBookResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpOaMeetingRoomBookResult 解析失败: {e}"))
    }
}
