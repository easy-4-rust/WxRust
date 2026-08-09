//! 群聊（appchat）相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Chat`。

/// 创建群聊会话。
pub const APPCHAT_CREATE: &str = "/cgi-bin/appchat/create";
/// 修改群聊会话。
pub const APPCHAT_UPDATE: &str = "/cgi-bin/appchat/update";
/// 获取群聊会话（`chatid` 拼在路径后）。
pub const APPCHAT_GET_CHATID: &str = "/cgi-bin/appchat/get?chatid=";
/// 群聊发送消息。
pub const APPCHAT_SEND: &str = "/cgi-bin/appchat/send";
