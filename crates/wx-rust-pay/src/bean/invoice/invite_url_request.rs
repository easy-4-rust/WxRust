//! 获取服务商电子发票能力邀请链接请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InviteUrlRequest`。

use serde::{Deserialize, Serialize};

/// 获取服务商电子发票能力邀请链接请求。
///
/// 对应 Java: `InviteUrlRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteUrlRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 操作类型。
    #[serde(rename = "operation_type", skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,

    /// 开票模式。
    #[serde(rename = "fapiao_mode", skip_serializing_if = "Option::is_none")]
    pub fapiao_mode: Option<String>,

    /// 开票能力类型列表。
    #[serde(
        rename = "fapiao_ability_type_list",
        skip_serializing_if = "Option::is_none"
    )]
    pub fapiao_ability_type_list: Option<Vec<String>>,

    /// 邀请渠道。
    #[serde(rename = "invite_channel", skip_serializing_if = "Option::is_none")]
    pub invite_channel: Option<String>,

    /// 操作用户。
    #[serde(rename = "operate_user", skip_serializing_if = "Option::is_none")]
    pub operate_user: Option<String>,

    /// 邀请码。
    #[serde(rename = "invite_code", skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
}
