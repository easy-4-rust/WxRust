//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取售后列表（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_LIST_URL`）。
pub const AFTER_SALE_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/getaftersalelist";

/// 获取售后单（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_GET_URL`）。
pub const AFTER_SALE_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/getaftersaleorder";

/// 同意售后（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_ACCEPT_URL`）。
pub const AFTER_SALE_ACCEPT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/acceptapply";

/// 拒绝售后（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_REJECT_URL`）。
pub const AFTER_SALE_REJECT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/rejectapply";

/// 上传退款凭证（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_UPLOAD_URL`）。
pub const AFTER_SALE_UPLOAD_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/uploadrefundcertificate";

/// 获取全量售后原因（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_REASON_GET_URL`）。
pub const AFTER_SALE_REASON_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/reason/get";

/// 获取拒绝售后原因（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_REJECT_REASON_GET_URL`）。
pub const AFTER_SALE_REJECT_REASON_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/rejectreason/get";

/// 换货发货（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_ACCEPT_EXCHANGE_RESHIP_URL`）。
pub const AFTER_SALE_ACCEPT_EXCHANGE_RESHIP_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/acceptexchangereship";

/// 换货拒绝发货（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_REJECT_EXCHANGE_RESHIP_URL`）。
pub const AFTER_SALE_REJECT_EXCHANGE_RESHIP_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/rejectexchangereship";

/// 商家协商（对应 Java `WxChannelApiUrlConstants` 常量 `AFTER_SALE_MERCHANT_UPDATE_URL`）。
pub const AFTER_SALE_MERCHANT_UPDATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/merchantupdateaftersale";

/// 查询保障单列表（对应 Java `WxChannelApiUrlConstants` 常量 `GUARANTEE_ORDER_LIST_URL`）。
pub const GUARANTEE_ORDER_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/searchguaranteeorder";

/// 获取保障单详情（对应 Java `WxChannelApiUrlConstants` 常量 `GUARANTEE_ORDER_GET_URL`）。
pub const GUARANTEE_ORDER_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/getguaranteeorder";

/// 商家同意保障单（对应 Java `WxChannelApiUrlConstants` 常量 `GUARANTEE_ORDER_ACCEPT_URL`）。
pub const GUARANTEE_ORDER_ACCEPT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/merchantacceptguarantee";

/// 商家协商保障单（对应 Java `WxChannelApiUrlConstants` 常量 `GUARANTEE_ORDER_MODIFY_URL`）。
pub const GUARANTEE_ORDER_MODIFY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/merchantmodifyguarantee";

/// 商家举证保障单（对应 Java `WxChannelApiUrlConstants` 常量 `GUARANTEE_ORDER_PROOF_URL`）。
pub const GUARANTEE_ORDER_PROOF_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/merchantproofguarantee";

/// 商家拒绝保障单（对应 Java `WxChannelApiUrlConstants` 常量 `GUARANTEE_ORDER_REFUSE_URL`）。
pub const GUARANTEE_ORDER_REFUSE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/merchantrefuseguarantee";
