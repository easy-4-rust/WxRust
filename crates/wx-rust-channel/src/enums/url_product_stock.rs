//! 视频号小店商品库存接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 更新商品库存（对应 Java `SPU_UPDATE_STOCK_URL`）。
pub const UPDATE_STOCK_URL: &str = "https://api.weixin.qq.com/channels/ec/product/stock/update";

/// 获取商品库存流水（对应 Java `SPU_GET_STOCK_FLOW_URL`）。
pub const GET_STOCK_FLOW_URL: &str = "https://api.weixin.qq.com/channels/ec/product/stock/getflow";
