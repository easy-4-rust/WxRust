//! 视频号小店 枚举（对应 Java `LiveDistributionSceneType`）。

/// LiveDistributionSceneType（对应 Java `me.chanjar.weixin.channel.enums.LiveDistributionSceneType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiveDistributionSceneType {
    /// 商品曝光
    ProductImpression,
    /// 直播间曝光次数
    LiveRoomImpressionPv,
    /// 商品点击次数
    ProductClickPv,
    /// 创建订单数按渠道统计
    ChannelTotalCreatePv,
    /// 成交订单数按渠道统计
    ChannelTotalPayPv,
}

impl LiveDistributionSceneType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            LiveDistributionSceneType::ProductImpression => 6,
            LiveDistributionSceneType::LiveRoomImpressionPv => 7,
            LiveDistributionSceneType::ProductClickPv => 8,
            LiveDistributionSceneType::ChannelTotalCreatePv => 9,
            LiveDistributionSceneType::ChannelTotalPayPv => 10,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            LiveDistributionSceneType::ProductImpression => "商品曝光",
            LiveDistributionSceneType::LiveRoomImpressionPv => "直播间曝光次数",
            LiveDistributionSceneType::ProductClickPv => "商品点击次数",
            LiveDistributionSceneType::ChannelTotalCreatePv => "创建订单数按渠道统计",
            LiveDistributionSceneType::ChannelTotalPayPv => "成交订单数按渠道统计",
        }
    }
}
