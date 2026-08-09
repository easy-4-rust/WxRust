//! 企业互联相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.CorpGroup`。

/// 获取应用共享信息。
/// https://developer.work.weixin.qq.com/document/path/93403
pub const LIST_SHARE_APP_INFO: &str = "/cgi-bin/corpgroup/corp/list_app_share_info";
/// 获取下级/下游企业的 access_token。
/// https://developer.work.weixin.qq.com/document/path/93359
pub const CORP_GET_TOKEN: &str = "/cgi-bin/corpgroup/corp/gettoken";
/// 获取下级/下游企业小程序 session。
/// https://developer.work.weixin.qq.com/document/path/93355
pub const MA_TRANSFER_SESSION: &str = "/cgi-bin/miniprogram/transfer_session";
