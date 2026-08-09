//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取订单列表（对应 Java `WxChannelApiUrlConstants` 常量 `ORDER_LIST_URL`）。
pub const ORDER_LIST_URL: &str = "https://api.weixin.qq.com/channels/ec/order/list/get";

/// 获取订单详情（对应 Java `WxChannelApiUrlConstants` 常量 `ORDER_GET_URL`）。
pub const ORDER_GET_URL: &str = "https://api.weixin.qq.com/channels/ec/order/get";

/// 更改订单价格（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_PRICE_URL`）。
pub const UPDATE_PRICE_URL: &str = "https://api.weixin.qq.com/channels/ec/order/price/update";

/// 修改订单备注（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_REMARK_URL`）。
pub const UPDATE_REMARK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/merchantnotes/update";

/// 更修改订单地址（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_ADDRESS_URL`）。
pub const UPDATE_ADDRESS_URL: &str = "https://api.weixin.qq.com/channels/ec/order/address/update";

/// 修改物流信息（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_EXPRESS_URL`）。
pub const UPDATE_EXPRESS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/deliveryinfo/update";

/// 同意用户修改收货地址申请（对应 Java `WxChannelApiUrlConstants` 常量 `ACCEPT_ADDRESS_MODIFY_URL`）。
pub const ACCEPT_ADDRESS_MODIFY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/addressmodify/accept";

/// 拒绝用户修改收货地址申请（对应 Java `WxChannelApiUrlConstants` 常量 `REJECT_ADDRESS_MODIFY_URL`）。
pub const REJECT_ADDRESS_MODIFY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/addressmodify/reject";

/// 订单搜索（对应 Java `WxChannelApiUrlConstants` 常量 `ORDER_SEARCH_URL`）。
pub const ORDER_SEARCH_URL: &str = "https://api.weixin.qq.com/channels/ec/order/search";

/// 上传生鲜质检信息（对应 Java `WxChannelApiUrlConstants` 常量 `UPLOAD_FRESH_INSPECT_URL`）。
pub const UPLOAD_FRESH_INSPECT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/freshinspect/submit";

/// 兑换虚拟号（对应 Java `WxChannelApiUrlConstants` 常量 `VIRTUAL_TEL_NUMBER_URL`）。
pub const VIRTUAL_TEL_NUMBER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/virtualtelnumber/get";

/// 解码订单包含的敏感数据（对应 Java `WxChannelApiUrlConstants` 常量 `DECODE_SENSITIVE_INFO_URL`）。
pub const DECODE_SENSITIVE_INFO_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/sensitiveinfo/decode";
