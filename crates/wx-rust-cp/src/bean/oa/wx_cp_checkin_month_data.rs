//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpCheckinMonthData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCheckinMonthData {
    #[serde(rename = "base_info", default)]
    pub base_info: BaseInfo,
    #[serde(rename = "summary_info", default)]
    pub summary_info: SummaryInfo,
    #[serde(rename = "exception_infos", default)]
    pub exception_infos: Vec<ExceptionInfo>,
    #[serde(rename = "sp_items", default)]
    pub sp_items: Vec<SpItem>,
    #[serde(rename = "overwork_info", default)]
    pub overwork_info: OverWorkInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BaseInfo {
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
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RuleInfo {
    #[serde(rename = "groupid", default)]
    pub group_id: i32,
    #[serde(rename = "groupname", default)]
    pub group_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SummaryInfo {
    #[serde(rename = "work_days", default)]
    pub work_days: i32,
    #[serde(rename = "regular_days", default)]
    pub regular_days: i32,
    #[serde(rename = "except_days", default)]
    pub except_days: i32,
    #[serde(rename = "regular_work_sec", default)]
    pub regular_work_sec: i32,
    #[serde(rename = "standard_work_sec", default)]
    pub standard_work_sec: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExceptionInfo {
    #[serde(rename = "exception", default)]
    pub exception: i32,
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "duration", default)]
    pub duration: i32,
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

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OverWorkInfo {
    #[serde(rename = "workday_over_sec", default)]
    pub workday_over_sec: i32,
    #[serde(rename = "holidays_over_sec", default)]
    pub holidays_over_sec: i32,
    #[serde(rename = "restdays_over_sec", default)]
    pub restdays_over_sec: i32,
}
