//! 对应 Java `com.github.binarywang.wxpay.bean.notify.MerchantViolationNotifyResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerchantViolationNotifyResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawData")]
    pub raw_data: Option<OriginNotifyResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<DecryptNotifyResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecryptNotifyResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "company_name"
    )]
    pub company_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "record_id")]
    pub record_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "punish_plan"
    )]
    pub punish_plan: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "punish_time"
    )]
    pub punish_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "punish_description"
    )]
    pub punish_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "risk_type")]
    pub risk_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "risk_description"
    )]
    pub risk_description: Option<String>,
}
