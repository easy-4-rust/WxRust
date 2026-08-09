//! 微信客服相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Kf`。

/// 添加客服账号。
pub const ACCOUNT_ADD: &str = "/cgi-bin/kf/account/add";
/// 更新客服账号。
pub const ACCOUNT_UPD: &str = "/cgi-bin/kf/account/update";
/// 删除客服账号。
pub const ACCOUNT_DEL: &str = "/cgi-bin/kf/account/del";
/// 获取客服账号列表。
pub const ACCOUNT_LIST: &str = "/cgi-bin/kf/account/list";
/// 添加客服「联系我」方式。
pub const ADD_CONTACT_WAY: &str = "/cgi-bin/kf/add_contact_way";
/// 添加客服接待人员。
pub const SERVICER_ADD: &str = "/cgi-bin/kf/servicer/add";
/// 删除客服接待人员。
pub const SERVICER_DEL: &str = "/cgi-bin/kf/servicer/del";
/// 获取客服接待人员列表（`open_kfid` 拼在路径后）。
pub const SERVICER_LIST: &str = "/cgi-bin/kf/servicer/list?open_kfid=";
/// 获取会话状态。
pub const SERVICE_STATE_GET: &str = "/cgi-bin/kf/service_state/get";
/// 变更会话状态。
pub const SERVICE_STATE_TRANS: &str = "/cgi-bin/kf/service_state/trans";
/// 读取消息。
pub const SYNC_MSG: &str = "/cgi-bin/kf/sync_msg";
/// 发送消息。
pub const SEND_MSG: &str = "/cgi-bin/kf/send_msg";
/// 发送事件响应消息。
pub const SEND_MSG_ON_EVENT: &str = "/cgi-bin/kf/send_msg_on_event";
/// 获取客户基本信息。
pub const CUSTOMER_BATCH_GET: &str = "/cgi-bin/kf/customer/batchget";
/// 获取企业统计。
pub const GET_CORP_STATISTIC: &str = "/cgi-bin/kf/get_corp_statistic";
/// 获取接待人员统计。
pub const GET_SERVICER_STATISTIC: &str = "/cgi-bin/kf/get_servicer_statistic";
/// 获取客户升级服务配置。
pub const CUSTOMER_GET_UPGRADE_SERVICE_CONFIG: &str =
    "/cgi-bin/kf/customer/get_upgrade_service_config";
/// 客户升级服务。
pub const CUSTOMER_UPGRADE_SERVICE: &str = "/cgi-bin/kf/customer/upgrade_service";
/// 客户取消升级服务。
pub const CUSTOMER_CANCEL_UPGRADE_SERVICE: &str = "/cgi-bin/kf/customer/cancel_upgrade_service";
