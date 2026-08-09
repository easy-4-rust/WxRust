//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpCheckinSchedule.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCheckinSchedule {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "yearmonth", default)]
    pub yearmonth: i32,
    #[serde(rename = "groupid", default)]
    pub groupid: i32,
    #[serde(rename = "groupname", default)]
    pub group_name: String,
    #[serde(rename = "schedule", default)]
    pub schedule: UserSchedule,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserSchedule {
    #[serde(rename = "scheduleList", default)]
    pub schedule_list: Vec<Schedule>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    #[serde(rename = "day", default)]
    pub day: i32,
    #[serde(rename = "schedule_info", default)]
    pub schedule_info: ScheduleInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScheduleInfo {
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: i32,
    #[serde(rename = "schedule_name", default)]
    pub schedule_name: String,
    #[serde(rename = "time_section", default)]
    pub time_section: Vec<TimeSection>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeSection {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "work_sec", default)]
    pub work_sec: i32,
    #[serde(rename = "off_work_sec", default)]
    pub off_work_sec: i32,
    #[serde(rename = "remind_work_sec", default)]
    pub remind_work_sec: i32,
    #[serde(rename = "remind_off_work_sec", default)]
    pub remind_off_work_sec: i32,
}
