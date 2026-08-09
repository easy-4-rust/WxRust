//! OAuth2 相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.OAuth2`。

/// 获取成员授权信息（code 换 userid，含 agentid 版本）。
pub const GET_USER_INFO: &str = "/cgi-bin/user/getuserinfo?code=%s&agentid=%d";
/// 获取家校应用成员授权信息（code 换 userid）。
pub const GET_SCHOOL_USER_INFO: &str = "/cgi-bin/school/getuserinfo?code=%s";
/// 获取访问用户身份。
pub const GET_USER_DETAIL: &str = "/cgi-bin/auth/getuserdetail";
/// OAuth2 授权地址。
pub const URL_OAUTH2_AUTHORIZE: &str = "https://open.weixin.qq.com/connect/oauth2/authorize";
/// 获取成员授权信息（不带 agentid 版本）。
pub const GET_USER_AUTH_INFO: &str = "/cgi-bin/auth/getuserinfo?code=%s";
/// 获取二次验证信息。
pub const GET_TFA_INFO: &str = "/cgi-bin/auth/get_tfa_info";
