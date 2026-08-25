//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.UserConfirmAuthorizationResult.java`。
//!
//! 免确认收款授权响应结果。

#[allow(unused_imports)]
use super::*;

/// 免确认收款授权响应结果（对应 Java `UserConfirmAuthorizationResult`）。
///
/// 发起授权、查询授权和解除授权接口返回的都是同一类授权实体，各接口返回字段会略有差异。
///
/// 对应 Java: `UserConfirmAuthorizationResult`
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserConfirmAuthorizationResult {
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
        rename = "close_info"
    )]
    pub close_info: Option<AuthorizationCloseInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_scene_id"
    )]
    pub transfer_scene_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_recv_perception"
    )]
    pub user_recv_perception: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "package_info"
    )]
    pub package_info: Option<String>,
}

/// 授权关闭信息（对应 Java `UserConfirmAuthorizationResult.CloseInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthorizationCloseInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "close_time"
    )]
    pub close_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "close_reason"
    )]
    pub close_reason: Option<String>,
}
