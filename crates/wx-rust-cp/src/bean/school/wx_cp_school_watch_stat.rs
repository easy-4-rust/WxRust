//! 对应 Java `me.chanjar.weixin.cp.bean.school.WxCpSchoolWatchStat.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpSchoolWatchStat {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "ending", default)]
    pub ending: i32,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "stat_infoes", default)]
    pub stat_infoes: StatInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatInfo {
    #[serde(rename = "students", default)]
    pub students: Vec<crate::bean::school::wx_cp_school_watch_stat::Student>,
    #[serde(rename = "visitors", default)]
    pub visitors: Vec<crate::bean::school::wx_cp_school_watch_stat::Visitor>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Student {
    #[serde(rename = "student_userid", default)]
    pub student_user_id: String,
    #[serde(rename = "parent_userid", default)]
    pub parent_user_id: String,
    #[serde(rename = "watch_time", default)]
    pub watch_time: i32,
    #[serde(rename = "is_comment", default)]
    pub is_comment: i32,
    #[serde(rename = "enter_time", default)]
    pub enter_time: i64,
    #[serde(rename = "leave_time", default)]
    pub leave_time: i64,
    #[serde(rename = "partyids", default)]
    pub party_ids: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Visitor {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "watch_time", default)]
    pub watch_time: i32,
    #[serde(rename = "is_comment", default)]
    pub is_comment: i32,
    #[serde(rename = "enter_time", default)]
    pub enter_time: i64,
    #[serde(rename = "leave_time", default)]
    pub leave_time: i64,
}

impl WxCpSchoolWatchStat {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpSchoolWatchStat 解析失败: {e}"))
    }
}

impl WxCpSchoolWatchStat {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpSchoolWatchStat 序列化失败: {e}"))
    }
}
