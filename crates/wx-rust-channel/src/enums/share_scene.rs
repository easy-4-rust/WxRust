//! 视频号小店 枚举（对应 Java `ShareScene`）。

/// ShareScene（对应 Java `me.chanjar.weixin.channel.enums.ShareScene`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShareScene {
    /// 1 直播间
    LiveRoom,
    /// 2 橱窗
    Window,
    /// 3 短视频
    ShortVideo,
    /// 4 视频号主页
    ChannelHome,
    /// 5 商品详情页
    ProductDetail,
    /// 6 带商品的公众号文章
    MpArticle,
    /// 7 商品链接
    ProductLink,
    /// 8 商品二维码
    ProductQrCode,
    /// 9 商品口令
    ProductTagLink,
    /// 12 视频号橱窗链接
    WindowLink,
    /// 13 视频号橱窗二维码
    WindowQrCode,
}

impl ShareScene {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            ShareScene::LiveRoom => 1,
            ShareScene::Window => 2,
            ShareScene::ShortVideo => 3,
            ShareScene::ChannelHome => 4,
            ShareScene::ProductDetail => 5,
            ShareScene::MpArticle => 6,
            ShareScene::ProductLink => 7,
            ShareScene::ProductQrCode => 8,
            ShareScene::ProductTagLink => 9,
            ShareScene::WindowLink => 12,
            ShareScene::WindowQrCode => 13,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            ShareScene::LiveRoom => "直播间",
            ShareScene::Window => "橱窗",
            ShareScene::ShortVideo => "短视频",
            ShareScene::ChannelHome => "视频号主页",
            ShareScene::ProductDetail => "商品详情页",
            ShareScene::MpArticle => "带商品的公众号文章",
            ShareScene::ProductLink => "商品链接",
            ShareScene::ProductQrCode => "商品二维码",
            ShareScene::ProductTagLink => "商品口令",
            ShareScene::WindowLink => "视频号橱窗链接",
            ShareScene::WindowQrCode => "视频号橱窗二维码",
        }
    }
}
