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

/// 礼物订单新增备注（对应 Java `WxChannelApiUrlConstants` 常量 `PRESENT_NOTE_ADD_URL`）。
pub const PRESENT_NOTE_ADD_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/presentnote/add";

/// 获取礼物子单列表（对应 Java `WxChannelApiUrlConstants` 常量 `PRESENT_SUB_ORDER_GET_URL`）。
pub const PRESENT_SUB_ORDER_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/presentsuborder/get";

/// 获取待发货前更换 SKU 请求（对应 Java `WxChannelApiUrlConstants` 常量 `PRE_SHIPMENT_CHANGE_SKU_GET_URL`）。
pub const PRE_SHIPMENT_CHANGE_SKU_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/preshipmentchangesku/get";

/// 同意待发货前更换 SKU（对应 Java `WxChannelApiUrlConstants` 常量 `PRE_SHIPMENT_CHANGE_SKU_APPROVE_URL`）。
pub const PRE_SHIPMENT_CHANGE_SKU_APPROVE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/preshipmentchangesku/approve";

/// 拒绝待发货前更换 SKU（对应 Java `WxChannelApiUrlConstants` 常量 `PRE_SHIPMENT_CHANGE_SKU_REJECT_URL`）。
pub const PRE_SHIPMENT_CHANGE_SKU_REJECT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/preshipmentchangesku/reject";

/// 申请真实号（对应 Java `WxChannelApiUrlConstants` 常量 `REAL_NUMBER_APPLY_URL`）。
pub const REAL_NUMBER_APPLY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/realnumber/apply";

/// 查看真实号审核状态（对应 Java `WxChannelApiUrlConstants` 常量 `REAL_NUMBER_VIEW_AUDIT_GET_URL`）。
pub const REAL_NUMBER_VIEW_AUDIT_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/realnumberviewaudit/get";

/// 再次申请虚拟号（对应 Java `WxChannelApiUrlConstants` 常量 `VIRTUAL_NUMBER_APPLY_AGAIN_URL`）。
pub const VIRTUAL_NUMBER_APPLY_AGAIN_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/virtualnumber/applyagain";

/// 延长虚拟号有效期（对应 Java `WxChannelApiUrlConstants` 常量 `VIRTUAL_NUMBER_DELAY_URL`）。
pub const VIRTUAL_NUMBER_DELAY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/virtualnumber/delay";

/// 添加待认证手机号（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_PHONE_URL`）。
pub const ADD_PHONE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/privatenumber/addphone";

/// 获取短信验证码（对应 Java `WxChannelApiUrlConstants` 常量 `SEND_VERIFY_CODE_URL`）。
pub const SEND_VERIFY_CODE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/privatenumber/sendverifycode";

/// 获取小店手机号认证状态（对应 Java `WxChannelApiUrlConstants` 常量 `GET_PHONE_URL`）。
pub const GET_PHONE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/merchant/privatenumber/getphone";

/// 订单补发货（对应 Java `WxChannelApiUrlConstants` 常量 `DELIVERY_COMPENSATION_URL`）。
pub const DELIVERY_COMPENSATION_URL: &str =
    "https://api.weixin.qq.com/channels/ec/order/delivery/compensation";
