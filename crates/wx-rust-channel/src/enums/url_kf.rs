//! 视频号小店商家客服接口地址常量（对应 Java `WxChannelApiUrlConstants.Kf`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 上传多媒体资源（对应 Java `Kf.COS_UPLOAD_URL`）。
pub const COS_UPLOAD_URL: &str = "https://api.weixin.qq.com/channels/ec/commkf/cosupload";

/// 发送客服消息（对应 Java `Kf.SEND_MSG_URL`）。
pub const SEND_MSG_URL: &str = "https://api.weixin.qq.com/channels/ec/commkf/sendmsg";
