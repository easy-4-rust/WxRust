//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 商家补充纠纷单留言（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_COMPLAINT_MATERIAL_URL`）。
pub const ADD_COMPLAINT_MATERIAL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/addcomplaintmaterial";

/// 商家举证（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_COMPLAINT_PROOF_URL`）。
pub const ADD_COMPLAINT_PROOF_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/addcomplaintproof";

/// 获取纠纷单（对应 Java `WxChannelApiUrlConstants` 常量 `GET_COMPLAINT_ORDER_URL`）。
pub const GET_COMPLAINT_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/aftersale/getcomplaintorder";
