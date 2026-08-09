//! 会话存档相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.MsgAudit`。

/// 获取会话存档内部群成员信息。
pub const GET_PERMIT_USER_LIST: &str = "/cgi-bin/msgaudit/get_permit_user_list";
/// 获取会话内容存档内部群信息。
pub const GET_GROUP_CHAT: &str = "/cgi-bin/msgaudit/groupchat/get";
/// 获取会话存档同意情况。
pub const CHECK_SINGLE_AGREE: &str = "/cgi-bin/msgaudit/check_single_agree";
