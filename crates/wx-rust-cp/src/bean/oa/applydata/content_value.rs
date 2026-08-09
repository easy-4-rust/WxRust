//! 对应 Java `me.chanjar.weixin.cp.bean.oa.applydata.ContentValue.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentValue {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "new_number", default)]
    pub new_number: String,
    #[serde(rename = "new_money", default)]
    pub new_money: String,
    #[serde(rename = "date", default)]
    pub date: crate::bean::oa::applydata::content_value::Date,
    #[serde(rename = "selector", default)]
    pub selector: crate::bean::oa::applydata::content_value::Selector,
    #[serde(rename = "members", default)]
    pub members: Vec<crate::bean::oa::applydata::content_value::Member>,
    #[serde(rename = "departments", default)]
    pub departments: Vec<crate::bean::oa::applydata::content_value::Department>,
    #[serde(rename = "new_tips", default)]
    pub new_tips: NewTips,
    #[serde(rename = "files", default)]
    pub files: Vec<crate::bean::oa::applydata::content_value::File>,
    #[serde(rename = "children", default)]
    pub children: Vec<crate::bean::oa::applydata::content_value::Child>,
    #[serde(rename = "related_approval", default)]
    pub related_approval: Vec<RelatedApproval>,
    #[serde(rename = "attendance", default)]
    pub attendance: Attendance,
    #[serde(rename = "vacation", default)]
    pub vacation: Vacation,
    #[serde(rename = "date_range", default)]
    pub date_range: crate::bean::oa::applydata::content_value::DataRange,
    #[serde(rename = "punch_correction", default)]
    pub punch_correction: PunchCorrection,
    #[serde(rename = "location", default)]
    pub location: Location,
    #[serde(rename = "formula", default)]
    pub formula: Formula,
    #[serde(rename = "bank_account", default)]
    pub bank_account: BankAccount,
    #[serde(rename = "phonenumber", default)]
    pub phonenumber: PhoneNumber,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PhoneNumber {
    #[serde(rename = "area", default)]
    pub area: String,
    #[serde(rename = "number", default)]
    pub number: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Date {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "s_timestamp", default)]
    pub timestamp: String,
    #[serde(rename = "timezone_info", default)]
    pub timezone_info: TimezoneInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimezoneInfo {
    #[serde(rename = "zone_offset", default)]
    pub zone_offset: String,
    #[serde(rename = "zone_desc", default)]
    pub zone_desc: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Selector {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "options", default)]
    pub options: Vec<SelectorOption>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SelectorOption {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "value", default)]
    pub values: Vec<crate::bean::oa::applydata::content_title::ContentTitle>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Member {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Department {
    #[serde(rename = "openapi_id", default)]
    pub open_api_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewTips {
    #[serde(rename = "tips_content", default)]
    pub tips_content: Vec<TipsContent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TipsContent {
    #[serde(rename = "text", default)]
    pub text: Text,
    #[serde(rename = "lang", default)]
    pub lang: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Text {
    #[serde(rename = "sub_text", default)]
    pub sub_text: Vec<crate::bean::oa::applydata::content_value::SubText>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubText {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "content", default)]
    pub content: Content,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Content {
    #[serde(rename = "plain_text", default)]
    pub plain_text: PlainText,
    #[serde(rename = "link", default)]
    pub link: Link,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlainText {
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "url", default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct File {
    #[serde(rename = "file_id", default)]
    pub file_id: String,
    #[serde(rename = "file_name", default)]
    pub file_name: String,
    #[serde(rename = "file_url", default)]
    pub file_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Child {
    #[serde(rename = "list", default)]
    pub list: Vec<crate::bean::oa::applydata::apply_data_content::ApplyDataContent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attendance {
    #[serde(rename = "date_range", default)]
    pub date_range: DataRange,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "slice_info", default)]
    pub slice_info: SliceInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataRange {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "new_begin", default)]
    pub begin: i64,
    #[serde(rename = "new_end", default)]
    pub end: i64,
    #[serde(rename = "new_duration", default)]
    pub duration: i64,
    #[serde(rename = "timezone_info", default)]
    pub timezone_info: crate::bean::oa::applydata::content_value::TimezoneInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SliceInfo {
    #[serde(rename = "day_items", default)]
    pub day_items: Vec<DayItems>,
    #[serde(rename = "duration", default)]
    pub duration: i64,
    #[serde(rename = "state", default)]
    pub state: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DayItems {
    #[serde(rename = "daytime", default)]
    pub daytime: i64,
    #[serde(rename = "duration", default)]
    pub duration: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vacation {
    #[serde(rename = "selector", default)]
    pub selector: crate::bean::oa::applydata::content_value::Selector,
    #[serde(rename = "attendance", default)]
    pub attendance: crate::bean::oa::applydata::content_value::Attendance,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RelatedApproval {
    #[serde(rename = "template_names", default)]
    pub template_names: Vec<crate::bean::oa::applydata::content_value::TemplateName>,
    #[serde(rename = "sp_status", default)]
    pub sp_status: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "sp_no", default)]
    pub sp_no: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateName {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "lang", default)]
    pub lang: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PunchCorrection {
    #[serde(rename = "state", default)]
    pub state: String,
    #[serde(rename = "time", default)]
    pub time: i64,
    #[serde(rename = "version", default)]
    pub version: i32,
    #[serde(rename = "daymonthyear", default)]
    pub day_month_year: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Location {
    #[serde(rename = "latitude", default)]
    pub latitude: String,
    #[serde(rename = "longitude", default)]
    pub longitude: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "time", default)]
    pub time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Formula {
    #[serde(rename = "value", default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankAccount {
    #[serde(rename = "account_type", default)]
    pub account_type: i64,
    #[serde(rename = "account_name", default)]
    pub account_name: String,
    #[serde(rename = "account_number", default)]
    pub account_number: String,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "bank", default)]
    pub bank: Bank,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bank {
    #[serde(rename = "bank_alias", default)]
    pub bank_alias: String,
    #[serde(rename = "bank_alias_code", default)]
    pub bank_alias_code: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "province_code", default)]
    pub province_code: i64,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "city_code", default)]
    pub city_code: i64,
    #[serde(rename = "bank_branch_name", default)]
    pub bank_branch_name: String,
    #[serde(rename = "bank_branch_id", default)]
    pub bank_branch_id: String,
}
