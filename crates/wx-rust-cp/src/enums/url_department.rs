//! 部门相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Department`。

/// 创建部门。
pub const DEPARTMENT_CREATE: &str = "/cgi-bin/department/create";
/// 更新部门。
pub const DEPARTMENT_UPDATE: &str = "/cgi-bin/department/update";
/// 获取部门详情。
pub const DEPARTMENT_GET: &str = "/cgi-bin/department/get?id=%d";
/// 删除部门。
pub const DEPARTMENT_DELETE: &str = "/cgi-bin/department/delete?id=%d";
/// 获取部门列表。
pub const DEPARTMENT_LIST: &str = "/cgi-bin/department/list";
/// 获取子部门 ID 列表。
pub const DEPARTMENT_SIMPLE_LIST: &str = "/cgi-bin/department/simplelist";
