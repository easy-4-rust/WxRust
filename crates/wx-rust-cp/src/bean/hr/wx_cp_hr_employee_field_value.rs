//! 对应 Java `me.chanjar.weixin.cp.bean.hr.WxCpHrEmployeeFieldValue.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpHrEmployeeFieldValue {
    #[serde(rename = "text_value", default)]
    pub text_value: String,
    #[serde(rename = "option_value", default)]
    pub option_value: OptionValue,
    #[serde(rename = "option_value_list", default)]
    pub option_value_list: Vec<OptionValue>,
    #[serde(rename = "date_value", default)]
    pub date_value: DateValue,
    #[serde(rename = "attachment_value", default)]
    pub attachment_value: AttachmentValue,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OptionValue {
    #[serde(rename = "key", default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DateValue {
    #[serde(rename = "timestamp", default)]
    pub timestamp: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AttachmentValue {
    #[serde(rename = "id_list", default)]
    pub id_list: Vec<String>,
}
