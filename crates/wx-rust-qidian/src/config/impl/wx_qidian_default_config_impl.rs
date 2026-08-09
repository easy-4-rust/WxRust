//! 腾讯企点默认内存配置存储。
//!
//! 对应 Java `me.chanjar.weixin.qidian.config.impl.WxQidianDefaultConfigImpl`：
//! 单 appid 的 token/ticket 内存缓存 + 每类型锁 + 过期判断（预留 200 秒
//! 提前过期，对应 Java `(expiresInSeconds - 200) * 1000L`），线程安全。
//! token 语义与 common `WxDefaultConfig` 一致。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::{TokenEntry, WxConfigStorage};
use wx_rust_common::enums::TicketType;

use crate::bean::WxQidianHostConfig;
use crate::config::WxQidianConfigStorage;

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// 腾讯企点默认配置（内存实现）。
pub struct WxQidianDefaultConfig {
    app_id: String,
    secret: String,
    token: Option<String>,
    aes_key: Option<String>,
    template_id: Option<String>,
    oauth2_redirect_uri: Option<String>,
    http_proxy_host: Option<String>,
    http_proxy_port: Option<u16>,
    http_proxy_username: Option<String>,
    http_proxy_password: Option<String>,
    host_config: RwLock<WxQidianHostConfig>,
    access_token: Mutex<Option<TokenEntry>>,
    access_token_lock: Arc<AsyncMutex<()>>,
    tickets: Mutex<HashMap<TicketType, TokenEntry>>,
    ticket_locks: Mutex<HashMap<TicketType, Arc<AsyncMutex<()>>>>,
}

impl WxQidianDefaultConfig {
    /// 构建默认配置。
    ///
    /// # 参数
    /// - `app_id`：企点 appId
    /// - `secret`：appSecret
    pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            token: None,
            aes_key: None,
            template_id: None,
            oauth2_redirect_uri: None,
            http_proxy_host: None,
            http_proxy_port: None,
            http_proxy_username: None,
            http_proxy_password: None,
            host_config: RwLock::new(WxQidianHostConfig::new()),
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

    /// 设置模板 id。
    pub fn set_template_id(&mut self, template_id: impl Into<String>) -> &mut Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 设置 OAuth2 授权回调地址。
    pub fn set_oauth2_redirect_uri(&mut self, uri: impl Into<String>) -> &mut Self {
        self.oauth2_redirect_uri = Some(uri.into());
        self
    }

    /// 设置 HTTP 代理。
    pub fn set_http_proxy(
        &mut self,
        host: impl Into<String>,
        port: u16,
        username: Option<impl Into<String>>,
        password: Option<impl Into<String>>,
    ) -> &mut Self {
        self.http_proxy_host = Some(host.into());
        self.http_proxy_port = Some(port);
        self.http_proxy_username = username.map(|u| u.into());
        self.http_proxy_password = password.map(|p| p.into());
        self
    }
}

impl WxConfigStorage for WxQidianDefaultConfig {
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
        // 对应 Java：预留 200 秒提前过期
        *guard = Some(TokenEntry {
            value: access_token.to_string(),
            expires_at: Some(now() + (expires_in_seconds - 200).max(0) as i64),
        });
    }

    fn access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.access_token_lock.clone()
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
        // 对应 Java：预留 200 秒提前过期
        guard.insert(
            ticket_type,
            TokenEntry {
                value: ticket.to_string(),
                expires_at: Some(now() + (expires_in_seconds - 200).max(0) as i64),
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

    fn http_proxy_host(&self) -> Option<&str> {
        self.http_proxy_host.as_deref()
    }

    fn http_proxy_port(&self) -> Option<u16> {
        self.http_proxy_port
    }
}

impl WxQidianConfigStorage for WxQidianDefaultConfig {
    fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    fn aes_key(&self) -> Option<&str> {
        self.aes_key.as_deref()
    }

    fn template_id(&self) -> Option<&str> {
        self.template_id.as_deref()
    }

    fn expires_time(&self) -> i64 {
        let guard = self.access_token.lock().unwrap();
        guard.as_ref().and_then(|t| t.expires_at).unwrap_or(0)
    }

    fn oauth2_redirect_uri(&self) -> Option<&str> {
        self.oauth2_redirect_uri.as_deref()
    }

    fn http_proxy_username(&self) -> Option<&str> {
        self.http_proxy_username.as_deref()
    }

    fn http_proxy_password(&self) -> Option<&str> {
        self.http_proxy_password.as_deref()
    }

    fn host_config(&self) -> WxQidianHostConfig {
        self.host_config.read().unwrap().clone()
    }

    fn set_host_config(&self, host_config: WxQidianHostConfig) {
        *self.host_config.write().unwrap() = host_config;
    }
}
