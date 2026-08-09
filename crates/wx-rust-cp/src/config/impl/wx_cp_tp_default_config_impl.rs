//! 企业微信第三方应用默认内存配置存储。
//!
//! 对应 Java `me.chanjar.weixin.cp.config.impl.WxCpTpDefaultConfigImpl`：
//! 服务商侧 suite access token/suite ticket/provider token 内存缓存 +
//! 按授权企业区分的 access_token/auth_corp_jsapi_ticket/
//! auth_suite_jsapi_ticket 内存缓存，全部为「值 + 过期时刻 + 独立锁」
//! 语义；过期时刻均预留 200 秒提前过期（Java `expiresInSeconds - 200`
//! 语义，provider token 除外——Java 原样不加预留），线程安全。
//!
//! 与 Java 的字段对应关系：
//! - `suiteAccessToken`/`suiteAccessTokenExpiresTime`/`suiteAccessTokenLocker`
//!   → `suite_access_token: TokenCell`
//! - `suiteTicket`/`suiteTicketExpiresTime` → `suite_ticket: TokenCell`
//! - `providerToken`/`providerTokenExpiresTime` → `provider_token: TokenCell`
//!   （update 不加 200 秒预留，镜像 Java）
//! - `authCorpAccessTokenMap`/`authCorpAccessTokenExpireTimeMap`/
//!   `accessTokenLocker` → `auth_corp_access_tokens: Mutex<HashMap<String,
//!   TokenCell>>`（key 为 authCorpId；锁 key 为 `suiteId:authCorpId`）
//! - `authCorpJsApiTicketMap`/`authSuiteJsApiTicketMap` 同理
//! - 代理/编码键/锁 key 的 `String.join(":", ...)` 语义逐一镜像

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::bean::WxAccessToken;
use wx_rust_common::config::TokenEntry;

use crate::config::WxCpTpConfigStorage;

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// token 缓存单元（值 + 过期时刻；锁由 `per_key_locks` 承载，对应 Java
/// 各「值 Map + Lock Map」分离存储）。
#[derive(Debug)]
struct TokenCell {
    entry: Mutex<Option<TokenEntry>>,
}

impl TokenCell {
    fn new() -> Self {
        Self {
            entry: Mutex::new(None),
        }
    }

    fn get(&self) -> Option<String> {
        self.entry.lock().unwrap().as_ref().map(|t| t.value.clone())
    }

    fn is_expired(&self) -> bool {
        match self.entry.lock().unwrap().as_ref() {
            Some(t) => t.is_expired(now()),
            None => true,
        }
    }

    fn expire(&self) {
        *self.entry.lock().unwrap() = None;
    }

    /// 更新 token（Java 语义：预留 200 秒提前过期）。
    fn update(&self, value: &str, expires_in_seconds: i32) {
        *self.entry.lock().unwrap() = Some(TokenEntry {
            value: value.to_string(),
            expires_at: Some(now() + (expires_in_seconds - 200).max(0) as i64),
        });
    }

    /// 更新 token（不预留 200 秒，镜像 Java `updateProviderToken`）。
    fn update_no_reserve(&self, value: &str, expires_in_seconds: i32) {
        *self.entry.lock().unwrap() = Some(TokenEntry {
            value: value.to_string(),
            expires_at: Some(now() + expires_in_seconds.max(0) as i64),
        });
    }

    /// 实体剩余秒数（Java `(expiresTime - now) / 1000`，<=0 取 -1）。
    fn remaining_seconds(&self) -> i32 {
        match self.entry.lock().unwrap().as_ref() {
            Some(t) => {
                let remain = t.expires_at.unwrap_or(0) - now();
                if remain <= 0 { -1 } else { remain as i32 }
            }
            None => -1,
        }
    }
}

