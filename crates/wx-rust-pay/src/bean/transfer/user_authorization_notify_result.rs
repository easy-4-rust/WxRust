//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.UserAuthorizationNotifyResult.java`。
//!
//! 免确认收款授权结果通知。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::notify::OriginNotifyResponse;

/// 免确认收款授权结果通知（对应 Java `UserAuthorizationNotifyResult`）。
///
/// 微信支付会把授权确认或授权关闭结果发送到商户在发起授权时传入的
/// `authorization_notify_url`，商户可通过该通知保存 `authorization_id`
/// 并用于后续用户授权后转账。
///
/// 对应 Java: `UserAuthorizationNotifyResult`
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserAuthorizationNotifyResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawData")]
    pub raw_data: Option<OriginNotifyResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<UserAuthorizationDecryptNotifyResult>,
}

/// 免确认收款授权通知解密结果（对应 Java `UserAuthorizationNotifyResult.DecryptNotifyResult`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserAuthorizationDecryptNotifyResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_authorization_no"
    )]
    pub out_authorization_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_display_name"
    )]
    pub user_display_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_id"
    )]
    pub authorization_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "state")]
    pub state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorize_time"
    )]
    pub authorize_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "close_reason"
    )]
    pub close_reason: Option<String>,
}
