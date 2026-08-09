//! 开放平台（第三方平台）配置存储默认实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenInMemoryConfigStorage`
//! （任务命名 `WxOpenDefaultConfigImpl`，与 mp/ma 的 DefaultConfigImpl 对齐）：
//! 内存实现，component 三凭证 + verify_ticket + component_access_token 缓存
//! （预留 200 秒提前过期，Java `(expiresInSeconds - 200) * 1000L`）、按
//! appId 分桶的授权方 token/ticket 缓存、按 key 的锁表，线程安全。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::TokenEntry;

use crate::config::{API_DEFAULT_HOST_URL, WxOpenConfigStorage, WxOpenHostConfig};

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// 开放平台默认配置存储（内存实现）。
#[derive(Debug)]
pub struct WxOpenDefaultConfig {
    component_app_id: RwLock<Option<String>>,
    component_app_secret: RwLock<Option<String>>,
    component_token: RwLock<Option<String>>,
    component_aes_key: RwLock<Option<String>>,
    component_verify_ticket: RwLock<Option<String>>,
    component_access_token: Mutex<Option<TokenEntry>>,
    component_access_token_lock: Arc<AsyncMutex<()>>,
    /// 按 key 的锁表（对应 Java `Map<String, Lock> locks`）
    locks: Mutex<HashMap<String, Arc<AsyncMutex<()>>>>,
    /// 授权方 refresh_token 缓存（对应 Java `authorizerRefreshTokens`）
    authorizer_refresh_tokens: Mutex<HashMap<String, TokenEntry>>,
    /// 授权方 access_token 缓存（对应 Java `authorizerAccessTokens`）
    authorizer_access_tokens: Mutex<HashMap<String, TokenEntry>>,
    /// 授权方 jsapi ticket 缓存（对应 Java `jsapiTickets`）
    jsapi_tickets: Mutex<HashMap<String, TokenEntry>>,
    /// 授权方卡券 api ticket 缓存（对应 Java `cardApiTickets`）
    card_api_tickets: Mutex<HashMap<String, TokenEntry>>,
    http_proxy_host: RwLock<Option<String>>,
    http_proxy_port: i32,
    http_proxy_username: RwLock<Option<String>>,
    http_proxy_password: RwLock<Option<String>>,
    retry_sleep_millis: i32,
    max_retry_times: i32,
    /// 自定义 API 主机地址（对应 Java `apiHostUrl`，用于替换默认
    /// `https://api.weixin.qq.com`）
    api_host_url: RwLock<Option<String>>,
    /// 自定义获取 accessToken 地址（对应 Java `accessTokenUrl`）
    access_token_url: RwLock<Option<String>>,
    host_config: RwLock<WxOpenHostConfig>,
    component_api_signature_rsa_private_key: RwLock<Option<String>>,
    component_api_signature_aes_key: RwLock<Option<String>>,
    component_api_signature_rsa_private_key_sn: RwLock<Option<String>>,
    component_api_signature_aes_key_sn: RwLock<Option<String>>,
}

impl WxOpenDefaultConfig {
    /// 构建默认配置（Java 以无参构造 + setter 装配，Rust 提供同名语义的
    /// 便捷构造；字段默认 None/0/1000/5，与 Java 字段默认值一致）。
    pub fn new() -> Self {
        Self {
            component_app_id: RwLock::new(None),
            component_app_secret: RwLock::new(None),
            component_token: RwLock::new(None),
            component_aes_key: RwLock::new(None),
            component_verify_ticket: RwLock::new(None),
            component_access_token: Mutex::new(None),
            component_access_token_lock: Arc::new(AsyncMutex::new(())),
            locks: Mutex::new(HashMap::new()),
            authorizer_refresh_tokens: Mutex::new(HashMap::new()),
            authorizer_access_tokens: Mutex::new(HashMap::new()),
            jsapi_tickets: Mutex::new(HashMap::new()),
            card_api_tickets: Mutex::new(HashMap::new()),
            http_proxy_host: RwLock::new(None),
            http_proxy_port: 0,
            http_proxy_username: RwLock::new(None),
            http_proxy_password: RwLock::new(None),
            retry_sleep_millis: 1000,
            max_retry_times: 5,
            api_host_url: RwLock::new(None),
            access_token_url: RwLock::new(None),
            host_config: RwLock::new(WxOpenHostConfig::new()),
            component_api_signature_rsa_private_key: RwLock::new(None),
            component_api_signature_aes_key: RwLock::new(None),
            component_api_signature_rsa_private_key_sn: RwLock::new(None),
            component_api_signature_aes_key_sn: RwLock::new(None),
        }
    }

