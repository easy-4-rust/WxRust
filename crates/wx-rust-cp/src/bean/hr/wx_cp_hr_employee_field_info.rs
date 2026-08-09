//! 对应 Java `me.chanjar.weixin.cp.bean.hr.WxCpHrEmployeeFieldInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpHrEmployeeFieldInfo {
    #[serde(rename = "fieldid", default)]
    pub field_id: i32,
    #[serde(rename = "field_name", default)]
    pub field_name: String,
    #[serde(rename = "field_type", default)]
    pub field_type: i32,
    #[serde(rename = "is_must", default)]
    pub is_must: bool,
    #[serde(rename = "value_type", default)]
    pub value_type: i32,
    #[serde(rename = "option_list", default)]
    pub option_list: Vec<WxCpHrEmployeeFieldInfoOption>,
    #[serde(rename = "field_key", default)]
    pub field_key: String,
    #[serde(rename = "field_en_name", default)]
    pub field_en_name: String,
    #[serde(rename = "field_zh_name", default)]
    pub field_zh_name: String,
    #[serde(rename = "is_sys", default)]
    pub is_sys: i32,
    #[serde(rename = "field_detail", default)]
    pub field_detail: FieldDetail,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpHrEmployeeFieldInfoOption {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "value", default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FieldDetail {
    #[serde(rename = "option_list", default)]
    pub option_list: Vec<crate::bean::hr::wx_cp_hr_employee_field_info::OldOption>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OldOption {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "value", default)]
    pub value: String,
}
