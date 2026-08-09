//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取所有的类目（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_ALL_CATEGORY_URL`）。
pub const LIST_ALL_CATEGORY_URL: &str = "https://api.weixin.qq.com/shop/ec/category/all";

/// 获取类目详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_CATEGORY_DETAIL_URL`）。
pub const GET_CATEGORY_DETAIL_URL: &str = "https://api.weixin.qq.com/shop/ec/category/detail";

/// 获取可用的子类目详情（对应 Java `WxChannelApiUrlConstants` 常量 `AVAILABLE_CATEGORY_URL`）。
pub const AVAILABLE_CATEGORY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/category/availablesoncategories/get";

/// 上传类目资质（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_CATEGORY_URL`）。
pub const ADD_CATEGORY_URL: &str = "https://api.weixin.qq.com/channels/ec/category/add";

/// 获取类目审核结果（对应 Java `WxChannelApiUrlConstants` 常量 `GET_CATEGORY_AUDIT_URL`）。
pub const GET_CATEGORY_AUDIT_URL: &str = "https://api.weixin.qq.com/channels/ec/category/audit/get";

/// 取消类目提审（对应 Java `WxChannelApiUrlConstants` 常量 `CANCEL_CATEGORY_AUDIT_URL`）。
pub const CANCEL_CATEGORY_AUDIT_URL: &str =
    "https://api.weixin.qq.com/shop/ec/category/audit/cancel";

/// 获取账号申请通过的类目和资质信息（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_PASS_CATEGORY_URL`）。
pub const LIST_PASS_CATEGORY_URL: &str = "https://api.weixin.qq.com/channels/ec/category/list/get";

/// 获取店铺的类目权限列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_RELATION_CATEGORY_URL`）。
pub const LIST_RELATION_CATEGORY_URL: &str =
    "https://api.weixin.qq.com/shop/ec/category/get_category_relation_list";
