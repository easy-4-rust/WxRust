//! 视频号小店带货助手接口地址常量（对应 Java `WxChannelApiUrlConstants.Talent`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 获取佣金单列表（对应 Java `Talent.GET_ORDER_LIST_URL`）。
pub const GET_ORDER_LIST_URL: &str = "https://api.weixin.qq.com/channels/ec/talent/get_order_list";

/// 获取佣金单详情（对应 Java `Talent.GET_ORDER_DETAIL_URL`）。
pub const GET_ORDER_DETAIL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/talent/get_order_detail";

/// 获取达人橱窗商品列表（对应 Java `Talent.GET_WINDOW_PRODUCT_LIST_URL`）。
pub const GET_WINDOW_PRODUCT_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/talent/window/product/list/get";

/// 获取达人橱窗商品详情（对应 Java `Talent.GET_WINDOW_PRODUCT_DETAIL_URL`）。
pub const GET_WINDOW_PRODUCT_DETAIL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/talent/window/product/get";
