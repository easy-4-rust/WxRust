//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 添加区域仓库（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_WAREHOUSE_URL`）。
pub const ADD_WAREHOUSE_URL: &str = "https://api.weixin.qq.com/channels/ec/warehouse/create";

/// 获取区域仓库列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_WAREHOUSE_URL`）。
pub const LIST_WAREHOUSE_URL: &str = "https://api.weixin.qq.com/channels/ec/warehouse/list/get";

/// 获取区域仓库详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_WAREHOUSE_URL`）。
pub const GET_WAREHOUSE_URL: &str = "https://api.weixin.qq.com/channels/ec/warehouse/get";

/// 更新区域仓库详情（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_WAREHOUSE_URL`）。
pub const UPDATE_WAREHOUSE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/warehouse/detail/update";

/// 批量增加覆盖区域（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_COVER_AREA_URL`）。
pub const ADD_COVER_AREA_URL: &str =
    "https://api.weixin.qq.com/channels/ec/warehouse/coverlocations/add";

/// 批量删除覆盖区域（对应 Java `WxChannelApiUrlConstants` 常量 `DELETE_COVER_AREA_URL`）。
pub const DELETE_COVER_AREA_URL: &str =
    "https://api.weixin.qq.com/channels/ec/warehouse/coverlocations/del";

/// 设置指定地址下的仓的优先级（对应 Java `WxChannelApiUrlConstants` 常量 `SET_WAREHOUSE_PRIORITY_URL`）。
pub const SET_WAREHOUSE_PRIORITY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/warehouse/address/prioritysort/set";

/// 获取指定地址下的仓的优先级（对应 Java `WxChannelApiUrlConstants` 常量 `GET_WAREHOUSE_PRIORITY_URL`）。
pub const GET_WAREHOUSE_PRIORITY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/warehouse/address/prioritysort/get";

/// 更新区域仓库存（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_WAREHOUSE_STOCK_URL`）。
pub const UPDATE_WAREHOUSE_STOCK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/warehouse/stock/update";

/// 获取区域仓库存（对应 Java `WxChannelApiUrlConstants` 常量 `GET_WAREHOUSE_STOCK_URL`）。
pub const GET_WAREHOUSE_STOCK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/warehouse/stock/get";
