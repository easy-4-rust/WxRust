//! 企业微信第三方应用（服务商）配置存储。
//!
//! 对应 Java `me.chanjar.weixin.cp.config.WxCpTpConfigStorage`：
//! 服务商侧配置（suiteId/suiteSecret/token/encodingAESKey/corpId/
//! providerSecret）+ suite_access_token/suite_ticket/provider_token 缓存 +
//! 按授权企业（authCorpId）区分的 access_token 与两类 jsapi_ticket
//! （auth_corp_jsapi_ticket / auth_suite_jsapi_ticket）缓存，均为
//! 「值 + 过期时刻 + 独立锁」语义。
//!
//! 说明：
//! - Java `getCorpSecret()` 实际返回服务商 secret（`providerSecret` 字段），
//!   Rust 以 `corp_secret()` 镜像该语义（与 `provider_secret()` 同值）；
//! - Java 的 `getApacheHttpClientBuilder()` 为 Apache 客户端专属，Rust 以
//!   reqwest 单一后端承载（与 WxCpConfigStorage 同一 ADAPTED 说明），
//!   trait 不暴露该方法；
//! - 锁语义：provider/suite 级锁各一；按授权企业区分的锁以 key
//!   `suiteId:authCorpId` 区分（镜像 Java computeIfAbsent 的
//!   `String.join(":", suiteId, authCorpId)`）。

use std::sync::Arc;

use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::bean::WxAccessToken;

use crate::bean::WxCpProviderToken;

/// 企业微信第三方应用配置存储。
pub trait WxCpTpConfigStorage: Send + Sync {
    // ---- baseUrl / apiUrl（对应 Java setBaseApiUrl/getApiUrl） ----

    /// 设置企业微信服务器 baseUrl（对应 Java `setBaseApiUrl(String)`）。
    ///
    /// 默认值为 `https://qyapi.weixin.qq.com`，使用默认值时无需调用。
    fn set_base_api_url(&self, base_url: &str);

    /// 读取自定义企业微信 API Url（对应 Java `getBaseApiUrl`）。
    ///
    /// 返回 `None` 表示未设置（走默认域名）。
    fn base_api_url(&self) -> Option<String>;

    /// 读取企业微信 API Url（对应 Java `getApiUrl(String)`）。
    ///
    /// Java 语义：`baseApiUrl + path`，未设置时按默认域名
    /// `https://qyapi.weixin.qq.com` 拼接。
    fn api_url(&self, path: &str) -> String {
        let base = self
            .base_api_url()
            .filter(|b| !b.is_empty())
            .unwrap_or_else(|| crate::enums::url_core::DEFAULT_CP_BASE_URL.to_string());
        format!("{base}{path}")
    }

    // ---- suite access token（对应 Java getSuiteAccessToken 等） ----

    /// 第三方应用的 suite access token（对应 Java `getSuiteAccessToken()`）。
    fn suite_access_token(&self) -> Option<String>;

    /// 获取 suite_access_token 和剩余过期时间（对应 Java
    /// `getSuiteAccessTokenEntity()`）：`expiresIn` 为剩余秒数，
    /// 已过期/未设置时为 -1（Java `expiresIn <= 0 ? -1 : expiresIn`）。
    fn suite_access_token_entity(&self) -> WxAccessToken;

    /// suite access token 是否已过期（对应 Java `isSuiteAccessTokenExpired()`）。
    fn is_suite_access_token_expired(&self) -> bool;

    /// 强制将 suite access token 过期掉（对应 Java `expireSuiteAccessToken()`）。
    fn expire_suite_access_token(&self);

    /// 更新 suite access token（对应 Java
    /// `updateSuiteAccessToken(String, int)`；Java 预留 200 秒提前过期）。
    fn update_suite_access_token(&self, suite_access_token: &str, expires_in_seconds: i32);

    /// 更新 suite access token（对应 Java 重载
    /// `updateSuiteAccessToken(WxAccessToken)`，取实体上的
    /// accessToken/expiresIn）。
    fn update_suite_access_token_with_entity(&self, suite_access_token: &WxAccessToken) {
        self.update_suite_access_token(
            &suite_access_token.access_token,
            suite_access_token.expires_in,
        );
    }

    // ---- suite ticket（对应 Java getSuiteTicket 等） ----

    /// 第三方应用的 suite ticket（对应 Java `getSuiteTicket()`）。
    fn suite_ticket(&self) -> Option<String>;

