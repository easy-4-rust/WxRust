//! 卡券类型。
//!
//! 对应 Java `me.chanjar.weixin.mp.enums.WxCardType`。

/// 卡券类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WxCardType {
    /// 会员卡。
    #[default]
    MemberCard,
    /// 团购券。
    Groupon,
    /// 现金券。
    Cash,
    /// 折扣券。
    Discount,
    /// 礼品券。
    Gift,
    /// 通用券。
    GeneralCoupon,
}

impl WxCardType {
    /// 类型代码。
    pub fn code(self) -> &'static str {
        match self {
            Self::MemberCard => "MEMBER_CARD",
            Self::Groupon => "GROUPON",
            Self::Cash => "CASH",
            Self::Discount => "DISCOUNT",
            Self::Gift => "GIFT",
            Self::GeneralCoupon => "GENERAL_COUPON",
        }
    }
}
