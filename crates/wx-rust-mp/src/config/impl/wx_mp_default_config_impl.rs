//! 公众号默认内存配置存储。
//!
//! 对应 Java `WxMpDefaultConfigImpl`：单 appid 的 token/ticket 内存缓存 +
//! 每类型锁 + 过期判断，线程安全。token 语义与 common `WxDefaultConfig` 一致。

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::{TokenEntry, WxConfigStorage};
use wx_rust_common::enums::TicketType;

use crate::config::{WxMpConfigStorage, WxMpHostConfig};

/// 公众号默认配置存储（内存实现）。
#[derive(Debug)]
pub struct WxMpDefaultConfig {
    app_id: String,
    secret: String,
    token: Option<String>,
    aes_key: Option<String>,
    template_id: Option<String>,
    oauth2_redirect_url: Option<String>,
    qr_connect_redirect_url: Option<String>,
    retry_sleep_millis: i32,
    max_retry_times: i32,
    host_config: RwLock<WxMpHostConfig>,
    stable_access_token: AtomicBool,
    auto_refresh_token: AtomicBool,
    access_token: Mutex<Option<TokenEntry>>,
    access_token_lock: Arc<AsyncMutex<()>>,
    tickets: Mutex<HashMap<TicketType, TokenEntry>>,
    ticket_locks: Mutex<HashMap<TicketType, Arc<AsyncMutex<()>>>>,
}

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

impl WxMpDefaultConfig {
    /// 构建默认配置。
    ///
    /// # 参数
    /// - `app_id`：公众号 appId
    /// - `secret`：appSecret
    pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            token: None,
            aes_key: None,
            template_id: None,
            oauth2_redirect_url: None,
            qr_connect_redirect_url: None,
            retry_sleep_millis: 1000,
            max_retry_times: 5,
            host_config: RwLock::new(WxMpHostConfig::new()),
            stable_access_token: AtomicBool::new(false),
            auto_refresh_token: AtomicBool::new(true),
            access_token: Mutex::new(None),
            access_token_lock: Arc::new(AsyncMutex::new(())),
            tickets: Mutex::new(HashMap::new()),
            ticket_locks: Mutex::new(HashMap::new()),
        }
    }

    /// 设置消息校验 token。
    pub fn set_token(&mut self, token: impl Into<String>) -> &mut Self {
        self.token = Some(token.into());
        self
    }

    /// 设置消息加解密 aes key。
    pub fn set_aes_key(&mut self, aes_key: impl Into<String>) -> &mut Self {
        self.aes_key = Some(aes_key.into());
        self
    }

    /// 设置模板消息模板 id。
    pub fn set_template_id(&mut self, template_id: impl Into<String>) -> &mut Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 设置 OAuth2 回调地址。
    pub fn set_oauth2_redirect_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.oauth2_redirect_url = Some(url.into());
        self
    }

    /// 设置扫码连接回调地址。
    pub fn set_qr_connect_redirect_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.qr_connect_redirect_url = Some(url.into());
        self
    }

    /// 设置 HTTP 请求重试间隔（毫秒）。
    pub fn set_retry_sleep_millis(&mut self, millis: i32) -> &mut Self {
        self.retry_sleep_millis = millis;
        self
    }

    /// 设置 HTTP 请求最大重试次数。
    pub fn set_max_retry_times(&mut self, times: i32) -> &mut Self {
        self.max_retry_times = times;
        self
    }
}

impl WxConfigStorage for WxMpDefaultConfig {
    fn app_id(&self) -> &str {
        &self.app_id
    }

    fn secret(&self) -> &str {
        &self.secret
    }

    fn access_token(&self) -> Option<String> {
        let guard = self.access_token.lock().unwrap();
        guard.as_ref().map(|t| t.value.clone())
    }

    fn is_access_token_expired(&self) -> bool {
        let guard = self.access_token.lock().unwrap();
        match guard.as_ref() {
            Some(t) => t.is_expired(now()),
            None => true,
        }
    }

    fn expire_access_token(&self) {
        let mut guard = self.access_token.lock().unwrap();
        *guard = None;
    }

    fn update_access_token(&self, access_token: &str, expires_in_seconds: i32) {
        let mut guard = self.access_token.lock().unwrap();
        *guard = Some(TokenEntry {
            value: access_token.to_string(),
            expires_at: Some(now() + expires_in_seconds as i64),
        });
    }

    fn access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.access_token_lock.clone()
    }

    fn is_stable_access_token(&self) -> bool {
        self.stable_access_token.load(Ordering::SeqCst)
    }

    fn auto_refresh_token(&self) -> bool {
        self.auto_refresh_token.load(Ordering::SeqCst)
    }

    fn ticket(&self, ticket_type: TicketType) -> Option<String> {
        let guard = self.tickets.lock().unwrap();
        guard.get(&ticket_type).map(|t| t.value.clone())
    }

    fn is_ticket_expired(&self, ticket_type: TicketType) -> bool {
        let guard = self.tickets.lock().unwrap();
        match guard.get(&ticket_type) {
            Some(t) => t.is_expired(now()),
            None => true,
        }
    }

    fn expire_ticket(&self, ticket_type: TicketType) {
        let mut guard = self.tickets.lock().unwrap();
        guard.remove(&ticket_type);
    }

    fn update_ticket(&self, ticket_type: TicketType, ticket: &str, expires_in_seconds: i32) {
        let mut guard = self.tickets.lock().unwrap();
        guard.insert(
            ticket_type,
            TokenEntry {
                value: ticket.to_string(),
                expires_at: Some(now() + expires_in_seconds as i64),
            },
        );
    }

    fn ticket_lock(&self, ticket_type: TicketType) -> Arc<AsyncMutex<()>> {
        // Java 按 TicketType 分配独立 Lock；Rust 按类型维护锁表
        let mut guard = self.ticket_locks.lock().unwrap();
        guard
            .entry(ticket_type)
            .or_insert_with(|| Arc::new(AsyncMutex::new(())))
            .clone()
    }
}

impl WxMpConfigStorage for WxMpDefaultConfig {
    fn use_stable_access_token(&self, use_stable_access_token: bool) {
        self.stable_access_token
            .store(use_stable_access_token, Ordering::SeqCst);
    }

    fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    fn aes_key(&self) -> Option<&str> {
        self.aes_key.as_deref()
    }

    fn template_id(&self) -> Option<&str> {
        self.template_id.as_deref()
    }

    fn oauth2_redirect_url(&self) -> Option<&str> {
        self.oauth2_redirect_url.as_deref()
    }

    fn qr_connect_redirect_url(&self) -> Option<&str> {
        self.qr_connect_redirect_url.as_deref()
    }

    fn retry_sleep_millis(&self) -> i32 {
        self.retry_sleep_millis
    }

    fn max_retry_times(&self) -> i32 {
        self.max_retry_times
    }

    fn host_config(&self) -> WxMpHostConfig {
        self.host_config.read().unwrap().clone()
    }

    fn set_host_config(&self, host_config: WxMpHostConfig) {
        *self.host_config.write().unwrap() = host_config;
    }
}
