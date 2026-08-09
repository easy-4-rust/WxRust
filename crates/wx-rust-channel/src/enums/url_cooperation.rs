//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取合作账号列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_COOPERATION_URL`）。
pub const LIST_COOPERATION_URL: &str = "https://api.weixin.qq.com/channels/ec/cooperation/list";

/// 查看合作账号邀请状态（对应 Java `WxChannelApiUrlConstants` 常量 `GET_COOPERATION_STATUS_URL`）。
pub const GET_COOPERATION_STATUS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/cooperation/invitation/get";

/// 邀请合作账号（对应 Java `WxChannelApiUrlConstants` 常量 `GENERATE_QRCODE_COOPERATION_URL`）。
pub const GENERATE_QRCODE_COOPERATION_URL: &str =
    "https://api.weixin.qq.com/channels/ec/cooperation/invitation/qrcode/generate";

/// 取消合作账号邀请（对应 Java `WxChannelApiUrlConstants` 常量 `CANCEL_COOPERATION_URL`）。
pub const CANCEL_COOPERATION_URL: &str =
    "https://api.weixin.qq.com/channels/ec/cooperation/invitation/cancel";

/// 解绑合作账号（对应 Java `WxChannelApiUrlConstants` 常量 `UNBIND_COOPERATION_URL`）。
pub const UNBIND_COOPERATION_URL: &str = "https://api.weixin.qq.com/channels/ec/cooperation/unbind";
