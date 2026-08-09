//! 智能机器人相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.IntelligentRobot`。
//! 官方文档: https://developer.work.weixin.qq.com/document/path/101039

/// 创建智能机器人。
pub const CREATE_ROBOT: &str = "/cgi-bin/intelligent_robot/create";
/// 删除智能机器人。
pub const DELETE_ROBOT: &str = "/cgi-bin/intelligent_robot/delete";
/// 更新智能机器人。
pub const UPDATE_ROBOT: &str = "/cgi-bin/intelligent_robot/update";
/// 查询智能机器人。
pub const GET_ROBOT: &str = "/cgi-bin/intelligent_robot/get";
/// 智能机器人会话。
pub const CHAT: &str = "/cgi-bin/intelligent_robot/chat";
/// 重置智能机器人会话。
pub const RESET_SESSION: &str = "/cgi-bin/intelligent_robot/reset_session";
/// 智能机器人主动发送消息。
/// 官方文档: https://developer.work.weixin.qq.com/document/path/100719
pub const SEND_MESSAGE: &str = "/cgi-bin/intelligent_robot/send_message";
