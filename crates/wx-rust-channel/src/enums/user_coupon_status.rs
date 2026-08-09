//! 视频号小店 枚举（对应 Java `UserCouponStatus`）。

/// UserCouponStatus（对应 Java `me.chanjar.weixin.channel.enums.UserCouponStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserCouponStatus {
    /// 100 生效中
    Valid,
    /// 101 已过期
    Expired,
    /// 102 已使用
    Used,
}

impl UserCouponStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            UserCouponStatus::Valid => 100,
            UserCouponStatus::Expired => 101,
            UserCouponStatus::Used => 102,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            UserCouponStatus::Valid => "生效中",
            UserCouponStatus::Expired => "已过期",
            UserCouponStatus::Used => "已使用",
        }
    }
}
