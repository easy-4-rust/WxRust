//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.payroll.AuthRecordResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthRecordResult {
    #[serde(default, rename = "data")]
    pub data_list: Vec<RecordData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecordData {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_scene"
    )]
    pub authenticate_scene: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_source"
    )]
    pub authenticate_source: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "project_name"
    )]
    pub project_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "employer_name"
    )]
    pub employer_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_state"
    )]
    pub authenticate_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_time"
    )]
    pub authenticate_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_number"
    )]
    pub authenticate_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
}
