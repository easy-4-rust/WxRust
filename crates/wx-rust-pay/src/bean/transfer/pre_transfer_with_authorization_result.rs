//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.PreTransferWithAuthorizationResult.java`。
//!
//! 发起转账并完成免确认收款授权响应结果。

#[allow(unused_imports)]
use super::*;

/// 发起转账并完成免确认收款授权响应结果（对应 Java `PreTransferWithAuthorizationResult`）。
///
/// 对应 Java: `PreTransferWithAuthorizationResult`
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreTransferWithAuthorizationResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_bill_no"
    )]
    pub out_bill_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_bill_no"
    )]
    pub transfer_bill_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "state")]
    pub state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "package_info"
    )]
    pub package_info: Option<String>,
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
}
