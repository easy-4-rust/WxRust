//! 视频号小店质检管理接口地址常量（对应 Java `WxChannelApiUrlConstants.Qic`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 查询质检仓配置（对应 Java `Qic.GET_INSPECT_CONFIG_URL`）。
pub const GET_INSPECT_CONFIG_URL: &str =
    "https://api.weixin.qq.com/channels/ec/qic/inspect/config/get";

/// 查询送检配置模板信息（对应 Java `Qic.GET_SUBMIT_CONFIG_URL`）。
pub const GET_SUBMIT_CONFIG_URL: &str =
    "https://api.weixin.qq.com/channels/ec/qic/inspect/submitconfig/get";

/// 打印质检码（对应 Java `Qic.PRINT_INSPECT_CODE_URL`）。
pub const PRINT_INSPECT_CODE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/qic/inspect/code/print";

/// 绑定送检信息（对应 Java `Qic.SUBMIT_INSPECT_INFO_URL`）。
pub const SUBMIT_INSPECT_INFO_URL: &str =
    "https://api.weixin.qq.com/channels/ec/qic/inspect/submit";

/// 自寄快递送检（对应 Java `Qic.REGISTER_LOGISTICS_URL`）。
pub const REGISTER_LOGISTICS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/qic/inspect/register_logistics";
