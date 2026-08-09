//! 视频号小店 枚举（对应 Java `OrderScene`）。

/// OrderScene（对应 Java `me.chanjar.weixin.channel.enums.OrderScene`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderScene {
    /// 其他
    Other,
    /// 直播间下单
    Live,
    /// 短视频
    Video,
    /// 商品分享
    Share,
    /// 商品橱窗主页
    ShowCase,
    /// 公众号文章商品卡片
    ArticleCard,
}

impl OrderScene {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            OrderScene::Other => 1,
            OrderScene::Live => 2,
            OrderScene::Video => 3,
            OrderScene::Share => 4,
            OrderScene::ShowCase => 5,
            OrderScene::ArticleCard => 6,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            OrderScene::Other => "其他",
            OrderScene::Live => "直播间",
            OrderScene::Video => "短视频",
            OrderScene::Share => "商品分享",
            OrderScene::ShowCase => "商品橱窗主页",
            OrderScene::ArticleCard => "公众号文章商品卡片",
        }
    }
}
