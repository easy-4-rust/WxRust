//! 视频号小店收藏管理接口地址常量（对应 Java `WxChannelApiUrlConstants.Favorite`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 获取店铺收藏的人数（对应 Java `Favorite.GET_FAVORITE_COUNT`）。
pub const GET_FAVORITE_COUNT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/favorites/count/get";
