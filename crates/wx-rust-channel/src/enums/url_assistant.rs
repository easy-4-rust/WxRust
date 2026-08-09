//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 上架商品到橱窗（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_WINDOW_PRODUCT_URL`）。
pub const ADD_WINDOW_PRODUCT_URL: &str = "https://api.weixin.qq.com/channels/ec/window/product/add";

/// 获取橱窗商品详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_WINDOW_PRODUCT_URL`）。
pub const GET_WINDOW_PRODUCT_URL: &str = "https://api.weixin.qq.com/channels/ec/window/product/get";

/// 获取已添加到橱窗的商品列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_WINDOW_PRODUCT_URL`）。
pub const LIST_WINDOW_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/window/product/list/get";

/// 下架橱窗商品（对应 Java `WxChannelApiUrlConstants` 常量 `OFF_WINDOW_PRODUCT_URL`）。
pub const OFF_WINDOW_PRODUCT_URL: &str = "https://api.weixin.qq.com/channels/ec/window/product/off";
