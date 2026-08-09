//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meeting.WxCpUserMeetingIdResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserMeetingIdResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "meetingid_list", default)]
    pub meeting_id_list: Vec<String>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

impl WxCpUserMeetingIdResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpUserMeetingIdResult 解析失败: {e}"))
    }
}
