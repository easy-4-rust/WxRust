//! 对应 Java `com.github.binarywang.wxpay.bean.complaint.NegotiationHistoryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NegotiationHistoryResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(default, rename = "data")]
    pub data: Vec<NegotiationHistory>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NegotiationHistory {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_media_list"
    )]
    pub complaint_media_list: Option<ComplaintMedia>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintMedia {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "media_type"
    )]
    pub media_type: Option<String>,
    #[serde(default, rename = "media_url")]
    pub media_url: Vec<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "log_id")]
    pub log_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operator")]
    pub operator: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "operate_time"
    )]
    pub operate_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "operate_type"
    )]
    pub operate_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "operate_details"
    )]
    pub operate_details: Option<String>,
    #[serde(default, rename = "image_list")]
    pub image_list: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_appy_platform_service_reason"
    )]
    pub user_apply_platform_service_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_appy_platform_service_reason_description"
    )]
    pub user_apply_platform_service_reason_description: Option<String>,
}