    /// suite ticket 是否已过期（对应 Java `isSuiteTicketExpired()`）。
    fn is_suite_ticket_expired(&self) -> bool;

    /// 强制将 suite ticket 过期掉（对应 Java `expireSuiteTicket()`）。
    fn expire_suite_ticket(&self);

    /// 更新 suite ticket（对应 Java `updateSuiteTicket(String, int)`；
    /// 预留 200 秒提前过期）。
    fn update_suite_ticket(&self, suite_ticket: &str, expires_in_seconds: i32);

    // ---- 第三方应用其他配置（对应 Java getSuiteId 等） ----

    /// 第三方应用 suite id（对应 Java `getSuiteId()`）。
    fn suite_id(&self) -> String;

    /// 第三方应用 suite secret（对应 Java `getSuiteSecret()`）。
    fn suite_secret(&self) -> String;

    /// 第三方应用的 token，用来检查应用的签名（对应 Java `getToken()`）。
    fn token(&self) -> Option<String>;

    /// 第三方应用的 EncodingAESKey，用来检查签名（对应 Java
    /// `getEncodingAESKey()`）。
    fn encoding_aes_key(&self) -> Option<String>;

    /// 企微服务商企业 ID（对应 Java `getCorpId()`）。
    fn corp_id(&self) -> String;

    /// 服务商 secret（对应 Java `getCorpSecret()`：Java 中该方法实际返回
    /// `providerSecret` 字段）。
    fn corp_secret(&self) -> String;

    /// 设置服务商 secret（对应 Java `setProviderSecret(String)`）。
    fn set_provider_secret(&self, provider_secret: &str);

    /// 服务商 secret（对应 Java `getProviderSecret()`）。
    fn provider_secret(&self) -> String;

    // ---- 授权企业的 access token（对应 Java getAccessToken(authCorpId) 等） ----

    /// 授权企业的 access token（对应 Java `getAccessToken(String)`）。
    fn access_token(&self, auth_corp_id: &str) -> Option<String>;

    /// 获取授权企业 access token 实体（对应 Java `getAccessTokenEntity`）：
    /// `expiresIn` 按 Java 公式
    /// `(expireTime - now) / 1000 + 200`（未设置时 token 为空串、
    /// expire 按 0 计算）。
    fn access_token_entity(&self, auth_corp_id: &str) -> WxAccessToken;

    /// 授权企业 access token 是否已过期（对应 Java
    /// `isAccessTokenExpired(String)`：不存在或已过期均为 true）。
    fn is_access_token_expired(&self, auth_corp_id: &str) -> bool;

    /// 强制将授权企业 access token 过期掉（对应 Java
    /// `expireAccessToken(String)`）。
    fn expire_access_token(&self, auth_corp_id: &str);

    /// 更新授权企业 access token（对应 Java
    /// `updateAccessToken(String, String, int)`；预留 200 秒提前过期）。
    fn update_access_token(&self, auth_corp_id: &str, access_token: &str, expired_in_seconds: i32);

    // ---- 授权企业的 jsapi ticket（对应 Java getAuthCorpJsApiTicket 等） ----

    /// 授权企业的 jsapi ticket（对应 Java `getAuthCorpJsApiTicket(String)`）。
    fn auth_corp_js_api_ticket(&self, auth_corp_id: &str) -> Option<String>;

    /// 授权企业 jsapi ticket 是否已过期（对应 Java
    /// `isAuthCorpJsApiTicketExpired(String)`：不存在即过期）。
    fn is_auth_corp_js_api_ticket_expired(&self, auth_corp_id: &str) -> bool;

    /// 强制将授权企业 jsapi ticket 过期掉（对应 Java
    /// `expireAuthCorpJsApiTicket(String)`）。
    fn expire_auth_corp_js_api_ticket(&self, auth_corp_id: &str);

    /// 更新授权企业 jsapi ticket（对应 Java
    /// `updateAuthCorpJsApiTicket(String, String, int)`；预留 200 秒）。
    fn update_auth_corp_js_api_ticket(
        &self,
        auth_corp_id: &str,
        js_api_ticket: &str,
        expired_in_seconds: i32,
    );

    // ---- 授权企业的第三方应用 jsapi ticket（对应 Java getAuthSuiteJsApiTicket 等） ----

