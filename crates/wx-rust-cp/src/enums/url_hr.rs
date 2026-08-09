//! 人事助手相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Hr`。
//! 官方文档：https://developer.work.weixin.qq.com/document/path/99132

/// 获取员工档案字段信息。
pub const GET_FIELD_INFO: &str = "/cgi-bin/hr/get_fields";
/// 获取员工档案数据。
pub const GET_EMPLOYEE_FIELD_INFO: &str = "/cgi-bin/hr/get_staff_info";
/// 更新员工档案数据。
pub const UPDATE_EMPLOYEE_FIELD_INFO: &str = "/cgi-bin/hr/update_staff_info";
