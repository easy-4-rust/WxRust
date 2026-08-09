//! 成员（User）相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.User`。

/// 二次验证成员（`userid` 拼在路径后）。
pub const USER_AUTHENTICATE: &str = "/cgi-bin/user/authsucc?userid=";
/// 创建成员。
pub const USER_CREATE: &str = "/cgi-bin/user/create";
/// 更新成员。
pub const USER_UPDATE: &str = "/cgi-bin/user/update";
/// 删除成员（`userid` 拼在路径后）。
pub const USER_DELETE: &str = "/cgi-bin/user/delete?userid=";
/// 批量删除成员。
pub const USER_BATCH_DELETE: &str = "/cgi-bin/user/batchdelete";
/// 获取成员详情（`userid` 拼在路径后）。
pub const USER_GET: &str = "/cgi-bin/user/get?userid=";
/// 获取部门成员详情（`department_id` 拼在路径后）。
pub const USER_LIST: &str = "/cgi-bin/user/list?department_id=";
/// 获取部门成员（`department_id` 拼在路径后）。
pub const USER_SIMPLE_LIST: &str = "/cgi-bin/user/simplelist?department_id=";
/// 邀请成员。
pub const BATCH_INVITE: &str = "/cgi-bin/batch/invite";
/// userid 转 openid。
pub const USER_CONVERT_TO_OPENID: &str = "/cgi-bin/user/convert_to_openid";
/// openid 转 userid。
pub const USER_CONVERT_TO_USERID: &str = "/cgi-bin/user/convert_to_userid";
/// 通过手机号/邮箱获取 userid。
pub const GET_USER_ID: &str = "/cgi-bin/user/getuserid";
/// 通过邮箱获取 userid。
pub const GET_USER_ID_BY_EMAIL: &str = "/cgi-bin/user/get_userid_by_email";
/// 获取外部联系人详情（`external_userid` 拼在路径后）。
pub const GET_EXTERNAL_CONTACT: &str = "/cgi-bin/crm/get_external_contact?external_userid=";
/// 获取加入企业二维码（`size_type` 拼在路径后）。
pub const GET_JOIN_QR_CODE: &str = "/cgi-bin/corp/get_join_qrcode?size_type=";
/// 获取成员活跃度。
pub const GET_ACTIVE_STAT: &str = "/cgi-bin/user/get_active_stat";
/// userid 批量转 open_userid。
pub const USERID_TO_OPEN_USERID: &str = "/cgi-bin/batch/userid_to_openuserid";
/// open_userid 批量转 userid。
pub const OPEN_USERID_TO_USERID: &str = "/cgi-bin/batch/openuserid_to_userid";
/// 获取成员 ID 列表。
pub const USER_LIST_ID: &str = "/cgi-bin/user/list_id";
