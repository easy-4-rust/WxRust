//! 视频号小店 枚举（对应 Java `SaleProfileUserType`）。

/// SaleProfileUserType（对应 Java `me.chanjar.weixin.channel.enums.SaleProfileUserType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaleProfileUserType {
    /// 商品曝光用户
    ProductImpressionUser,
    /// 商品点击用户
    ProductClickUser,
    /// 购买用户
    PurchasingUser,
    /// 首购用户
    FirstPurchaseUser,
    /// 复购用户
    RepurchaseUser,
    /// 直播观看用户
    LiveWatcherUser,
}

impl SaleProfileUserType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            SaleProfileUserType::ProductImpressionUser => 1,
            SaleProfileUserType::ProductClickUser => 2,
            SaleProfileUserType::PurchasingUser => 3,
            SaleProfileUserType::FirstPurchaseUser => 4,
            SaleProfileUserType::RepurchaseUser => 5,
            SaleProfileUserType::LiveWatcherUser => 6,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            SaleProfileUserType::ProductImpressionUser => "商品曝光用户",
            SaleProfileUserType::ProductClickUser => "商品点击用户",
            SaleProfileUserType::PurchasingUser => "购买用户",
            SaleProfileUserType::FirstPurchaseUser => "首购用户",
            SaleProfileUserType::RepurchaseUser => "复购用户",
            SaleProfileUserType::LiveWatcherUser => "直播观看用户",
        }
    }
}
