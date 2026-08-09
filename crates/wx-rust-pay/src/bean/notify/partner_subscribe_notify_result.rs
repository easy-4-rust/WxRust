//! 对应 Java `com.github.binarywang.wxpay.bean.notify.PartnerSubscribeNotifyResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerSubscribeNotifyResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawData")]
    pub raw_data: Option<OriginNotifyResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<DecryptNotifyResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecryptNotifyResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "message_content"
    )]
    pub message_content: Option<MessageContent>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "topic_name"
    )]
    pub topic_name: Option<TopicName>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageContent {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_code"
    )]
    pub merchant_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_company_name"
    )]
    pub merchant_company_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_time"
    )]
    pub business_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_code"
    )]
    pub business_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_state"
    )]
    pub business_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remark")]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopicName {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "topic_english_name"
    )]
    pub topic_english_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "topic_chinese_name"
    )]
    pub topic_chinese_name: Option<String>,
}
