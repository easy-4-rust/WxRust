//! 对应 Java `me.chanjar.weixin.cp.bean.oa.calendar.WxCpOaCalendar.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaCalendar {
    #[serde(rename = "cal_id", default)]
    pub cal_id: String,
    #[serde(rename = "organizer", default)]
    pub organizer: String,
    #[serde(rename = "readonly", default)]
    pub readonly: i32,
    #[serde(rename = "set_as_default", default)]
    pub set_as_default: i32,
    #[serde(rename = "summary", default)]
    pub summary: String,
    #[serde(rename = "color", default)]
    pub color: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "shares", default)]
    pub shares: Vec<ShareInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShareInfo {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "readonly", default)]
    pub readonly: i32,
}

impl WxCpOaCalendar {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpOaCalendar 序列化失败: {e}"))
    }
}
