//! 消息推送相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Message`。
//! https://work.weixin.qq.com/api/doc/90000/90135/90235

/// 发送应用消息。
pub const MESSAGE_SEND: &str = "/cgi-bin/message/send";
/// 查询应用消息发送统计。
pub const GET_STATISTICS: &str = "/cgi-bin/message/get_statistics";
/// 发送「学校通知」。
/// https://developer.work.weixin.qq.com/document/path/92321
pub const EXTERNAL_CONTACT_MESSAGE_SEND: &str = "/cgi-bin/externalcontact/message/send";
/// 撤回应用消息。
/// https://developer.work.weixin.qq.com/document/path/94867
pub const MESSAGE_RECALL: &str = "/cgi-bin/message/recall";
/// 互联企业发送应用消息。
/// https://developer.work.weixin.qq.com/document/path/90250
pub const LINKEDCORP_MESSAGE_SEND: &str = "/cgi-bin/linkedcorp/message/send";