    // ---- 便捷 setter（对应 Java Lombok setter；返回 &mut Self 链式调用） ----

    /// 设置第三方平台 appid。
    pub fn set_component_app_id(&mut self, v: impl Into<String>) -> &mut Self {
        *self.component_app_id.write().unwrap() = Some(v.into());
        self
    }

    /// 设置第三方平台 appsecret。
    pub fn set_component_app_secret(&mut self, v: impl Into<String>) -> &mut Self {
        *self.component_app_secret.write().unwrap() = Some(v.into());
        self
    }

    /// 设置消息校验 Token。
    pub fn set_component_token(&mut self, v: impl Into<String>) -> &mut Self {
        *self.component_token.write().unwrap() = Some(v.into());
        self
    }

    /// 设置消息加解密 Key。
    pub fn set_component_aes_key(&mut self, v: impl Into<String>) -> &mut Self {
        *self.component_aes_key.write().unwrap() = Some(v.into());
        self
    }

    /// 设置推送的 verify ticket。
    pub fn set_component_verify_ticket(&mut self, v: impl Into<String>) -> &mut Self {
        *self.component_verify_ticket.write().unwrap() = Some(v.into());
        self
    }

    /// 设置 HTTP 代理主机。
    pub fn set_http_proxy_host(&mut self, v: impl Into<String>) -> &mut Self {
        *self.http_proxy_host.write().unwrap() = Some(v.into());
        self
    }

    /// 设置 HTTP 代理端口。
    pub fn set_http_proxy_port(&mut self, v: i32) -> &mut Self {
        self.http_proxy_port = v;
        self
    }

    /// 设置 HTTP 代理用户名。
    pub fn set_http_proxy_username(&mut self, v: impl Into<String>) -> &mut Self {
        *self.http_proxy_username.write().unwrap() = Some(v.into());
        self
    }

    /// 设置 HTTP 代理密码。
    pub fn set_http_proxy_password(&mut self, v: impl Into<String>) -> &mut Self {
        *self.http_proxy_password.write().unwrap() = Some(v.into());
        self
    }

    /// 设置 HTTP 请求重试间隔（毫秒）。
    pub fn set_retry_sleep_millis(&mut self, v: i32) -> &mut Self {
        self.retry_sleep_millis = v;
        self
    }

    /// 设置 HTTP 请求最大重试次数。
    pub fn set_max_retry_times(&mut self, v: i32) -> &mut Self {
        self.max_retry_times = v;
        self
    }

    /// 设置自定义 API 主机地址（对应 Java `setApiHostUrl(String)`）。
    pub fn set_api_host_url(&mut self, v: impl Into<String>) -> &mut Self {
        *self.api_host_url.write().unwrap() = Some(v.into());
        self
    }

    /// 设置自定义获取 accessToken 地址（对应 Java `setAccessTokenUrl(String)`）。
    pub fn set_access_token_url(&mut self, v: impl Into<String>) -> &mut Self {
        *self.access_token_url.write().unwrap() = Some(v.into());
        self
    }

    /// 自定义 API 主机地址（对应 Java `getApiHostUrl()`）。
    pub fn api_host_url(&self) -> Option<String> {
        self.api_host_url.read().unwrap().clone()
    }

    /// 自定义获取 accessToken 地址（对应 Java `getAccessTokenUrl()`）。
    pub fn access_token_url(&self) -> Option<String> {
        self.access_token_url.read().unwrap().clone()
    }

    /// 根据配置获取实际应使用的 API 主机地址（对应 Java
    /// `getEffectiveApiHostUrl()`：自定义 apiHostUrl 优先，否则默认
    /// `https://api.weixin.qq.com`）。
    pub fn effective_api_host_url(&self) -> String {
        if let Some(api_host_url) = self.api_host_url() {
            if !api_host_url.is_empty() {
                return api_host_url;
            }
        }
        API_DEFAULT_HOST_URL.to_string()
    }

    /// 从缓存 map 读取 token 值（过期或缺失返回 None）。
    fn get_token_string(map: &Mutex<HashMap<String, TokenEntry>>, key: &str) -> Option<String> {
        let guard = map.lock().unwrap();
        match guard.get(key) {
            Some(t) if !t.is_expired(now()) => Some(t.value.clone()),
            _ => None,
        }
    }

    /// 强制将缓存 map 中指定 key 的 token 过期（Java `expireToken`：expiresTime=0）。
    fn expire_token(map: &Mutex<HashMap<String, TokenEntry>>, key: &str) {
        let mut guard = map.lock().unwrap();
        guard.remove(key);
    }