    /// 授权企业的第三方应用 jsapi ticket（对应 Java
    /// `getAuthSuiteJsApiTicket(String)`）。
    fn auth_suite_js_api_ticket(&self, auth_corp_id: &str) -> Option<String>;

    /// 授权企业第三方应用 jsapi ticket 是否已过期（对应 Java
    /// `isAuthSuiteJsApiTicketExpired(String)`：不存在即过期）。
    fn is_auth_suite_js_api_ticket_expired(&self, auth_corp_id: &str) -> bool;

    /// 强制将授权企业第三方应用 jsapi ticket 过期掉（对应 Java
    /// `expireAuthSuiteJsApiTicket(String)`）。
    fn expire_auth_suite_js_api_ticket(&self, auth_corp_id: &str);

    /// 更新授权企业第三方应用 jsapi ticket（对应 Java
    /// `updateAuthSuiteJsApiTicket(String, String, int)`；预留 200 秒）。
    fn update_auth_suite_js_api_ticket(
        &self,
        auth_corp_id: &str,
        js_api_ticket: &str,
        expired_in_seconds: i32,
    );

    // ---- provider token（对应 Java isProviderTokenExpired 等） ----

    /// provider token 是否已过期（对应 Java `isProviderTokenExpired()`）。
    fn is_provider_token_expired(&self) -> bool;

    /// 更新 provider token（对应 Java `updateProviderToken(String, int)`；
    /// 无 200 秒预留，Java 语义原样）。
    fn update_provider_token(&self, provider_token: &str, expired_in_seconds: i32);

    /// 服务商 provider token（对应 Java `getProviderToken()`）。
    fn provider_token(&self) -> Option<String>;

    /// 获取 provider token 实体（对应 Java `getProviderTokenEntity()`；
    /// Java 返回 null，Rust 以 `None` 表达）。
    fn provider_token_entity(&self) -> Option<WxCpProviderToken> {
        None
    }

    /// 强制将 provider token 过期掉（对应 Java `expireProviderToken()`）。
    fn expire_provider_token(&self);

    // ---- 网络代理（对应 Java getHttpProxyHost 等） ----

    /// HTTP 代理主机（对应 Java `getHttpProxyHost()`）。
    fn http_proxy_host(&self) -> Option<String>;

    /// HTTP 代理端口（对应 Java `getHttpProxyPort()`）。
    fn http_proxy_port(&self) -> i32;

    /// HTTP 代理用户名（对应 Java `getHttpProxyUsername()`）。
    fn http_proxy_username(&self) -> Option<String>;

    /// HTTP 代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<String>;

    /// token 过期时是否自动刷新（对应 Java `autoRefreshToken()`，
    /// 恒 true）。
    fn auto_refresh_token(&self) -> bool {
        true
    }

    /// 临时文件目录（对应 Java `getTmpDirFile()`；Java 返回 `File`，
    /// Rust 以路径字符串表达，`None` 表示未设置）。
    fn tmp_dir_file(&self) -> Option<String>;

    // ---- 锁（对应 Java getProviderAccessTokenLock 等） ----

    /// provider access token 的锁（对应 Java
    /// `getProviderAccessTokenLock()`，key 为 `suiteId:corpId`）。
    fn provider_access_token_lock(&self) -> Arc<AsyncMutex<()>>;

    /// suite access token 的锁（对应 Java `getSuiteAccessTokenLock()`，
    /// key 为 `suiteId`）。
    fn suite_access_token_lock(&self) -> Arc<AsyncMutex<()>>;

    /// 授权企业 access token 的锁（对应 Java `getAccessTokenLock(String)`，
    /// key 为 `suiteId:authCorpId`）。
    fn access_token_lock(&self, auth_corp_id: &str) -> Arc<AsyncMutex<()>>;

    /// 授权企业 jsapi ticket 的锁（对应 Java
    /// `getAuthCorpJsapiTicketLock(String)`，key 为 `suiteId:authCorpId`）。
    fn auth_corp_jsapi_ticket_lock(&self, auth_corp_id: &str) -> Arc<AsyncMutex<()>>;

    /// 授权企业第三方应用 jsapi ticket 的锁（对应 Java
    /// `getSuiteJsapiTicketLock(String)`，key 为 `suiteId:authCorpId`）。
    fn suite_jsapi_ticket_lock(&self, auth_corp_id: &str) -> Arc<AsyncMutex<()>>;
}
