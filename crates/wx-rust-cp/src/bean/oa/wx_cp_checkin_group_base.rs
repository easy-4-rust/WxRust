//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpCheckinGroupBase.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCheckinGroupBase {
    #[serde(rename = "grouptype", default)]
    pub group_type: i64,
    #[serde(rename = "groupid", default)]
    pub group_id: i64,
    #[serde(rename = "groupname", default)]
    pub group_name: String,
    #[serde(rename = "checkindate", default)]
    pub checkin_date: Vec<CheckinDate>,
    #[serde(rename = "spe_workdays", default)]
    pub spe_workdays: Vec<SpeWorkday>,
    #[serde(rename = "spe_offdays", default)]
    pub spe_off_days: Vec<SpeOffDay>,
    #[serde(rename = "sync_holidays", default)]
    pub sync_holidays: bool,
    #[serde(rename = "need_photo", default)]
    pub need_photo: bool,
    #[serde(rename = "note_can_use_local_pic", default)]
    pub note_can_use_local_pic: bool,
    #[serde(rename = "allow_checkin_offworkday", default)]
    pub allow_checkin_off_work_day: bool,
    #[serde(rename = "allow_apply_offworkday", default)]
    pub allow_apply_off_work_day: bool,
    #[serde(rename = "wifimac_infos", default)]
    pub wifi_mac_infos: Vec<WifiMacInfo>,
    #[serde(rename = "loc_infos", default)]
    pub loc_infos: Vec<LocInfo>,
    #[serde(rename = "schedulelist", default)]
    pub schedulelist: Vec<Schedule>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckinDate {
    #[serde(rename = "workdays", default)]
    pub workdays: Vec<i32>,
    #[serde(rename = "checkintime", default)]
    pub checkin_time: Vec<crate::bean::oa::wx_cp_checkin_group_base::CheckinTime>,
    #[serde(rename = "noneed_offwork", default)]
    pub noneed_offwork: bool,
    #[serde(rename = "limit_aheadtime", default)]
    pub limit_aheadtime: i64,
    #[serde(rename = "flex_time", default)]
    pub flex_time: i32,
    #[serde(rename = "flex_on_duty_time", default)]
    pub flex_on_duty_time: i32,
    #[serde(rename = "flex_off_duty_time", default)]
    pub flex_off_duty_time: i32,
    #[serde(rename = "allow_flex", default)]
    pub allow_flex: bool,
    #[serde(rename = "late_rule", default)]
    pub late_rule: crate::bean::oa::wx_cp_checkin_group_base::LateRule,
    #[serde(rename = "max_allow_arrive_early", default)]
    pub max_allow_arrive_early: i32,
    #[serde(rename = "max_allow_arrive_late", default)]
    pub max_allow_arrive_late: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckinTime {
    #[serde(rename = "time_id", default)]
    pub time_id: i32,
    #[serde(rename = "work_sec", default)]
    pub work_sec: i32,
    #[serde(rename = "off_work_sec", default)]
    pub off_work_sec: i32,
    #[serde(rename = "remind_work_sec", default)]
    pub remind_work_sec: i32,
    #[serde(rename = "remind_off_work_sec", default)]
    pub remind_off_work_sec: i32,
    #[serde(rename = "rest_begin_time", default)]
    pub rest_begin_time: i32,
    #[serde(rename = "rest_end_time", default)]
    pub rest_end_time: i32,
    #[serde(rename = "allow_rest", default)]
    pub allow_rest: bool,
    #[serde(rename = "earliest_work_sec", default)]
    pub earliest_work_sec: i32,
    #[serde(rename = "latest_work_sec", default)]
    pub latest_work_sec: i32,
    #[serde(rename = "earliest_off_work_sec", default)]
    pub earliest_off_work_sec: i32,
    #[serde(rename = "latest_off_work_sec", default)]
    pub latest_off_work_sec: i32,
    #[serde(rename = "no_need_checkon", default)]
    pub no_need_checkon: bool,
    #[serde(rename = "no_need_checkoff", default)]
    pub no_need_checkoff: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpeWorkday {
    #[serde(rename = "timestamp", default)]
    pub timestamp: i64,
    #[serde(rename = "notes", default)]
    pub notes: String,
    #[serde(rename = "checkintime", default)]
    pub checkin_time: Vec<crate::bean::oa::wx_cp_checkin_group_base::CheckinTime>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpeOffDay {
    #[serde(rename = "timestamp", default)]
    pub timestamp: i64,
    #[serde(rename = "notes", default)]
    pub notes: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WifiMacInfo {
    #[serde(rename = "wifiname", default)]
    pub wifiname: String,
    #[serde(rename = "wifimac", default)]
    pub wifimac: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LocInfo {
    #[serde(rename = "lat", default)]
    pub lat: i64,
    #[serde(rename = "lng", default)]
    pub lng: i64,
    #[serde(rename = "loc_title", default)]
    pub loc_title: String,
    #[serde(rename = "loc_detail", default)]
    pub loc_detail: String,
    #[serde(rename = "distance", default)]
    pub distance: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: i32,
    #[serde(rename = "schedule_name", default)]
    pub schedule_name: String,
    #[serde(rename = "time_section", default)]
    pub time_section: Vec<crate::bean::oa::wx_cp_checkin_group_base::TimeSection>,
    #[serde(rename = "limit_aheadtime", default)]
    pub limit_ahead_time: i64,
    #[serde(rename = "limit_offtime", default)]
    pub limit_off_time: i32,
    #[serde(rename = "noneed_offwork", default)]
    pub no_need_off_work: bool,
    #[serde(rename = "allow_flex", default)]
    pub allow_flex: bool,
    #[serde(rename = "flex_on_duty_time", default)]
    pub flex_on_duty_time: i32,
    #[serde(rename = "flex_off_duty_time", default)]
    pub flex_off_duty_time: i32,
    #[serde(rename = "late_rule", default)]
    pub late_rule: crate::bean::oa::wx_cp_checkin_group_base::LateRule,
    #[serde(rename = "max_allow_arrive_early", default)]
    pub max_allow_arrive_early: i32,
    #[serde(rename = "max_allow_arrive_late", default)]
    pub max_allow_arrive_late: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeSection {
    #[serde(rename = "time_id", default)]
    pub time_id: i32,
    #[serde(rename = "work_sec", default)]
    pub work_sec: i32,
    #[serde(rename = "off_work_sec", default)]
    pub off_work_sec: i32,
    #[serde(rename = "remind_work_sec", default)]
    pub remind_work_sec: i64,
    #[serde(rename = "remind_off_work_sec", default)]
    pub remind_off_work_sec: i32,
    #[serde(rename = "rest_begin_time", default)]
    pub rest_begin_time: i32,
    #[serde(rename = "rest_end_time", default)]
    pub rest_end_time: i32,
    #[serde(rename = "allow_rest", default)]
    pub allow_rest: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LateRule {
    #[serde(rename = "offwork_after_time", default)]
    pub off_work_after_time: i32,
    #[serde(rename = "onwork_flex_time", default)]
    pub on_work_flex_time: i32,
    #[serde(rename = "allow_offwork_after_time", default)]
    pub allow_off_work_after_time: bool,
    #[serde(rename = "timerules", default)]
    pub timerules: Vec<crate::bean::oa::wx_cp_checkin_group_base::TimeRule>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeRule {
    #[serde(rename = "offwork_after_time", default)]
    pub off_work_after_time: i32,
    #[serde(rename = "onwork_flex_time", default)]
    pub on_work_flex_time: i32,
}
