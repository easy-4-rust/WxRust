//! 接口许可（License）相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.License`。

/// 创建下单接口。
pub const CREATE_NEW_ORDER: &str = "/cgi-bin/license/create_new_order";
/// 创建续期任务。
pub const CREATE_RENEW_ORDER_JOB: &str = "/cgi-bin/license/create_renew_order_job";
/// 提交续期任务。
pub const SUBMIT_ORDER_JOB: &str = "/cgi-bin/license/submit_order_job";
/// 获取订单列表。
pub const LIST_ORDER: &str = "/cgi-bin/license/list_order";
/// 获取订单详情。
pub const GET_ORDER: &str = "/cgi-bin/license/get_order";
/// 获取订单中的账号列表。
pub const LIST_ORDER_ACCOUNT: &str = "/cgi-bin/license/list_order_account";
/// 激活账号。
pub const ACTIVE_ACCOUNT: &str = "/cgi-bin/license/active_account";
/// 批量激活账号。
pub const BATCH_ACTIVE_ACCOUNT: &str = "/cgi-bin/license/batch_active_account";
/// 查询激活码详情。
pub const GET_ACTIVE_INFO_BY_CODE: &str = "/cgi-bin/license/get_active_info_by_code";
/// 批量查询激活码详情。
pub const BATCH_GET_ACTIVE_INFO_BY_CODE: &str = "/cgi-bin/license/batch_get_active_info_by_code";
/// 获取已激活成员列表。
pub const LIST_ACTIVED_ACCOUNT: &str = "/cgi-bin/license/list_actived_account";
/// 查询成员激活详情。
pub const GET_ACTIVE_INFO_BY_USER: &str = "/cgi-bin/license/get_active_info_by_user";
/// 批量转移成员 license。
pub const BATCH_TRANSFER_LICENSE: &str = "/cgi-bin/license/batch_transfer_license";
