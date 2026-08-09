//! 视频号小店 枚举（对应 Java `PromoteType`）。

/// PromoteType（对应 Java `me.chanjar.weixin.channel.enums.PromoteType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromoteType {
    /// 1 小店内推广
    PromoteTypeShop,
    /// 9 会员券
    Member,
    /// 10 会员开卡礼券
    MemberCard,
}

impl PromoteType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            PromoteType::PromoteTypeShop => 1,
            PromoteType::Member => 9,
            PromoteType::MemberCard => 10,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            PromoteType::PromoteTypeShop => "小店内推广",
            PromoteType::Member => "会员券",
            PromoteType::MemberCard => "会员开卡礼券",
        }
    }
}
