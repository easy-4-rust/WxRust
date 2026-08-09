//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 增加地址（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_ADDRESS_URL`）。
pub const ADD_ADDRESS_URL: &str = "https://api.weixin.qq.com/channels/ec/merchant/address/add";

/// 获取地址列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_ADDRESS_URL`）。
pub const LIST_ADDRESS_URL: &str = "https://api.weixin.qq.com/channels/ec/merchant/address/list";

/// 获取地址详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_ADDRESS_URL`）。
pub const GET_ADDRESS_URL: &str = "https://api.weixin.qq.com/channels/ec/merchant/address/get";

/// 更新地址（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_ADDRESS_URL`）。
pub const UPDATE_ADDRESS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/address/update";

/// 删除地址（对应 Java `WxChannelApiUrlConstants` 常量 `DELETE_ADDRESS_URL`）。
pub const DELETE_ADDRESS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/address/delete";
