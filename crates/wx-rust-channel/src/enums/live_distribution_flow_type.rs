//! 视频号小店 枚举（对应 Java `LiveDistributionFlowType`）。

/// LiveDistributionFlowType（对应 Java `me.chanjar.weixin.channel.enums.LiveDistributionFlowType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiveDistributionFlowType {
    /// 无效值
    Invalid,
    /// 自然流量
    Natural,
    /// 加热流量
    Promote,
    /// 广告流量
    Ads,
    /// 公域流量
    CommonDomain,
    /// 私域流量
    PrivateDomain,
}

impl LiveDistributionFlowType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            LiveDistributionFlowType::Invalid => 0,
            LiveDistributionFlowType::Natural => 1,
            LiveDistributionFlowType::Promote => 2,
            LiveDistributionFlowType::Ads => 3,
            LiveDistributionFlowType::CommonDomain => 4,
            LiveDistributionFlowType::PrivateDomain => 5,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            LiveDistributionFlowType::Invalid => "无效值",
            LiveDistributionFlowType::Natural => "自然流量",
            LiveDistributionFlowType::Promote => "加热流量",
            LiveDistributionFlowType::Ads => "广告流量",
            LiveDistributionFlowType::CommonDomain => "公域流量",
            LiveDistributionFlowType::PrivateDomain => "私域流量",
        }
    }
}
