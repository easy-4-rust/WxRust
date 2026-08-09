//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 创建优惠券（对应 Java `WxChannelApiUrlConstants` 常量 `CREATE_COUPON_URL`）。
pub const CREATE_COUPON_URL: &str = "https://api.weixin.qq.com/channels/ec/coupon/create";

/// 更新优惠券（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_COUPON_URL`）。
pub const UPDATE_COUPON_URL: &str = "https://api.weixin.qq.com/channels/ec/coupon/update";

/// 更新优惠券状态（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_COUPON_STATUS_URL`）。
pub const UPDATE_COUPON_STATUS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/coupon/update_status";

/// 获取优惠券详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_COUPON_URL`）。
pub const GET_COUPON_URL: &str = "https://api.weixin.qq.com/channels/ec/coupon/get";

/// 获取优惠券ID列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_COUPON_URL`）。
pub const LIST_COUPON_URL: &str = "https://api.weixin.qq.com/channels/ec/coupon/get_list";

/// 获取用户优惠券ID列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_USER_COUPON_URL`）。
pub const LIST_USER_COUPON_URL: &str =
    "https://api.weixin.qq.com/channels/ec/coupon/get_user_coupon_list";

/// 获取用户优惠券详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_USER_COUPON_URL`）。
pub const GET_USER_COUPON_URL: &str =
    "https://api.weixin.qq.com/channels/ec/coupon/get_user_coupon";
