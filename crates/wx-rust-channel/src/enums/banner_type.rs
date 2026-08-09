//! 视频号小店 枚举（对应 Java `BannerType`）。

/// BannerType（对应 Java `me.chanjar.weixin.channel.enums.BannerType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BannerType {
    /// 1 商品
    Product,
    /// 3 视频号
    Channel,
    /// 4 公众号
    Mp,
}

impl BannerType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            BannerType::Product => 1,
            BannerType::Channel => 3,
            BannerType::Mp => 4,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            BannerType::Product => "商品",
            BannerType::Channel => "视频号",
            BannerType::Mp => "公众号",
        }
    }
}
