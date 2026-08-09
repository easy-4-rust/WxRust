//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 邀请分享员（对应 Java `WxChannelApiUrlConstants` 常量 `BIND_SHARER_URL`）。
pub const BIND_SHARER_URL: &str = "https://api.weixin.qq.com/channels/ec/sharer/bind";

/// 获取绑定的分享员（对应 Java `WxChannelApiUrlConstants` 常量 `SEARCH_SHARER_URL`）。
pub const SEARCH_SHARER_URL: &str = "https://api.weixin.qq.com/channels/ec/sharer/search_sharer";

/// 获取绑定的分享员列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_SHARER_URL`）。
pub const LIST_SHARER_URL: &str = "https://api.weixin.qq.com/channels/ec/sharer/get_sharer_list";

/// 获取分享员订单列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_SHARER_ORDER_URL`）。
pub const LIST_SHARER_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/sharer/get_sharer_order_list";

/// 解绑分享员（对应 Java `WxChannelApiUrlConstants` 常量 `UNBIND_SHARER_URL`）。
pub const UNBIND_SHARER_URL: &str = "https://api.weixin.qq.com/channels/ec/sharer/unbind";
