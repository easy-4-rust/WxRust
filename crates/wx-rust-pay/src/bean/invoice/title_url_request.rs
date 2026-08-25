//! 获取用户抬头填写链接请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.TitleUrlRequest`。

use serde::{Deserialize, Serialize};

/// 获取用户抬头填写链接请求。
///
/// 对应 Java: `TitleUrlRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TitleUrlRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 开票申请单号。
    #[serde(rename = "fapiao_apply_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_apply_id: Option<String>,

    /// 来源。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// AppID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,

    /// 用户 OpenID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,

    /// 总金额。
    #[serde(rename = "total_amount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,

    /// 卖家名称。
    #[serde(rename = "seller_name", skip_serializing_if = "Option::is_none")]
    pub seller_name: Option<String>,

    /// 是否展示手机号输入框。
    #[serde(rename = "show_phone_cell", skip_serializing_if = "Option::is_none")]
    pub show_phone_cell: Option<bool>,

    /// 是否必须输入手机号。
    #[serde(rename = "must_input_phone", skip_serializing_if = "Option::is_none")]
    pub must_input_phone: Option<bool>,

    /// 是否展示邮箱输入框。
    #[serde(rename = "show_email_cell", skip_serializing_if = "Option::is_none")]
    pub show_email_cell: Option<bool>,

    /// 是否必须输入邮箱。
    #[serde(rename = "must_input_email", skip_serializing_if = "Option::is_none")]
    pub must_input_email: Option<bool>,
}
