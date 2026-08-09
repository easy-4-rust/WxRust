//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 拉取用户详情（对应 Java `WxChannelApiUrlConstants` 常量 `VIP_USER_INFO_URL`）。
pub const VIP_USER_INFO_URL: &str = "https://api.weixin.qq.com/channels/ec/vip/user/info/get";

/// 拉取用户列表（对应 Java `WxChannelApiUrlConstants` 常量 `VIP_USER_LIST_URL`）。
pub const VIP_USER_LIST_URL: &str = "https://api.weixin.qq.com/channels/ec/vip/user/list/get";

/// 获取用户积分（对应 Java `WxChannelApiUrlConstants` 常量 `VIP_SCORE_URL`）。
pub const VIP_SCORE_URL: &str = "https://api.weixin.qq.com/channels/ec/vip/user/score/get";

/// 增加用户积分（对应 Java `WxChannelApiUrlConstants` 常量 `SCORE_INCREASE_URL`）。
pub const SCORE_INCREASE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/vip/user/score/increase";

/// 减少用户积分（对应 Java `WxChannelApiUrlConstants` 常量 `SCORE_DECREASE_URL`）。
pub const SCORE_DECREASE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/vip/user/score/decrease";

/// 更新用户等级（对应 Java `WxChannelApiUrlConstants` 常量 `GRADE_UPDATE_URL`）。
pub const GRADE_UPDATE_URL: &str = "https://api.weixin.qq.com/channels/ec/vip/user/grade/update";
