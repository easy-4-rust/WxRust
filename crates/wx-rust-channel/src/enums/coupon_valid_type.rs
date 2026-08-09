//! 视频号小店 枚举（对应 Java `CouponValidType`）。

/// CouponValidType（对应 Java `me.chanjar.weixin.channel.enums.CouponValidType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CouponValidType {
    /// 指定时间范围生效
    CouponValidTypeTime,
    /// 生效天数
    CouponValidTypeDay,
}

impl CouponValidType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            CouponValidType::CouponValidTypeTime => 1,
            CouponValidType::CouponValidTypeDay => 2,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            CouponValidType::CouponValidTypeTime => "指定时间范围生效",
            CouponValidType::CouponValidTypeDay => "生效天数",
        }
    }
}
