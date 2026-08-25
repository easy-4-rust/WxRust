//! 服务商电子发票开通邀请链接。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InviteUrlResult`。

use serde::{Deserialize, Serialize};

/// 服务商电子发票开通邀请链接。
///
/// 对应 Java: `InviteUrlResult`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteUrlResult {
    /// 邀请链接。
    #[serde(rename = "invite_url", skip_serializing_if = "Option::is_none")]
    pub invite_url: Option<String>,
}
