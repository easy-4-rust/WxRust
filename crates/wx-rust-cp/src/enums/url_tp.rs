//! 第三方代开发（服务商）相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Tp`。

/// 第三方小程序登录凭证校验。
pub const JSCODE_TO_SESSION: &str = "/cgi-bin/service/miniprogram/jscode2session";
/// 获取企业 token。
pub const GET_CORP_TOKEN: &str = "/cgi-bin/service/get_corp_token";
/// 获取企业永久授权码。
pub const GET_PERMANENT_CODE: &str = "/cgi-bin/service/get_permanent_code";
/// 获取企业永久授权码（v2）。
pub const GET_V2_PERMANENT_CODE: &str = "/cgi-bin/service/v2/get_permanent_code";
/// 获取第三方应用凭证。
pub const GET_SUITE_TOKEN: &str = "/cgi-bin/service/get_suite_token";
/// 获取服务商凭证。
pub const GET_PROVIDER_TOKEN: &str = "/cgi-bin/service/get_provider_token";
/// 获取预授权码。
pub const GET_PREAUTH_CODE: &str = "/cgi-bin/service/get_pre_auth_code";
/// 获取企业授权信息。
pub const GET_AUTH_INFO: &str = "/cgi-bin/service/get_auth_info";
/// 获取企业 jsapi ticket。
pub const GET_AUTH_CORP_JSAPI_TICKET: &str = "/cgi-bin/get_jsapi_ticket";
/// 获取第三方应用 suite jsapi ticket。
pub const GET_SUITE_JSAPI_TICKET: &str = "/cgi-bin/ticket/get";
/// 获取成员授权信息（第三方）。
pub const GET_USERINFO3RD: &str = "/cgi-bin/service/auth/getuserinfo3rd";
/// 获取访问用户身份（第三方）。
pub const GET_USERDETAIL3RD: &str = "/cgi-bin/service/auth/getuserdetail3rd";
/// 获取登录用户信息。
pub const GET_LOGIN_INFO: &str = "/cgi-bin/service/get_login_info";
/// 获取定制化授权链接。
pub const GET_CUSTOMIZED_AUTH_URL: &str = "/cgi-bin/service/get_customized_auth_url";
/// 获取模板列表。
pub const GET_TEMPLATE_LIST: &str = "/cgi-bin/service/get_template_list";
/// 获取定制化应用详情。
pub const GET_CUSTOMIZED_APP_DETAIL: &str = "/cgi-bin/service/get_customized_app_detail";
/// 通讯录搜索。
pub const CONTACT_SEARCH: &str = "/cgi-bin/service/contact/search";
/// 获取应用管理员列表。
pub const GET_ADMIN_LIST: &str = "/cgi-bin/service/get_admin_list";
/// 获取应用二维码。
pub const GET_APP_QRCODE: &str = "/cgi-bin/service/get_app_qrcode";
/// corpid 转 opencorpid。
pub const CORPID_TO_OPENCORPID: &str = "/cgi-bin/service/corpid_to_opencorpid";
/// 获取订单详情。
pub const GET_ORDER: &str = "/cgi-bin/service/get_order";
/// 获取订单列表。
pub const GET_ORDER_LIST: &str = "/cgi-bin/service/get_order_list";
/// 延长试用期。
pub const PROLONG_TRY: &str = "/cgi-bin/service/prolong_try";
