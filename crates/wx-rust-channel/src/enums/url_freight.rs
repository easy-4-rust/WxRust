//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取运费模板列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_TEMPLATE_URL`）。
pub const LIST_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/getfreighttemplatelist";

/// 查询运费模版（对应 Java `WxChannelApiUrlConstants` 常量 `GET_TEMPLATE_URL`）。
pub const GET_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/getfreighttemplatedetail";

/// 增加运费模版（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_TEMPLATE_URL`）。
pub const ADD_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/addfreighttemplate";

/// 更新运费模版（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_TEMPLATE_URL`）。
pub const UPDATE_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/updatefreighttemplate";