    /// 线程安全地更新缓存 map 中指定 key 的 token。
    ///
    /// Java `updateToken` 语义：expiresInSeconds 为 null 或 -1 时不更新过期
    /// 时间（refresh_token 永久有效）；否则 `expiresTime = now +
    /// (expiresInSeconds - 200) * 1000`（预留 200 秒提前过期）。
    fn update_token(
        map: &Mutex<HashMap<String, TokenEntry>>,
        key: &str,
        token: &str,
        expires_in_seconds: Option<i32>,
    ) {
        let mut guard = map.lock().unwrap();
        let entry = guard.entry(key.to_string()).or_insert_with(|| TokenEntry {
            value: String::new(),
            expires_at: None,
        });
        entry.value = token.to_string();
        if let Some(expires_in) = expires_in_seconds {
            if expires_in != -1 {
                entry.expires_at = Some(now() + (expires_in - 200).max(0) as i64);
            }
        }
    }
}

impl WxOpenConfigStorage for WxOpenDefaultConfig {
    fn component_app_id(&self) -> Option<String> {
        self.component_app_id.read().unwrap().clone()
    }

    fn set_component_app_id(&self, component_app_id: &str) {
        *self.component_app_id.write().unwrap() = Some(component_app_id.to_string());
    }

    fn component_app_secret(&self) -> Option<String> {
        self.component_app_secret.read().unwrap().clone()
    }

    fn set_component_app_secret(&self, component_app_secret: &str) {
        *self.component_app_secret.write().unwrap() = Some(component_app_secret.to_string());
    }

    fn component_token(&self) -> Option<String> {
        self.component_token.read().unwrap().clone()
    }

    fn set_component_token(&self, component_token: &str) {
        *self.component_token.write().unwrap() = Some(component_token.to_string());
    }

    fn component_aes_key(&self) -> Option<String> {
        self.component_aes_key.read().unwrap().clone()
    }

    fn set_component_aes_key(&self, component_aes_key: &str) {
        *self.component_aes_key.write().unwrap() = Some(component_aes_key.to_string());
    }

    fn component_verify_ticket(&self) -> Option<String> {
        self.component_verify_ticket.read().unwrap().clone()
    }

    fn set_component_verify_ticket(&self, component_verify_ticket: &str) {
        *self.component_verify_ticket.write().unwrap() = Some(component_verify_ticket.to_string());
    }

    fn component_access_token(&self) -> Option<String> {
        let guard = self.component_access_token.lock().unwrap();
        guard.as_ref().map(|t| t.value.clone())
    }

    fn is_component_access_token_expired(&self) -> bool {
        let guard = self.component_access_token.lock().unwrap();
        match guard.as_ref() {
            Some(t) => t.is_expired(now()),
            None => true,
        }
    }

    fn expire_component_access_token(&self) {
        let mut guard = self.component_access_token.lock().unwrap();
        *guard = None;
    }

    fn update_component_access_token_with_expiry(
        &self,
        component_access_token: &str,
        expires_in_seconds: i32,
    ) {
        let mut guard = self.component_access_token.lock().unwrap();
        *guard = Some(TokenEntry {
            value: component_access_token.to_string(),
            // Java `(expiresInSeconds - 200) * 1000L`：预留 200 秒提前过期
            expires_at: Some(now() + (expires_in_seconds - 200).max(0) as i64),
        });
    }