/// 企业微信第三方应用默认配置存储（内存实现）。
#[derive(Debug)]
pub struct WxCpTpDefaultConfig {
    /// 企微服务商企业 ID（对应 Java `corpId`）。
    corp_id: RwLock<String>,
    /// 服务商 secret（对应 Java `providerSecret`）。
    provider_secret: RwLock<String>,
    /// 第三方应用 suite id（对应 Java `suiteId`）。
    suite_id: RwLock<String>,
    /// 第三方应用 suite secret（对应 Java `suiteSecret`）。
    suite_secret: RwLock<String>,
    /// 第三方应用 token，用来检查应用的签名（对应 Java `token`）。
    token: RwLock<Option<String>>,
    /// 第三方应用 EncodingAESKey（对应 Java `encodingAESKey`）。
    encoding_aes_key: RwLock<Option<String>>,
    /// suite access token 缓存（对应 Java `suiteAccessToken`）。
    suite_access_token: TokenCell,
    /// suite ticket 缓存（对应 Java `suiteTicket`）。
    suite_ticket: TokenCell,
    /// provider token 缓存（对应 Java `providerToken`）。
    provider_token: TokenCell,
    /// 按授权企业区分的 access token 缓存（对应 Java
    /// `authCorpAccessTokenMap`）。
    auth_corp_access_tokens: Mutex<HashMap<String, Arc<TokenCell>>>,
    /// 按授权企业区分的 jsapi ticket 缓存（对应 Java
    /// `authCorpJsApiTicketMap`）。
    auth_corp_js_api_tickets: Mutex<HashMap<String, Arc<TokenCell>>>,
    /// 按授权企业区分的第三方应用 jsapi ticket 缓存（对应 Java
    /// `authSuiteJsApiTicketMap`）。
    auth_suite_js_api_tickets: Mutex<HashMap<String, Arc<TokenCell>>>,
    /// 按 key（`suiteId:authCorpId`）区分的锁（对应 Java
    /// `providerAccessTokenLocker`/`suiteAccessTokenLocker`/
    /// `accessTokenLocker`/`authCorpJsapiTicketLocker`/
    /// `authSuiteJsapiTicketLocker` 的 computeIfAbsent 语义）。
    per_key_locks: Mutex<HashMap<String, Arc<AsyncMutex<()>>>>,
    /// 自定义 baseUrl（对应 Java `baseApiUrl`，`None` 走默认域名）。
    base_api_url: RwLock<Option<String>>,
    http_proxy_host: RwLock<Option<String>>,
    http_proxy_port: RwLock<i32>,
    http_proxy_username: RwLock<Option<String>>,
    http_proxy_password: RwLock<Option<String>>,
    tmp_dir_file: RwLock<Option<String>>,
}

impl WxCpTpDefaultConfig {
    /// 构建默认配置。
    pub fn new() -> Self {
        Self {
            corp_id: RwLock::new(String::new()),
            provider_secret: RwLock::new(String::new()),
            suite_id: RwLock::new(String::new()),
            suite_secret: RwLock::new(String::new()),
            token: RwLock::new(None),
            encoding_aes_key: RwLock::new(None),
            suite_access_token: TokenCell::new(),
            suite_ticket: TokenCell::new(),
            provider_token: TokenCell::new(),
            auth_corp_access_tokens: Mutex::new(HashMap::new()),
            auth_corp_js_api_tickets: Mutex::new(HashMap::new()),
            auth_suite_js_api_tickets: Mutex::new(HashMap::new()),
            per_key_locks: Mutex::new(HashMap::new()),
            base_api_url: RwLock::new(None),
            http_proxy_host: RwLock::new(None),
            http_proxy_port: RwLock::new(0),
            http_proxy_username: RwLock::new(None),
            http_proxy_password: RwLock::new(None),
            tmp_dir_file: RwLock::new(None),
        }
    }

    /// 设置企微服务商企业 ID（对应 Java `setCorpId`）。
    pub fn set_corp_id(&self, corp_id: impl Into<String>) {
        *self.corp_id.write().unwrap() = corp_id.into();
    }

