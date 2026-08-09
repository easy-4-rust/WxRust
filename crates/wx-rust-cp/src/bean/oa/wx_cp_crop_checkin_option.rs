//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpCropCheckinOption.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCropCheckinOption {
    #[serde(rename = "grouptype", default)]
    pub group_type: i64,
    #[serde(rename = "groupid", default)]
    pub group_id: i64,
    #[serde(rename = "groupname", default)]
    pub group_name: String,
    #[serde(rename = "checkindate", default)]
    pub checkin_date: Vec<crate::bean::oa::wx_cp_checkin_group_base::CheckinDate>,
    #[serde(rename = "spe_workdays", default)]
    pub spe_workdays: Vec<crate::bean::oa::wx_cp_checkin_group_base::SpeWorkday>,
    #[serde(rename = "spe_offdays", default)]
    pub spe_off_days: Vec<crate::bean::oa::wx_cp_checkin_group_base::SpeOffDay>,
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
    pub wifi_mac_infos: Vec<crate::bean::oa::wx_cp_checkin_group_base::WifiMacInfo>,
    #[serde(rename = "loc_infos", default)]
    pub loc_infos: Vec<crate::bean::oa::wx_cp_checkin_group_base::LocInfo>,
    #[serde(rename = "schedulelist", default)]
    pub schedulelist: Vec<crate::bean::oa::wx_cp_checkin_group_base::Schedule>,
    #[serde(rename = "range", default)]
    pub range: Range,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "white_users", default)]
    pub white_users: Vec<String>,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "reporterinfo", default)]
    pub reporter_info: ReporterInfo,
    #[serde(rename = "ot_info", default)]
    pub ot_info: OtInfo,
    #[serde(rename = "ot_info_v2", default)]
    pub ot_info_v2: OtInfoV2,
    #[serde(rename = "allow_apply_bk_cnt", default)]
    pub allow_apply_bk_cnt: i32,
    #[serde(rename = "option_out_range", default)]
    pub option_out_range: i32,
    #[serde(rename = "create_userid", default)]
    pub create_userid: String,
    #[serde(rename = "use_face_detect", default)]
    pub use_face_detect: bool,
    #[serde(rename = "allow_apply_bk_day_limit", default)]
    pub allow_apply_bk_day_limit: i32,
    #[serde(rename = "update_userid", default)]
    pub update_userid: String,
    #[serde(rename = "offwork_interval_time", default)]
    pub off_work_interval_time: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Range {
    #[serde(rename = "party_id", default)]
    pub partyid: Vec<String>,
    #[serde(rename = "userid", default)]
    pub userid: Vec<String>,
    #[serde(rename = "tagid", default)]
    pub tagid: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReporterInfo {
    #[serde(rename = "reporters", default)]
    pub reporters: Vec<crate::bean::oa::wx_cp_crop_checkin_option::Reporter>,
    #[serde(rename = "updatetime", default)]
    pub update_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reporter {
    #[serde(rename = "userid", default)]
    pub userid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "allow_ot_workingday", default)]
    pub allow_ot_working_day: bool,
    #[serde(rename = "allow_ot_nonworkingday", default)]
    pub allow_ot_nonworking_day: bool,
    #[serde(rename = "otcheckinfo", default)]
    pub otcheckinfo: crate::bean::oa::wx_cp_crop_checkin_option::OtCheckInfo,
    #[serde(rename = "uptime", default)]
    pub uptime: i64,
    #[serde(rename = "otapplyinfo", default)]
    pub otapplyinfo: crate::bean::oa::wx_cp_crop_checkin_option::OtApplyInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtCheckInfo {
    #[serde(rename = "ot_workingday_time_start", default)]
    pub ot_working_day_time_start: i32,
    #[serde(rename = "ot_workingday_time_min", default)]
    pub ot_working_day_time_min: i32,
    #[serde(rename = "ot_workingday_time_max", default)]
    pub ot_working_day_time_max: i32,
    #[serde(rename = "ot_nonworkingday_time_min", default)]
    pub ot_nonworking_day_time_min: i32,
    #[serde(rename = "ot_nonworkingday_time_max", default)]
    pub ot_nonworking_day_time_max: i32,
    #[serde(rename = "ot_nonworkingday_spanday_time", default)]
    pub ot_nonworking_day_span_day_time: i32,
    #[serde(rename = "ot_workingday_restinfo", default)]
    pub ot_workingday_restinfo: crate::bean::oa::wx_cp_crop_checkin_option::OtWorkingDayRestInfo,
    #[serde(rename = "ot_nonworkingday_restinfo", default)]
    pub ot_nonworkingday_restinfo:
        crate::bean::oa::wx_cp_crop_checkin_option::OtNonworkingDayRestInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtWorkingDayRestInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "fix_time_rule", default)]
    pub fix_time_rule: crate::bean::oa::wx_cp_crop_checkin_option::FixTimeRule,
    #[serde(rename = "cal_ottime_rule", default)]
    pub cal_ottime_rule: crate::bean::oa::wx_cp_crop_checkin_option::CalOtTimeRule,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FixTimeRule {
    #[serde(rename = "fix_time_begin_sec", default)]
    pub fix_time_begin_sec: i32,
    #[serde(rename = "fix_time_end_sec", default)]
    pub fix_time_end_sec: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CalOtTimeRule {
    #[serde(rename = "items", default)]
    pub items: Vec<crate::bean::oa::wx_cp_crop_checkin_option::Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "ot_time", default)]
    pub ot_time: i32,
    #[serde(rename = "rest_time", default)]
    pub rest_time: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtNonworkingDayRestInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "fix_time_rule", default)]
    pub fix_time_rule: crate::bean::oa::wx_cp_crop_checkin_option::FixTimeRule,
    #[serde(rename = "cal_ottime_rule", default)]
    pub cal_ottime_rule: crate::bean::oa::wx_cp_crop_checkin_option::CalOtTimeRule,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtApplyInfo {
    #[serde(rename = "allow_ot_workingday", default)]
    pub allow_ot_working_day: bool,
    #[serde(rename = "allow_ot_nonworkingday", default)]
    pub allow_ot_nonworking_day: bool,
    #[serde(rename = "uptime", default)]
    pub uptime: i64,
    #[serde(rename = "ot_workingday_restinfo", default)]
    pub ot_workingday_restinfo: crate::bean::oa::wx_cp_crop_checkin_option::OtWorkingDayRestInfo,
    #[serde(rename = "ot_nonworkingday_restinfo", default)]
    pub ot_nonworkingday_restinfo:
        crate::bean::oa::wx_cp_crop_checkin_option::OtNonworkingDayRestInfo,
    #[serde(rename = "ot_nonworkingday_spanday_time", default)]
    pub ot_nonworking_day_span_day_time: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtInfoV2 {
    #[serde(rename = "workdayconf", default)]
    pub workday_conf: WorkdayConf,
    #[serde(rename = "restdayconf", default)]
    pub restday_conf: RestdayConf,
    #[serde(rename = "holidayconf", default)]
    pub holiday_conf: HolidayConf,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkdayConf {
    #[serde(rename = "allow_ot", default)]
    pub allow_ot: bool,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RestdayConf {
    #[serde(rename = "allow_ot", default)]
    pub allow_ot: bool,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HolidayConf {
    #[serde(rename = "allow_ot", default)]
    pub allow_ot: bool,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}
