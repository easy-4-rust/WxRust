//! 开放平台（第三方平台）核心接口地址。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenComponentService` 中声明的
//! URL 常量（`API_COMPONENT_TOKEN_URL`/`API_CREATE_PREAUTHCODE_URL` 等）。
//! 函数风格参照 `wx-rust-miniapp/src/enums/url_core.rs`（config 参数 +
//! api_host 前缀模式）；Java 常量值为写死的完整地址，Rust 侧函数基于
//! `WxOpenHostConfig.api_host` 拼接，支持自定义域名（Java
//! `apiHostUrl` 替换语义一致）。
//!
//! 各业务子域完整地址表（小程序管理、minishop、tcb 等）随对应子服务批次补齐。

use crate::config::{API_DEFAULT_HOST_URL, WxOpenConfigStorage};

/// 生成完整接口地址：域名前缀 + 路径。
///
/// 域名优先级：配置的 host 配置（自定义 apiHostUrl）> 默认
/// `https://api.weixin.qq.com`（对应 Java `apiHostUrl` 替换语义）。
fn url(config: &dyn WxOpenConfigStorage, path: &str) -> String {
    let host = config
        .wx_open_host_config()
        .map(|h| h.api_host)
        .unwrap_or_else(|| API_DEFAULT_HOST_URL.to_string());
    format!("{host}{path}")
}

/// 获取 component_access_token（对应 Java
/// `WxOpenComponentService.API_COMPONENT_TOKEN_URL`）。
pub fn api_component_token_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/api_component_token")
}

/// 启动 verify ticket 推送服务（对应 Java
/// `WxOpenComponentService.API_START_PUSH_TICKET`）。
pub fn api_start_push_ticket_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/api_start_push_ticket")
}

/// 获取预授权码（对应 Java
/// `WxOpenComponentService.API_CREATE_PREAUTHCODE_URL`）。
pub fn api_create_preauthcode_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/api_create_preauthcode")
}

/// 使用授权码换取授权信息（对应 Java
/// `WxOpenComponentService.API_QUERY_AUTH_URL`）。
pub fn api_query_auth_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/api_query_auth")
}

/// 获取（刷新）授权方 access_token（对应 Java
/// `WxOpenComponentService.API_AUTHORIZER_TOKEN_URL`）。
pub fn api_authorizer_token_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/api_authorizer_token")
}

/// 获取授权方基本信息（对应 Java
/// `WxOpenComponentService.API_GET_AUTHORIZER_INFO_URL`）。
pub fn api_get_authorizer_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/api_get_authorizer_info")
}

/// 获取授权方选项设置（对应 Java
/// `WxOpenComponentService.GET_AUTHORIZER_OPTION_URL`）。
pub fn get_authorizer_option_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/get_authorizer_option")
}

/// 设置授权方选项（对应 Java
/// `WxOpenComponentService.SET_AUTHORIZER_OPTION_URL`）。
pub fn set_authorizer_option_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/set_authorizer_option")
}

/// 获取授权方列表（对应 Java
/// `WxOpenComponentService.API_GET_AUTHORIZER_LIST`）。
pub fn api_get_authorizer_list_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/api_get_authorizer_list")
}

/// 网页授权预授权链接（对应 Java
/// `WxOpenComponentService.COMPONENT_LOGIN_PAGE_URL`）。
///
/// 格式化串：`component_appid`/`pre_auth_code`/`redirect_uri`（须由调用方
/// 预编码），Java `String.format` 语义；`auth_type`/`biz_appid` 以 `xxx`
/// 占位（Java 中由 `createPreAuthUrl` 按需 replace）。
pub fn component_login_page_url(
    component_app_id: &str,
    pre_auth_code: &str,
    redirect_uri: &str,
) -> String {
    format!(
        "https://mp.weixin.qq.com/cgi-bin/componentloginpage?component_appid={}&pre_auth_code={}&redirect_uri={}&auth_type=xxx&biz_appid=xxx",
        component_app_id, pre_auth_code, redirect_uri
    )
}

/// 移动端预授权链接（对应 Java
/// `WxOpenComponentService.COMPONENT_MOBILE_LOGIN_PAGE_URL`）。
pub fn component_mobile_login_page_url(
    component_app_id: &str,
    pre_auth_code: &str,
    redirect_uri: &str,
) -> String {
    format!(
        "https://open.weixin.qq.com/wxaopen/safe/bindcomponent?action=bindcomponent&no_scan=1&component_appid={}&pre_auth_code={}&redirect_uri={}&auth_type=xxx&biz_appid=xxx#wechat_redirect",
        component_app_id, pre_auth_code, redirect_uri
    )
}

/// 默认 API 域名字面量（对应 Java `WxOpenInMemoryConfigStorage` 默认值）。
pub const API_HOST: &str = API_DEFAULT_HOST_URL;
