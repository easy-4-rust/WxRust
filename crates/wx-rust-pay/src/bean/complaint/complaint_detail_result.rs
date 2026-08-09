//! 对应 Java `com.github.binarywang.wxpay.bean.complaint.ComplaintDetailResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintDetailResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_id"
    )]
    pub complaint_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_time"
    )]
    pub complaint_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_detail"
    )]
    pub complaint_detail: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complainted_mchid"
    )]
    pub complained_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_state"
    )]
    pub complaint_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_phone"
    )]
    pub payer_phone: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_openid"
    )]
    pub payer_openid: Option<String>,
    #[serde(default, rename = "complaint_media_list")]
    pub complaint_media_list: Vec<ComplaintMedia>,
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
    #[serde(default, rename = "complaint_order_info")]
    pub complaint_order_info: Vec<ComplaintOrder>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintOrder {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(default, rename = "service_order_info")]
    pub service_order_info: Vec<ServiceOrder>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceOrder {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "order_id")]
    pub order_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_order_no"
    )]
    pub out_order_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "state")]
    pub state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_full_refunded"
    )]
    pub complaint_full_refunded: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "incoming_user_response"
    )]
    pub incoming_user_response: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "problem_description"
    )]
    pub problem_description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_complaint_times"
    )]
    pub user_complaint_times: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "problem_type"
    )]
    pub problem_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apply_refund_amount"
    )]
    pub apply_refund_amount: Option<i32>,
    #[serde(default, rename = "user_tag_list")]
    pub user_tag_list: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "additional_info"
    )]
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdditionalInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "share_power_info"
    )]
    pub share_power_info: Option<SharePowerInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharePowerInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_time"
    )]
    pub return_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_address_info"
    )]
    pub return_address_info: Option<ReturnAddressInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReturnAddressInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_address"
    )]
    pub return_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "longitude")]
    pub longitude: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latitude")]
    pub latitude: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "is_returned_to_same_machine"
    )]
    pub is_returned_to_same_machine: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "in_platform_service"
    )]
    pub in_platform_service: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "need_immediate_service"
    )]
    pub need_immediate_service: Option<bool>,
}
