//! 提现通知信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.fund.WithdrawNotifyInfo.java`。

use serde::{Deserialize, Serialize};

/// 提现通知信息（对应 Java `WithdrawNotifyInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WithdrawNotifyInfo {
    /// 1.发起提现，生成二维码 2.扫码验证成功，申请提现 3.提现成功 4.提现失败
    /// （对应 Java `event`）。
    #[serde(
        rename = "event",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub event: Option<i32>,
    /// 提现单号（对应 Java `withdrawId`）。
    #[serde(rename = "withdraw_id", default)]
    pub withdraw_id: Option<String>,
}
