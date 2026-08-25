//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.PreTransferWithAuthorizationRequest.java`。
//!
//! 发起转账并完成免确认收款授权请求参数。

#[allow(unused_imports)]
use super::*;

/// 发起转账并完成免确认收款授权请求参数（对应 Java `PreTransferWithAuthorizationRequest`）。
///
/// 该接口和普通 `TransferBillsRequest` 一样会创建商家转账单，但额外携带
/// `authorization_info`，用于在用户确认收款时同时引导用户完成免确认收款授权。
///
/// 对应 Java: `PreTransferWithAuthorizationRequest`
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreTransferWithAuthorizationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_bill_no"
    )]
    pub out_bill_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_scene_id"
    )]
    pub transfer_scene_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "user_name")]
    pub user_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_amount"
    )]
    pub transfer_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_remark"
    )]
    pub transfer_remark: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_url"
    )]
    pub notify_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_recv_perception"
    )]
    pub user_recv_perception: Option<String>,
    #[serde(default, rename = "transfer_scene_report_infos")]
    pub transfer_scene_report_infos: Vec<PreTransferTransferSceneReportInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_info"
    )]
    pub authorization_info: Option<AuthorizationInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sponsor_mchid"
    )]
    pub sponsor_mchid: Option<String>,
}

/// 转账场景报备信息（对应 Java `PreTransferWithAuthorizationRequest.TransferSceneReportInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreTransferTransferSceneReportInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "info_type")]
    pub info_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "info_content"
    )]
    pub info_content: Option<String>,
}

/// 免确认收款授权信息（对应 Java `PreTransferWithAuthorizationRequest.AuthorizationInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthorizationInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_display_name"
    )]
    pub user_display_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_authorization_no"
    )]
    pub out_authorization_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_notify_url"
    )]
    pub authorization_notify_url: Option<String>,
}
