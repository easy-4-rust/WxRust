//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpCheckinDayData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCheckinDayData {
    #[serde(rename = "base_info", default)]
    pub base_info: BaseInfo,
    #[serde(rename = "summary_info", default)]
    pub summary_info: SummaryInfo,
    #[serde(rename = "holiday_infos", default)]
    pub holiday_infos: Vec<HolidayInfos>,
    #[serde(rename = "exception_infos", default)]
    pub exception_infos: Vec<ExceptionInfos>,
    #[serde(rename = "ot_info", default)]
    pub ot_info: OtInfo,
    #[serde(rename = "sp_items", default)]
    pub sp_items: Vec<SpItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BaseInfo {
    #[serde(rename = "date", default)]
    pub date: i32,
    #[serde(rename = "record_type", default)]
    pub record_type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "name_ex", default)]
    pub name_ex: String,
    #[serde(rename = "departs_name", default)]
    pub departs_name: String,
    #[serde(rename = "acctid", default)]
    pub acct_id: String,
    #[serde(rename = "rule_info", default)]
    pub rule_info: RuleInfo,
    #[serde(rename = "day_type", default)]
    pub day_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RuleInfo {
    #[serde(rename = "groupid", default)]
    pub group_id: i32,
    #[serde(rename = "groupname", default)]
    pub group_name: String,
    #[serde(rename = "scheduleid", default)]
    pub schedule_id: i32,
    #[serde(rename = "schedulename", default)]
    pub schedule_name: String,
    #[serde(rename = "checkintime", default)]
    pub checkin_time: Vec<CheckinTime>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckinTime {
    #[serde(rename = "work_sec", default)]
    pub work_sec: i32,
    #[serde(rename = "off_work_sec", default)]
    pub off_work_sec: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SummaryInfo {
    #[serde(rename = "checkin_count", default)]
    pub checkin_count: i32,
    #[serde(rename = "regular_work_sec", default)]
    pub regular_work_sec: i32,
    #[serde(rename = "standard_work_sec", default)]
    pub standard_work_sec: i32,
    #[serde(rename = "earliest_time", default)]
    pub earliest_time: i32,
    #[serde(rename = "lastest_time", default)]
    pub lastest_time: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HolidayInfos {
    #[serde(rename = "sp_number", default)]
    pub sp_number: String,
    #[serde(rename = "sp_title", default)]
    pub sp_title: SpTitle,
    #[serde(rename = "sp_description", default)]
    pub sp_description: SpDescription,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpTitle {
    #[serde(rename = "data", default)]
    pub data: Vec<Data>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Data {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "lang", default)]
    pub lang: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpDescription {
    #[serde(rename = "data", default)]
    pub data: Vec<SpDescriptionData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpDescriptionData {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "lang", default)]
    pub lang: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExceptionInfos {
    #[serde(rename = "exception", default)]
    pub exception: i32,
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "duration", default)]
    pub duration: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtInfo {
    #[serde(rename = "ot_status", default)]
    pub ot_status: i32,
    #[serde(rename = "ot_duration", default)]
    pub ot_duration: i32,
    #[serde(rename = "exception_duration", default)]
    pub exception_duration: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpItem {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "vacation_id", default)]
    pub vacation_id: i32,
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "duration", default)]
    pub duration: i32,
    #[serde(rename = "time_type", default)]
    pub time_type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
}
