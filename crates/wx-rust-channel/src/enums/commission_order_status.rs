//! 视频号小店 枚举（对应 Java `CommissionOrderStatus`）。

/// CommissionOrderStatus（对应 Java `me.chanjar.weixin.channel.enums.CommissionOrderStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommissionOrderStatus {
    /// 20 未结算
    NotSettled,
    /// 100 已结算
    Settled,
    /// 200 取消结算
    CancelSettled,
}

impl CommissionOrderStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            CommissionOrderStatus::NotSettled => 20,
            CommissionOrderStatus::Settled => 100,
            CommissionOrderStatus::CancelSettled => 200,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            CommissionOrderStatus::NotSettled => "未结算",
            CommissionOrderStatus::Settled => "已结算",
            CommissionOrderStatus::CancelSettled => "取消结算",
        }
    }
}
