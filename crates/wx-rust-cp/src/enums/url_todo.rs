//! 待办相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Todo`。
//! 官方文档: <https://developer.work.weixin.qq.com/document/path/101524>

/// 获取待办详情。
pub const TODO_GET: &str = "/cgi-bin/todo/get";
/// 更新待办状态。
pub const TODO_UPDATE: &str = "/cgi-bin/todo/update";
