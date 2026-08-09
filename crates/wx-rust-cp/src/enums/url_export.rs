//! 异步导出相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Export`。

/// 导出成员（简单信息）。
pub const SIMPLE_USER: &str = "/cgi-bin/export/simple_user";
/// 导出成员（详情）。
pub const USER: &str = "/cgi-bin/export/user";
/// 导出部门。
pub const DEPARTMENT: &str = "/cgi-bin/export/department";
/// 导出标签成员。
pub const TAG_USER: &str = "/cgi-bin/export/taguser";
/// 获取导出结果（`jobid` 拼在路径后）。
pub const GET_RESULT: &str = "/cgi-bin/export/get_result?jobid=%s";
