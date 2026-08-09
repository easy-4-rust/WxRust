//! 腾讯企点接口地址表。
//!
//! 对应 Java `me.chanjar.weixin.qidian.enums.WxQidianApiUrl`：Java 以
//! 接口 + 嵌套枚举（`OAuth2`/`Other`/`Dial`/`CallData`）表达；Rust 以
//! `ApiUrl` 结构体（prefix + path）+ 四个常量模块表达同一契约。
//! `get_url(config)` 按配置的 `host_config` 做域名替换
//! （`WxQidianHostConfig::build_url`）。

use crate::bean::WxQidianHostConfig;
use crate::config::WxQidianConfigStorage;

/// 企点接口地址（对应 Java `WxQidianApiUrl` 的一个枚举常量）。
#[derive(Debug, Clone, Copy)]
pub struct ApiUrl {
    /// 域名前缀（对应 Java `getPrefix()`）
    prefix: &'static str,
    /// 路径（可能含 `%s` 占位符，对应 Java `getPath()`）
    path: &'static str,
}

impl ApiUrl {
    /// 构建地址常量。
    pub const fn new(prefix: &'static str, path: &'static str) -> Self {
        Self { prefix, path }
    }

    /// 路径部分（对应 Java `getPath()`）。
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// 域名前缀（对应 Java `getPrefix()`）。
    pub fn prefix(&self) -> &'static str {
        self.prefix
    }

    /// 得到 API 完整地址（对应 Java 默认方法 `getUrl(config)`）。
    ///
    /// 根据配置的 `host_config` 替换默认域名前缀（config 为 None 或未
    /// 配置 host 时使用默认域名）。
    pub fn get_url(&self, config: Option<&dyn WxQidianConfigStorage>) -> String {
        let host_config: Option<WxQidianHostConfig> = config.map(|c| c.host_config());
        WxQidianHostConfig::build_url(host_config.as_ref(), self.prefix, self.path)
    }
}

/// OAuth2 授权类接口（对应 Java `WxQidianApiUrl.OAuth2` 枚举）。
pub mod o_auth2 {
    use super::ApiUrl;
    use crate::bean::{API_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL};

    /// 用 code 换取 oauth2 的 access token。
    pub const OAUTH2_ACCESS_TOKEN_URL: ApiUrl = ApiUrl::new(
        API_DEFAULT_HOST_URL,
        "/sns/oauth2/access_token?appid=%s&secret=%s&code=%s&grant_type=authorization_code",
    );
    /// 刷新 oauth2 的 access token。
    pub const OAUTH2_REFRESH_TOKEN_URL: ApiUrl = ApiUrl::new(
        API_DEFAULT_HOST_URL,
        "/sns/oauth2/refresh_token?appid=%s&grant_type=refresh_token&refresh_token=%s",
    );
    /// 用 oauth2 获取用户信息。
    pub const OAUTH2_USERINFO_URL: ApiUrl = ApiUrl::new(
        API_DEFAULT_HOST_URL,
        "/sns/userinfo?access_token=%s&openid=%s&lang=%s",
    );
    /// 验证 oauth2 的 access token 是否有效。
    pub const OAUTH2_VALIDATE_TOKEN_URL: ApiUrl =
        ApiUrl::new(API_DEFAULT_HOST_URL, "/sns/auth?access_token=%s&openid=%s");
    /// oauth2 授权的 url 连接。
    pub const CONNECT_OAUTH2_AUTHORIZE_URL: ApiUrl = ApiUrl::new(
        OPEN_DEFAULT_HOST_URL,
        "/connect/oauth2/authorize?appid=%s&redirect_uri=%s&response_type=code&scope=%s&state=%s&connect_redirect=1#wechat_redirect",
    );
}

/// 通用接口（对应 Java `WxQidianApiUrl.Other` 枚举）。
pub mod other {
    use super::ApiUrl;
    use crate::bean::{API_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL, QIDIAN_DEFAULT_HOST_URL};

    /// 获取 access_token。
    pub const GET_ACCESS_TOKEN_URL: ApiUrl = ApiUrl::new(
        QIDIAN_DEFAULT_HOST_URL,
        "/cgi-bin/token?grant_type=client_credential&appid=%s&secret=%s",
    );
    /// 获得各种类型的 ticket。
    pub const GET_TICKET_URL: ApiUrl =
        ApiUrl::new(API_DEFAULT_HOST_URL, "/cgi-bin/ticket/getticket?type=");
    /// 长链接转短链接接口。
    pub const SHORTURL_API_URL: ApiUrl = ApiUrl::new(API_DEFAULT_HOST_URL, "/cgi-bin/shorturl");
    /// 语义查询接口。
    pub const SEMANTIC_SEMPROXY_SEARCH_URL: ApiUrl =
        ApiUrl::new(API_DEFAULT_HOST_URL, "/semantic/semproxy/search");
    /// 获取微信服务器 IP 地址。
    pub const GET_CALLBACK_IP_URL: ApiUrl =
        ApiUrl::new(API_DEFAULT_HOST_URL, "/cgi-bin/getcallbackip");
    /// 网络检测。
    pub const NETCHECK_URL: ApiUrl = ApiUrl::new(API_DEFAULT_HOST_URL, "/cgi-bin/callback/check");
    /// 第三方使用网站应用授权登录的 url。
    pub const QRCONNECT_URL: ApiUrl = ApiUrl::new(
        OPEN_DEFAULT_HOST_URL,
        "/connect/qrconnect?appid=%s&redirect_uri=%s&response_type=code&scope=%s&state=%s#wechat_redirect",
    );
    /// 获取公众号的自动回复规则。
    pub const GET_CURRENT_AUTOREPLY_INFO_URL: ApiUrl =
        ApiUrl::new(API_DEFAULT_HOST_URL, "/cgi-bin/get_current_autoreply_info");
    /// 公众号调用或第三方平台帮公众号调用的 api 调用次数清零。
    pub const CLEAR_QUOTA_URL: ApiUrl = ApiUrl::new(API_DEFAULT_HOST_URL, "/cgi-bin/clear_quota");
}

/// 基础话务接口（对应 Java `WxQidianApiUrl.Dial` 枚举）。
pub mod dial {
    use super::ApiUrl;
    use crate::bean::QIDIAN_DEFAULT_HOST_URL;

    /// IVR 外呼。
    pub const IVR_DIAL: ApiUrl = ApiUrl::new(QIDIAN_DEFAULT_HOST_URL, "/cgi-bin/call/dial/ivrdial");
    /// 拉取 IVR 列表。
    pub const GET_IVR_LIST: ApiUrl =
        ApiUrl::new(QIDIAN_DEFAULT_HOST_URL, "/cgi-bin/call/dial/getivrlist");
}

/// 通话数据接口（对应 Java `WxQidianApiUrl.CallData` 枚举）。
pub mod call_data {
    use super::ApiUrl;
    use crate::bean::QIDIAN_DEFAULT_HOST_URL;

    /// 总机号列表拉取。
    pub const GET_SWITCH_BOARD_LIST: ApiUrl = ApiUrl::new(
        QIDIAN_DEFAULT_HOST_URL,
        "/cgi-bin/call/callData/getswitchboardlist",
    );
}
