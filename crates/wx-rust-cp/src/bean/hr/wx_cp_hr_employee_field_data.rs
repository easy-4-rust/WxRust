//! 对应 Java `me.chanjar.weixin.cp.bean.hr.WxCpHrEmployeeFieldData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpHrEmployeeFieldData {
    #[serde(rename = "fieldid", default)]
    pub field_id: i32,
    #[serde(rename = "sub_idx", default)]
    pub sub_idx: i32,
    #[serde(rename = "result", default)]
    pub result: i32,
    #[serde(rename = "value_type", default)]
    pub value_type: i32,
    #[serde(rename = "value_string", default)]
    pub value_string: String,
    #[serde(rename = "value_uint32", default)]
    pub value_uint32: i64,
    #[serde(rename = "value_int64", default)]
    pub value_int64: i64,
    #[serde(rename = "value_uint64", default)]
    pub value_uint64: i64,
    #[serde(rename = "value_mobile", default)]
    pub value_mobile: MobileValue,
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "field_list", default)]
    pub field_list: Vec<FieldItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MobileValue {
    #[serde(rename = "value_country_code", default)]
    pub value_country_code: String,
    #[serde(rename = "value_mobile", default)]
    pub value_mobile: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FieldItem {
    #[serde(rename = "fieldid", default)]
    pub field_id: i32,
    #[serde(rename = "field_value", default)]
    pub field_value: crate::bean::hr::wx_cp_hr_employee_field_value::WxCpHrEmployeeFieldValue,
    #[serde(rename = "value_string", default)]
    pub value_string: String,
}
