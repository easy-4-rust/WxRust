//! 提现二维码回调 信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.fund.QrNotifyInfo.java`。

use serde::{Deserialize, Serialize};

/// 提现二维码回调 信息（对应 Java `QrNotifyInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QrNotifyInfo {
    /// 二维码ticket（对应 Java `ticket`）。
    #[serde(rename = "ticket", default)]
    pub ticket: Option<String>,
    /// 二维码状态,1.已确认 2.已取消 3.已失效 4.已扫码（对应 Java `status`）。
    #[serde(
        rename = "status",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub status: Option<i32>,
    /// 扫码者身份, 0.非管理员 1.管理员（对应 Java `scanUserType`）。
    #[serde(
        rename = "scan_user_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub scan_user_type: Option<i32>,
}
