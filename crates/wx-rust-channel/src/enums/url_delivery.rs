//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取快递公司列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_DELIVERY_COMPANY_NEW_URL`）。
pub const GET_DELIVERY_COMPANY_NEW_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/deliverycompanylist/new/get";

/// 获取快递公司列表（旧）（对应 Java `WxChannelApiUrlConstants` 常量 `GET_DELIVERY_COMPANY_URL`）。
pub const GET_DELIVERY_COMPANY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/deliverycompanylist/get";

/// 订单发货（对应 Java `WxChannelApiUrlConstants` 常量 `DELIVERY_SEND_URL`）。
pub const DELIVERY_SEND_URL: &str = "https://api.weixin.qq.com/channels/ec/order/delivery/send";
