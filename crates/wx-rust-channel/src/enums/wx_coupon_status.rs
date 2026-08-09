//! 视频号小店 枚举（对应 Java `WxCouponStatus`）。

/// WxCouponStatus（对应 Java `me.chanjar.weixin.channel.enums.WxCouponStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WxCouponStatus {
    /// 1 初始
    Init,
    /// 2 生效
    Valid,
    /// 4 已作废
    Invalid,
    /// 5 删除
    Delete,
}

impl WxCouponStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            WxCouponStatus::Init => 1,
            WxCouponStatus::Valid => 2,
            WxCouponStatus::Invalid => 4,
            WxCouponStatus::Delete => 5,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            WxCouponStatus::Init => "初始",
            WxCouponStatus::Valid => "生效",
            WxCouponStatus::Invalid => "已作废",
            WxCouponStatus::Delete => "删除",
        }
    }
}