    /// 设置第三方应用 suite id（对应 Java `setSuiteId`）。
    pub fn set_suite_id(&self, suite_id: impl Into<String>) {
        *self.suite_id.write().unwrap() = suite_id.into();
    }

    /// 设置第三方应用 suite secret（对应 Java `setSuiteSecret`）。
    pub fn set_suite_secret(&self, suite_secret: impl Into<String>) {
        *self.suite_secret.write().unwrap() = suite_secret.into();
    }

    /// 设置第三方应用 token（对应 Java `setToken`）。
    pub fn set_token(&self, token: impl Into<String>) {
        *self.token.write().unwrap() = Some(token.into());
    }

    /// 设置第三方应用 EncodingAESKey（对应 Java `setEncodingAESKey`）。
    pub fn set_encoding_aes_key(&self, encoding_aes_key: impl Into<String>) {
        *self.encoding_aes_key.write().unwrap() = Some(encoding_aes_key.into());
    }

    /// 直接设置 suite access token（对应 Java `setSuiteAccessToken`，
    /// 仅赋值不更新过期时刻）。
    pub fn set_suite_access_token(&self, suite_access_token: impl Into<String>) {
        *self.suite_access_token.entry.lock().unwrap() = Some(TokenEntry {
            value: suite_access_token.into(),
            expires_at: None,
        });
    }

    /// 直接设置 suite ticket（对应 Java `setSuiteTicket`，@Deprecated，
    /// 仅赋值不更新过期时刻）。
    pub fn set_suite_ticket(&self, suite_ticket: impl Into<String>) {
        *self.suite_ticket.entry.lock().unwrap() = Some(TokenEntry {
            value: suite_ticket.into(),
            expires_at: None,
        });
    }

    /// 设置 HTTP 代理主机（对应 Java `setHttpProxyHost`）。
    pub fn set_http_proxy_host(&self, host: impl Into<String>) {
        *self.http_proxy_host.write().unwrap() = Some(host.into());
    }

    /// 设置 HTTP 代理端口（对应 Java `setHttpProxyPort`）。
    pub fn set_http_proxy_port(&self, port: i32) {
        *self.http_proxy_port.write().unwrap() = port;
    }

    /// 设置 HTTP 代理用户名（对应 Java `setHttpProxyUsername`）。
    pub fn set_http_proxy_username(&self, username: impl Into<String>) {
        *self.http_proxy_username.write().unwrap() = Some(username.into());
    }

    /// 设置 HTTP 代理密码（对应 Java `setHttpProxyPassword`）。
    pub fn set_http_proxy_password(&self, password: impl Into<String>) {
        *self.http_proxy_password.write().unwrap() = Some(password.into());
    }

    /// 设置临时文件目录（对应 Java `setTmpDirFile`）。
    pub fn set_tmp_dir_file(&self, tmp_dir_file: impl Into<String>) {
        *self.tmp_dir_file.write().unwrap() = Some(tmp_dir_file.into());
    }

    /// 按 key 获取（或创建）锁，对应 Java 各 `Locker.computeIfAbsent(key,
    /// k -> new ReentrantLock())`。
    fn per_key_lock(&self, key: &str) -> Arc<AsyncMutex<()>> {
        self.per_key_locks
            .lock()
            .unwrap()
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(AsyncMutex::new(())))
            .clone()
    }

    /// 按授权企业取缓存单元（不存在时创建，返回 Arc 克隆以脱离锁守卫
    /// 生命周期），对应 Java 的 Map.get/put 组合语义。
    fn cell<'a>(
        map: &'a Mutex<HashMap<String, Arc<TokenCell>>>,
        auth_corp_id: &str,
    ) -> Arc<TokenCell> {
        // 先取后插（不存在则创建空单元）
        map.lock()
            .unwrap()
            .entry(auth_corp_id.to_string())
            .or_insert_with(|| Arc::new(TokenCell::new()))
            .clone()
    }
}

