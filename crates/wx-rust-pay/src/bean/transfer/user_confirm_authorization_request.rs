//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.UserConfirmAuthorizationRequest.java`。
//!
//! 发起免确认收款授权请求参数。

#[allow(unused_imports)]
use super::*;

/// 发起免确认收款授权请求参数（对应 Java `UserConfirmAuthorizationRequest`）。
///
/// 该接口只创建免确认收款授权申请，不创建转账单。成功后返回的 `package_info`
/// 需要交给业务侧用于 JSAPI/APP 调起用户授权页面。
///
/// 对应 Java: `UserConfirmAuthorizationRequest`
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserConfirmAuthorizationRequest {
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
        rename = "transfer_scene_id"
    )]
    pub transfer_scene_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_display_name"
    )]
    pub user_display_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_recv_perception"
    )]
    pub user_recv_perception: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_notify_url"
    )]
    pub authorization_notify_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<AuthSceneInfo>,
}

/// 用户端场景信息（对应 Java `UserConfirmAuthorizationRequest.SceneInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthSceneInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "client_ip")]
    pub client_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "device_type"
    )]
    pub device_type: Option<String>,
}
