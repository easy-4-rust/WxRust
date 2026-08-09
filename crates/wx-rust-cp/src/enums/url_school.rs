//! 家校应用相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.School`。

/// 获取健康上报统计。
pub const GET_HEALTH_REPORT_STAT: &str = "/cgi-bin/health/get_health_report_stat";
/// 获取健康上报任务 ID 列表。
pub const GET_REPORT_JOBIDS: &str = "/cgi-bin/health/get_report_jobids";
/// 获取健康上报任务信息。
pub const GET_REPORT_JOB_INFO: &str = "/cgi-bin/health/get_report_job_info";
/// 获取健康上报答案。
pub const GET_REPORT_ANSWER: &str = "/cgi-bin/health/get_report_answer";
/// 获取教师健康上报自定义信息。
pub const GET_TEACHER_CUSTOMIZE_HEALTH_INFO: &str =
    "/cgi-bin/school/user/get_teacher_customize_health_info";
/// 获取学生健康上报自定义信息。
pub const GET_STUDENT_CUSTOMIZE_HEALTH_INFO: &str =
    "/cgi-bin/school/user/get_student_customize_health_info";
/// 获取健康上报二维码。
pub const GET_HEALTH_QRCODE: &str = "/cgi-bin/school/user/get_health_qrcode";

/// 批量创建学生。
pub const BATCH_CREATE_STUDENT: &str = "/cgi-bin/school/user/batch_create_student";
/// 批量删除学生。
pub const BATCH_DELETE_STUDENT: &str = "/cgi-bin/school/user/batch_delete_student";
/// 批量更新学生。
pub const BATCH_UPDATE_STUDENT: &str = "/cgi-bin/school/user/batch_update_student";
/// 批量创建家长。
pub const BATCH_CREATE_PARENT: &str = "/cgi-bin/school/user/batch_create_parent";
/// 批量删除家长。
pub const BATCH_DELETE_PARENT: &str = "/cgi-bin/school/user/batch_delete_parent";
/// 批量更新家长。
pub const BATCH_UPDATE_PARENT: &str = "/cgi-bin/school/user/batch_update_parent";

/// 创建学生。
pub const CREATE_STUDENT: &str = "/cgi-bin/school/user/create_student";
/// 删除学生（`userid` 拼在路径后）。
pub const DELETE_STUDENT: &str = "/cgi-bin/school/user/delete_student?userid=";
/// 更新学生。
pub const UPDATE_STUDENT: &str = "/cgi-bin/school/user/update_student";
/// 创建家长。
pub const CREATE_PARENT: &str = "/cgi-bin/school/user/create_parent";
/// 更新家长。
pub const UPDATE_PARENT: &str = "/cgi-bin/school/user/update_parent";
/// 删除家长（`userid` 拼在路径后）。
pub const DELETE_PARENT: &str = "/cgi-bin/school/user/delete_parent?userid=";
/// 获取学生详情（`userid` 拼在路径后）。
pub const GET_USER: &str = "/cgi-bin/school/user/get?userid=";
/// 获取部门成员列表。
pub const GET_USER_LIST: &str = "/cgi-bin/school/user/list?department_id=%s&fetch_child=%d";
/// 获取部门家长列表（`department_id` 拼在路径后）。
pub const GET_USER_LIST_PARENT: &str = "/cgi-bin/school/user/list_parent?department_id=";
/// 设置通讯录同步模式。
pub const SET_ARCH_SYNC_MODE: &str = "/cgi-bin/school/set_arch_sync_mode";
/// 设置升级信息。
pub const SET_UPGRADE_INFO: &str = "/cgi-bin/school/set_upgrade_info";

/// 创建部门。
pub const DEPARTMENT_CREATE: &str = "/cgi-bin/school/department/create";
/// 更新部门。
pub const DEPARTMENT_UPDATE: &str = "/cgi-bin/school/department/update";
/// 删除部门（`id` 拼在路径后）。
pub const DEPARTMENT_DELETE: &str = "/cgi-bin/school/department/delete?id=";
/// 获取部门列表。
pub const DEPARTMENT_LIST: &str = "/cgi-bin/school/department/list";

/// 获取付款结果。
pub const GET_PAYMENT_RESULT: &str = "/cgi-bin/school/get_payment_result";
/// 获取交易记录。
pub const GET_TRADE: &str = "/cgi-bin/school/get_trade";
/// 获取应用可用范围（`agentid` 拼在路径后）。
pub const GET_ALLOW_SCOPE: &str = "/cgi-bin/school/agent/get_allow_scope?agentid=";

/// 上课直播：获取直播详情（`livingid` 拼在路径后）。
pub const GET_LIVING_INFO: &str = "/cgi-bin/school/living/get_living_info?livingid=";
/// 上课直播：获取观看统计。
pub const GET_WATCH_STAT: &str = "/cgi-bin/school/living/get_watch_stat";
/// 上课直播：获取未观看统计。
pub const GET_UNWATCH_STAT: &str = "/cgi-bin/school/living/get_unwatch_stat";