impl Default for WxCpTpDefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl WxCpTpConfigStorage for WxCpTpDefaultConfig {
    fn set_base_api_url(&self, base_url: &str) {
        *self.base_api_url.write().unwrap() = Some(base_url.to_string());
    }

    fn base_api_url(&self) -> Option<String> {
        self.base_api_url.read().unwrap().clone()
    }

    fn suite_access_token(&self) -> Option<String> {
        self.suite_access_token.get()
    }

    fn suite_access_token_entity(&self) -> WxAccessToken {
        WxAccessToken {
            access_token: self.suite_access_token.get().unwrap_or_default(),
            expires_in: self.suite_access_token.remaining_seconds(),
        }
    }

    fn is_suite_access_token_expired(&self) -> bool {
        self.suite_access_token.is_expired()
    }

    fn expire_suite_access_token(&self) {
        self.suite_access_token.expire();
    }

    fn update_suite_access_token(&self, suite_access_token: &str, expires_in_seconds: i32) {
        self.suite_access_token
            .update(suite_access_token, expires_in_seconds);
    }

    fn suite_ticket(&self) -> Option<String> {
        self.suite_ticket.get()
    }

    fn is_suite_ticket_expired(&self) -> bool {
        self.suite_ticket.is_expired()
    }

    fn expire_suite_ticket(&self) {
        self.suite_ticket.expire();
    }

    fn update_suite_ticket(&self, suite_ticket: &str, expires_in_seconds: i32) {
        self.suite_ticket.update(suite_ticket, expires_in_seconds);
    }

    fn suite_id(&self) -> String {
        self.suite_id.read().unwrap().clone()
    }

    fn suite_secret(&self) -> String {
        self.suite_secret.read().unwrap().clone()
    }

    fn token(&self) -> Option<String> {
        self.token.read().unwrap().clone()
    }

    fn encoding_aes_key(&self) -> Option<String> {
        self.encoding_aes_key.read().unwrap().clone()
    }

    fn corp_id(&self) -> String {
        self.corp_id.read().unwrap().clone()
    }

    fn corp_secret(&self) -> String {
        // Java：getCorpSecret() 返回 providerSecret 字段
        self.provider_secret.read().unwrap().clone()
    }

    fn set_provider_secret(&self, provider_secret: &str) {
        *self.provider_secret.write().unwrap() = provider_secret.to_string();
    }

    fn provider_secret(&self) -> String {
        self.provider_secret.read().unwrap().clone()
    }

    fn access_token(&self, auth_corp_id: &str) -> Option<String> {
        Self::cell(&self.auth_corp_access_tokens, auth_corp_id).get()
    }

    fn access_token_entity(&self, auth_corp_id: &str) -> WxAccessToken {
        // Java：expire 缺省按 0L，expiresIn = (expire - now)/1000 + 200
        let cell = Self::cell(&self.auth_corp_access_tokens, auth_corp_id);
        let remain = cell
            .entry
            .lock()
            .unwrap()
            .as_ref()
            .map(|t| t.expires_at.unwrap_or(0) - now())
            .unwrap_or(-now());
        WxAccessToken {
            access_token: cell.get().unwrap_or_default(),
            expires_in: ((remain + 200) as i64 / 1) as i32,
        }
    }

    fn is_access_token_expired(&self, auth_corp_id: &str) -> bool {
        Self::cell(&self.auth_corp_access_tokens, auth_corp_id).is_expired()
    }

    fn expire_access_token(&self, auth_corp_id: &str) {
        Self::cell(&self.auth_corp_access_tokens, auth_corp_id).expire();
    }

    fn update_access_token(&self, auth_corp_id: &str, access_token: &str, expired_in_seconds: i32) {
        Self::cell(&self.auth_corp_access_tokens, auth_corp_id)
            .update(access_token, expired_in_seconds);
    }

