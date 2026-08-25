//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.TransferBillsAfterAuthorizationRequest.java`。
//!
//! 用户授权后转账请求参数。

#[allow(unused_imports)]
use super::*;

/// 用户授权后转账请求参数（对应 Java `TransferBillsAfterAuthorizationRequest`）。
///
/// 该接口用于给已经完成免确认收款授权的用户发起转账。请求中不再传 openid，
/// 而是通过微信免确认收款授权单号或商户侧授权单号定位已授权用户。
///
/// 对应 Java: `TransferBillsAfterAuthorizationRequest`
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferBillsAfterAuthorizationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_bill_no"
    )]
    pub out_bill_no: Option<String>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_scene_id"
    )]
    pub transfer_scene_id: Option<String>,
    #[serde(default, rename = "transfer_scene_report_infos")]
    pub transfer_scene_report_infos: Vec<AfterAuthTransferSceneReportInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_id"
    )]
    pub authorization_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sponsor_mchid"
    )]
    pub sponsor_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_authorization_no"
    )]
    pub out_authorization_no: Option<String>,
}

/// 转账场景报备信息（对应 Java `TransferBillsAfterAuthorizationRequest.TransferSceneReportInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterAuthTransferSceneReportInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "info_type")]
    pub info_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "info_content"
    )]
    pub info_content: Option<String>,
}
