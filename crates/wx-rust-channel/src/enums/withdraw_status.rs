//! 视频号小店 枚举（对应 Java `WithdrawStatus`）。

/// WithdrawStatus（对应 Java `me.chanjar.weixin.channel.enums.WithdrawStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WithdrawStatus {
    /// 受理成功
    CreateSuccess,
    /// 提现成功
    Success,
    /// 提现失败
    Fail,
    /// 提现退票
    Refund,
    /// 关单
    Close,
    /// 业务单已创建
    Init,
}

impl WithdrawStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            WithdrawStatus::CreateSuccess => "CREATE_SUCCESS",
            WithdrawStatus::Success => "SUCCESS",
            WithdrawStatus::Fail => "FAIL",
            WithdrawStatus::Refund => "REFUND",
            WithdrawStatus::Close => "CLOSE",
            WithdrawStatus::Init => "INIT",
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            WithdrawStatus::CreateSuccess => "受理成功",
            WithdrawStatus::Success => "提现成功",
            WithdrawStatus::Fail => "提现失败",
            WithdrawStatus::Refund => "提现退票",
            WithdrawStatus::Close => "关单",
            WithdrawStatus::Init => "业务单已创建",
        }
    }
}
