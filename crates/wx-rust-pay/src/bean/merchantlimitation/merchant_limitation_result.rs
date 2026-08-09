//! 对应 Java `com.github.binarywang.wxpay.bean.merchantlimitation.MerchantLimitationResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerchantLimitationResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mch_id: Option<String>,
    #[serde(default, rename = "limited_functions")]
    pub limited_functions: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "other_limited_functions"
    )]
    pub other_limited_functions: Option<String>,
    #[serde(default, rename = "recovery_specifications")]
    pub recovery_specifications: Vec<RecoverySpecification>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecoverySpecification {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limitation_case_id"
    )]
    pub limitation_case_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limitation_reason_type"
    )]
    pub limitation_reason_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limitation_reason"
    )]
    pub limitation_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limitation_reason_describe"
    )]
    pub limitation_reason_describe: Option<String>,
    #[serde(default, rename = "relate_limitations")]
    pub relate_limitations: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "other_relate_limitations"
    )]
    pub other_relate_limitations: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "recover_way"
    )]
    pub recover_way: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "recover_way_param"
    )]
    pub recover_way_param: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "recover_help_url"
    )]
    pub recover_help_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limitation_action_type"
    )]
    pub limitation_action_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limitation_start_date"
    )]
    pub limitation_start_date: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limitation_date"
    )]
    pub limitation_date: Option<String>,
}
