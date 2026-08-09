//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取电商数据概览（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_OVERALL_URL`）。
pub const GET_SHOP_OVERALL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/overall/get";

/// 获取授权视频号列表（对应 Java `WxChannelApiUrlConstants` 常量 `FINDER_AUTH_LIST_URL`）。
pub const FINDER_AUTH_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/finder/authorization/list/get";

/// 获取带货达人列表（对应 Java `WxChannelApiUrlConstants` 常量 `FINDER_LIST_URL`）。
pub const FINDER_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/finder/list/get";

/// 获取带货数据概览（对应 Java `WxChannelApiUrlConstants` 常量 `GET_FINDER_OVERALL_URL`）。
pub const GET_FINDER_OVERALL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/finder/overall/get";

/// 获取带货达人商品列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_FINDER_PRODUCT_LIST_URL`）。
pub const GET_FINDER_PRODUCT_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/finder/product/list/get";

/// 获取带货达人商品数据（对应 Java `WxChannelApiUrlConstants` 常量 `GET_FINDER_PRODUCT_OVERALL_URL`）。
pub const GET_FINDER_PRODUCT_OVERALL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/finder/product/overall/get";

/// 获取店铺开播列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LIVE_LIST_URL`）。
pub const GET_LIVE_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/live/list/get";

/// 获取商品详细信息（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_PRODUCT_DATA_URL`）。
pub const GET_SHOP_PRODUCT_DATA_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/product/data/get";

/// 获取商品列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_PRODUCT_LIST_URL`）。
pub const GET_SHOP_PRODUCT_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/product/list/get";

/// 获取店铺人群数据（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_SALE_PROFILE_DATA_URL`）。
pub const GET_SHOP_SALE_PROFILE_DATA_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/shop/sale/profile/data/get";
