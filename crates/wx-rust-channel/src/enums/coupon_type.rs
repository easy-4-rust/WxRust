//! 视频号小店 枚举（对应 Java `CouponType`）。

/// CouponType（对应 Java `me.chanjar.weixin.channel.enums.CouponType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CouponType {
    /// 1 商品条件折券
    C1,
    /// 2 商品满减券
    C2,
    /// 3 商品统一折扣券
    C3,
    /// 4 商品直减券
    C4,
    /// 101 店铺条件折扣券
    C101,
    /// 102 店铺满减券
    C102,
    /// 103 店铺统一折扣券
    C103,
    /// 104 店铺直减券
    C104,
}

impl CouponType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            CouponType::C1 => 1,
            CouponType::C2 => 2,
            CouponType::C3 => 3,
            CouponType::C4 => 4,
            CouponType::C101 => 101,
            CouponType::C102 => 102,
            CouponType::C103 => 103,
            CouponType::C104 => 104,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            CouponType::C1 => "商品条件折券",
            CouponType::C2 => "商品满减券",
            CouponType::C3 => "商品统一折扣券",
            CouponType::C4 => "商品直减券",
            CouponType::C101 => "店铺条件折扣券",
            CouponType::C102 => "店铺满减券",
            CouponType::C103 => "店铺统一折扣券",
            CouponType::C104 => "店铺直减券",
        }
    }
}