    fn component_access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.component_access_token_lock.clone()
    }

    fn lock_by_key(&self, key: &str) -> Arc<AsyncMutex<()>> {
        // Java `locks.computeIfAbsent(key, e -> new ReentrantLock())`
        let mut guard = self.locks.lock().unwrap();
        guard
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(AsyncMutex::new(())))
            .clone()
    }

    fn wx_open_host_config(&self) -> Option<WxOpenHostConfig> {
        let mut host_config = self.host_config.read().unwrap().clone();
        // Java `apiHostUrl` 替换语义：自定义 apiHostUrl 优先于默认域名
        if let Some(api_host_url) = self.api_host_url() {
            if !api_host_url.is_empty() {
                host_config.api_host = api_host_url;
            }
        }
        Some(host_config)
    }

    fn authorizer_refresh_token(&self, app_id: &str) -> Option<String> {
        Self::get_token_string(&self.authorizer_refresh_tokens, app_id)
    }

    fn set_authorizer_refresh_token(&self, app_id: &str, authorizer_refresh_token: &str) {
        Self::update_token(
            &self.authorizer_refresh_tokens,
            app_id,
            authorizer_refresh_token,
            None,
        );
    }

    fn authorizer_access_token(&self, app_id: &str) -> Option<String> {
        Self::get_token_string(&self.authorizer_access_tokens, app_id)
    }

    fn is_authorizer_access_token_expired(&self, app_id: &str) -> bool {
        Self::get_token_string(&self.authorizer_access_tokens, app_id).is_none()
    }

    fn expire_authorizer_access_token(&self, app_id: &str) {
        Self::expire_token(&self.authorizer_access_tokens, app_id);
    }

    fn update_authorizer_access_token_with_expiry(
        &self,
        app_id: &str,
        authorizer_access_token: &str,
        expires_in_seconds: i32,
    ) {
        Self::update_token(
            &self.authorizer_access_tokens,
            app_id,
            authorizer_access_token,
            Some(expires_in_seconds),
        );
    }

    fn jsapi_ticket(&self, app_id: &str) -> Option<String> {
        Self::get_token_string(&self.jsapi_tickets, app_id)
    }

    fn is_jsapi_ticket_expired(&self, app_id: &str) -> bool {
        Self::get_token_string(&self.jsapi_tickets, app_id).is_none()
    }

    fn expire_jsapi_ticket(&self, app_id: &str) {
        Self::expire_token(&self.jsapi_tickets, app_id);
    }

    fn update_jsapi_ticket(&self, app_id: &str, jsapi_ticket: &str, expires_in_seconds: i32) {
        Self::update_token(
            &self.jsapi_tickets,
            app_id,
            jsapi_ticket,
            Some(expires_in_seconds),
        );
    }

    fn card_api_ticket(&self, app_id: &str) -> Option<String> {
        Self::get_token_string(&self.card_api_tickets, app_id)
    }

    fn is_card_api_ticket_expired(&self, app_id: &str) -> bool {
        Self::get_token_string(&self.card_api_tickets, app_id).is_none()
    }

    fn expire_card_api_ticket(&self, app_id: &str) {
        Self::expire_token(&self.card_api_tickets, app_id);
    }

    fn update_card_api_ticket(&self, app_id: &str, card_api_ticket: &str, expires_in_seconds: i32) {
        Self::update_token(
            &self.card_api_tickets,
            app_id,
            card_api_ticket,
            Some(expires_in_seconds),
        );
    }

    fn http_proxy_host(&self) -> Option<String> {
        self.http_proxy_host.read().unwrap().clone()
    }

    fn http_proxy_port(&self) -> i32 {
        self.http_proxy_port
    }

    fn http_proxy_username(&self) -> Option<String> {
        self.http_proxy_username.read().unwrap().clone()
    }

    fn http_proxy_password(&self) -> Option<String> {
        self.http_proxy_password.read().unwrap().clone()
    }

    fn retry_sleep_millis(&self) -> i32 {
        self.retry_sleep_millis
    }

    fn max_retry_times(&self) -> i32 {
        self.max_retry_times
    }

    fn component_api_signature_rsa_private_key(&self) -> Option<String> {
        self.component_api_signature_rsa_private_key
            .read()
            .unwrap()
            .clone()
    }

    fn set_component_api_signature_rsa_private_key(&self, api_signature_rsa_private_key: &str) {
        *self
            .component_api_signature_rsa_private_key
            .write()
            .unwrap() = Some(api_signature_rsa_private_key.to_string());
    }

    fn component_api_signature_aes_key(&self) -> Option<String> {
        self.component_api_signature_aes_key.read().unwrap().clone()
    }

    fn set_component_api_signature_aes_key(&self, api_signature_aes_key: &str) {
        *self.component_api_signature_aes_key.write().unwrap() =
            Some(api_signature_aes_key.to_string());
    }

    fn component_api_signature_rsa_private_key_sn(&self) -> Option<String> {
        self.component_api_signature_rsa_private_key_sn
            .read()
            .unwrap()
            .clone()
    }

    fn set_component_api_signature_rsa_private_key_sn(
        &self,
        api_signature_rsa_private_key_sn: &str,
    ) {
        *self
            .component_api_signature_rsa_private_key_sn
            .write()
            .unwrap() = Some(api_signature_rsa_private_key_sn.to_string());
    }

    fn component_api_signature_aes_key_sn(&self) -> Option<String> {
        self.component_api_signature_aes_key_sn
            .read()
            .unwrap()
            .clone()
    }

    fn set_component_api_signature_aes_key_sn(&self, api_signature_aes_key_sn: &str) {
        *self.component_api_signature_aes_key_sn.write().unwrap() =
            Some(api_signature_aes_key_sn.to_string());
    }
}

impl Default for WxOpenDefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}