    fn auth_corp_js_api_ticket(&self, auth_corp_id: &str) -> Option<String> {
        Self::cell(&self.auth_corp_js_api_tickets, auth_corp_id).get()
    }

    fn is_auth_corp_js_api_ticket_expired(&self, auth_corp_id: &str) -> bool {
        Self::cell(&self.auth_corp_js_api_tickets, auth_corp_id).is_expired()
    }

    fn expire_auth_corp_js_api_ticket(&self, auth_corp_id: &str) {
        Self::cell(&self.auth_corp_js_api_tickets, auth_corp_id).expire();
    }

    fn update_auth_corp_js_api_ticket(
        &self,
        auth_corp_id: &str,
        js_api_ticket: &str,
        expired_in_seconds: i32,
    ) {
        Self::cell(&self.auth_corp_js_api_tickets, auth_corp_id)
            .update(js_api_ticket, expired_in_seconds);
    }

    fn auth_suite_js_api_ticket(&self, auth_corp_id: &str) -> Option<String> {
        Self::cell(&self.auth_suite_js_api_tickets, auth_corp_id).get()
    }

    fn is_auth_suite_js_api_ticket_expired(&self, auth_corp_id: &str) -> bool {
        Self::cell(&self.auth_suite_js_api_tickets, auth_corp_id).is_expired()
    }

    fn expire_auth_suite_js_api_ticket(&self, auth_corp_id: &str) {
        Self::cell(&self.auth_suite_js_api_tickets, auth_corp_id).expire();
    }

    fn update_auth_suite_js_api_ticket(
        &self,
        auth_corp_id: &str,
        js_api_ticket: &str,
        expired_in_seconds: i32,
    ) {
        Self::cell(&self.auth_suite_js_api_tickets, auth_corp_id)
            .update(js_api_ticket, expired_in_seconds);
    }

    fn is_provider_token_expired(&self) -> bool {
        self.provider_token.is_expired()
    }

    fn update_provider_token(&self, provider_token: &str, expired_in_seconds: i32) {
        // Java updateProviderToken 不加 200 秒预留
        self.provider_token
            .update_no_reserve(provider_token, expired_in_seconds);
    }

    fn provider_token(&self) -> Option<String> {
        self.provider_token.get()
    }

    fn expire_provider_token(&self) {
        self.provider_token.expire();
    }

    fn http_proxy_host(&self) -> Option<String> {
        self.http_proxy_host.read().unwrap().clone()
    }

    fn http_proxy_port(&self) -> i32 {
        *self.http_proxy_port.read().unwrap()
    }

    fn http_proxy_username(&self) -> Option<String> {
        self.http_proxy_username.read().unwrap().clone()
    }

    fn http_proxy_password(&self) -> Option<String> {
        self.http_proxy_password.read().unwrap().clone()
    }

    fn tmp_dir_file(&self) -> Option<String> {
        self.tmp_dir_file.read().unwrap().clone()
    }

    fn provider_access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        // Java key：String.join(":", suiteId, corpId)
        let key = format!("{}:{}", self.suite_id(), self.corp_id());
        self.per_key_lock(&key)
    }

    fn suite_access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        // Java key：suiteId
        self.per_key_lock(&self.suite_id())
    }

    fn access_token_lock(&self, auth_corp_id: &str) -> Arc<AsyncMutex<()>> {
        let key = format!("{}:{auth_corp_id}", self.suite_id());
        self.per_key_lock(&key)
    }

    fn auth_corp_jsapi_ticket_lock(&self, auth_corp_id: &str) -> Arc<AsyncMutex<()>> {
        let key = format!("{}:{auth_corp_id}", self.suite_id());
        self.per_key_lock(&key)
    }

    fn suite_jsapi_ticket_lock(&self, auth_corp_id: &str) -> Arc<AsyncMutex<()>> {
        let key = format!("{}:{auth_corp_id}", self.suite_id());
        self.per_key_lock(&key)
    }
}
