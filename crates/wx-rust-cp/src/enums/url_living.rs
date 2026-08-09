//! 直播相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Living`。

/// 获取直播预约码。
pub const GET_LIVING_CODE: &str = "/cgi-bin/living/get_living_code";
/// 获取直播详情（`livingid` 拼在路径后）。
pub const GET_LIVING_INFO: &str = "/cgi-bin/living/get_living_info?livingid=";
/// 获取直播观看统计。
pub const GET_WATCH_STAT: &str = "/cgi-bin/living/get_watch_stat";
/// 获取直播回看信息。
pub const GET_LIVING_SHARE_INFO: &str = "/cgi-bin/living/get_living_share_info";
/// 获取成员直播 ID 列表。
pub const GET_USER_ALL_LIVINGID: &str = "/cgi-bin/living/get_user_all_livingid";
/// 创建直播。
pub const CREATE: &str = "/cgi-bin/living/create";
/// 修改直播。
pub const MODIFY: &str = "/cgi-bin/living/modify";
/// 取消直播。
pub const CANCEL: &str = "/cgi-bin/living/cancel";
/// 删除直播回放。
pub const DELETE_REPLAY_DATA: &str = "/cgi-bin/living/delete_replay_data";
