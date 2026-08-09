//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取品牌库列表（对应 Java `WxChannelApiUrlConstants` 常量 `ALL_BRAND_URL`）。
pub const ALL_BRAND_URL: &str = "https://api.weixin.qq.com/shop/ec/brand/all";

/// 新增品牌资质（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_BRAND_URL`）。
pub const ADD_BRAND_URL: &str = "https://api.weixin.qq.com/shop/ec/brand/add";

/// 更新品牌资质（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_BRAND_URL`）。
pub const UPDATE_BRAND_URL: &str = "https://api.weixin.qq.com/channels/ec/brand/update";

/// 撤回品牌资质审核（对应 Java `WxChannelApiUrlConstants` 常量 `CANCEL_BRAND_AUDIT_URL`）。
pub const CANCEL_BRAND_AUDIT_URL: &str = "https://api.weixin.qq.com/shop/ec/brand/audit/cancel";

/// 删除品牌资质（对应 Java `WxChannelApiUrlConstants` 常量 `DELETE_BRAND_URL`）。
pub const DELETE_BRAND_URL: &str = "https://api.weixin.qq.com/channels/ec/brand/delete";

/// 获取品牌资质申请详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BRAND_URL`）。
pub const GET_BRAND_URL: &str = "https://api.weixin.qq.com/channels/ec/brand/get";

/// 获取品牌资质申请列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_BRAND_URL`）。
pub const LIST_BRAND_URL: &str = "https://api.weixin.qq.com/channels/ec/brand/list/get";

/// 获取生效中的品牌资质列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_BRAND_VALID_URL`）。
pub const LIST_BRAND_VALID_URL: &str = "https://api.weixin.qq.com/channels/ec/brand/valid/list/get";
