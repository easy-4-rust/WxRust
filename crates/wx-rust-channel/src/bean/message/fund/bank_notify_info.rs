//! 账户信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.fund.BankNotifyInfo.java`。

use serde::{Deserialize, Serialize};

/// 账户信息（对应 Java `BankNotifyInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BankNotifyInfo {
    /// 结算账户变更事件, 1.修改结算账户（对应 Java `event`）。
    #[serde(
        rename = "event",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub event: Option<i32>,
}
