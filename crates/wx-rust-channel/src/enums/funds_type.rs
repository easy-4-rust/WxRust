//! 视频号小店 枚举（对应 Java `FundsType`）。

/// FundsType（对应 Java `me.chanjar.weixin.channel.enums.FundsType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundsType {
    /// 1 订单支付收入
    OrderPayIncome,
    /// 2 订单手续费
    OrderFee,
    /// 3 退款
    Refund,
    /// 4 提现
    Withdraw,
    /// 5 提现失败退票
    WithdrawFail,
    /// 6 导购分账
    GuideShare,
    /// 7 联盟分账
    LeagueShare,
    /// 8 运费险分账
    FreightShare,
    /// 9 联盟平台抽佣
    LeaguePlatCommission,
    /// 10 联盟抽佣
    LeagueCommission,
    /// 11台抽佣
    PlatformCommission,
    /// 12 团长抽佣
    LeaderCommission,
    /// 13 返佣人气卡
    PopularityCard,
    /// 14 极速退款垫资金
    FastRefund,
    /// 15 极速退款垫资回补
    FastRefundReplenishment,
    /// 16 运费险
    FreightInsurance,
    /// 99 分账
    Share,
}

impl FundsType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            FundsType::OrderPayIncome => 1,
            FundsType::OrderFee => 2,
            FundsType::Refund => 3,
            FundsType::Withdraw => 4,
            FundsType::WithdrawFail => 5,
            FundsType::GuideShare => 6,
            FundsType::LeagueShare => 7,
            FundsType::FreightShare => 8,
            FundsType::LeaguePlatCommission => 9,
            FundsType::LeagueCommission => 10,
            FundsType::PlatformCommission => 11,
            FundsType::LeaderCommission => 12,
            FundsType::PopularityCard => 13,
            FundsType::FastRefund => 14,
            FundsType::FastRefundReplenishment => 15,
            FundsType::FreightInsurance => 16,
            FundsType::Share => 99,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            FundsType::OrderPayIncome => "订单支付收入",
            FundsType::OrderFee => "订单手续费",
            FundsType::Refund => "退款",
            FundsType::Withdraw => "提现",
            FundsType::WithdrawFail => "提现失败退票",
            FundsType::GuideShare => "导购分账",
            FundsType::LeagueShare => "联盟分账",
            FundsType::FreightShare => "运费险分账",
            FundsType::LeaguePlatCommission => "联盟平台抽佣",
            FundsType::LeagueCommission => "联盟抽佣",
            FundsType::PlatformCommission => "平台抽佣",
            FundsType::LeaderCommission => "团长抽佣",
            FundsType::PopularityCard => "返佣人气卡",
            FundsType::FastRefund => "极速退款垫资金",
            FundsType::FastRefundReplenishment => "极速退款垫资回补",
            FundsType::FreightInsurance => "运费险",
            FundsType::Share => "分账",
        }
    }
}
