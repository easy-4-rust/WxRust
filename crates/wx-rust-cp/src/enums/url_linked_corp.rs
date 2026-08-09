//! 互联企业（上下游）相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.LinkedCorp`。

/// 获取应用的可见范围。
/// https://developer.work.weixin.qq.com/document/path/93172
pub const GET_PERM_LIST: &str = "/cgi-bin/linkedcorp/agent/get_perm_list";
/// 获取互联企业成员详细信息。
/// https://developer.work.weixin.qq.com/document/path/93171
pub const GET_USER: &str = "/cgi-bin/linkedcorp/user/get";
/// 获取互联企业部门成员。
/// https://developer.work.weixin.qq.com/document/path/93168
pub const GET_USER_SIMPLELIST: &str = "/cgi-bin/linkedcorp/user/simplelist";
/// 获取互联企业部门成员详情。
/// https://developer.work.weixin.qq.com/document/path/93169
pub const GET_USER_LIST: &str = "/cgi-bin/linkedcorp/user/list";
/// 获取互联企业部门列表。
/// https://developer.work.weixin.qq.com/document/path/93170
pub const GET_DEPARTMENT_LIST: &str = "/cgi-bin/linkedcorp/department/list";
/// 发送应用消息。
/// https://developer.work.weixin.qq.com/document/path/90250
pub const SENG_MESSAGE: &str = "/cgi-bin/linkedcorp/message/send";
