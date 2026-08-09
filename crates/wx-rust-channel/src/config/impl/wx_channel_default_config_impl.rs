//! 视频号小店默认内存配置存储。
//!
//! 对应 Java `WxChannelDefaultConfigImpl`：单 appid 的 token 内存缓存 +
//! 每类型锁 + 过期判断（预留 200 秒提前过期，Java `expiresAheadInMillis`），
//! 线程安全。token 语义与 common `WxDefaultConfig` 一致。

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::{TokenEntry, WxConfigStorage};

use crate::config::{WxChannelConfig, WxChannelHostConfig};

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// 视频号小店默认配置存储（内存实现）。
#[derive(Debug)]
pub struct WxChannelDefaultConfig {
    app_id: String,
    secret: String,
    token: Option<String>,
    aes_key: Option<String>,
    msg_data_format: Option<String>,
    retry_sleep_millis: RwLock<i32>,
    max_retry_times: RwLock<i32>,
    host_config: RwLock<WxChannelHostConfig>,
    /// 自定义 apiHost 地址（对应 Java `apiHostUrl`）
    api_host_url: RwLock<Option<String>>,
    /// 自定义获取 accessToken 地址（对应 Java `accessTokenUrl`）
    access_token_url: RwLock<Option<String>>,
    /// 是否使用稳定版 access token 接口（对应 Java `stableAccessToken`）
    stable_access_token: AtomicBool,
    /// HTTP 代理（对应 Java `httpProxyHost/Port/Username/Password`）
    http_proxy_host: Option<String>,
    http_proxy_port: Option<u16>,
    http_proxy_username: RwLock<Option<String>>,
    http_proxy_password: RwLock<Option<String>>,
    access_token: Mutex<Option<TokenEntry>>,
    access_token_lock: Arc<AsyncMutex<()>>,
}

impl WxChannelDefaultConfig {
    /// 构建默认配置。
    ///
    /// # 参数
    /// - `app_id`：视频号小店 appid
    /// - `secret`：appSecret
    pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            token: None,
            aes_key: None,
            msg_data_format: None,
            retry_sleep_millis: RwLock::new(1000),
            max_retry_times: RwLock::new(5),
            host_config: RwLock::new(WxChannelHostConfig::new()),
            api_host_url: RwLock::new(None),
            access_token_url: RwLock::new(None),
            stable_access_token: AtomicBool::new(false),
            http_proxy_host: None,
            http_proxy_port: None,
            http_proxy_username: RwLock::new(None),
            http_proxy_password: RwLock::new(None),
            access_token: Mutex::new(None),
            access_token_lock: Arc::new(AsyncMutex::new(())),
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

    /// 设置消息数据格式（如 JSON/XML）。
    pub fn set_msg_data_format(&mut self, msg_data_format: impl Into<String>) -> &mut Self {
        self.msg_data_format = Some(msg_data_format.into());
        self
    }

    /// 设置是否使用稳定版 access token 接口。
    pub fn set_stable_access_token(&mut self, stable_access_token: bool) -> &mut Self {
        self.stable_access_token
            .store(stable_access_token, Ordering::SeqCst);
        self
    }

    /// 设置 HTTP 代理（host/port/username/password）。
    pub fn set_http_proxy(
        &mut self,
        host: Option<impl Into<String>>,
        port: Option<u16>,
        username: Option<impl Into<String>>,
        password: Option<impl Into<String>>,
    ) -> &mut Self {
        if let Some(host) = host {
            self.http_proxy_host = Some(host.into());
        }
        if let Some(port) = port {
            self.http_proxy_port = Some(port);
        }
        if let Some(username) = username {
            *self.http_proxy_username.write().unwrap() = Some(username.into());
        }
        if let Some(password) = password {
            *self.http_proxy_password.write().unwrap() = Some(password.into());
        }
        self
    }

    /// 设置 HTTP 请求重试间隔（毫秒）。
    pub fn set_retry_sleep_millis(&mut self, millis: i32) -> &mut Self {
        *self.retry_sleep_millis.write().unwrap() = millis;
        self
    }

    /// 设置 HTTP 请求最大重试次数。
    pub fn set_max_retry_times(&mut self, times: i32) -> &mut Self {
        *self.max_retry_times.write().unwrap() = times;
        self
    }
}

impl WxConfigStorage for WxChannelDefaultConfig {
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
        // Java `expiresAheadInMillis`：预留 200 秒提前过期
        let mut guard = self.access_token.lock().unwrap();
        *guard = Some(TokenEntry {
            value: access_token.to_string(),
            expires_at: Some(now() + (expires_in_seconds - 200) as i64),
        });
    }

    fn access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.access_token_lock.clone()
    }

    fn is_stable_access_token(&self) -> bool {
        self.stable_access_token.load(Ordering::SeqCst)
    }

    fn auto_refresh_token(&self) -> bool {
        true
    }

    fn http_proxy_host(&self) -> Option<&str> {
        self.http_proxy_host.as_deref()
    }

    fn http_proxy_port(&self) -> Option<u16> {
        self.http_proxy_port
    }
}

impl WxChannelConfig for WxChannelDefaultConfig {
    fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    fn aes_key(&self) -> Option<&str> {
        self.aes_key.as_deref()
    }

    fn msg_data_format(&self) -> Option<&str> {
        self.msg_data_format.as_deref()
    }

    fn expires_time(&self) -> i64 {
        let guard = self.access_token.lock().unwrap();
        guard
            .as_ref()
            .and_then(|t| t.expires_at)
            .map(|secs| secs * 1000)
            .unwrap_or(0)
    }

    fn http_proxy_username(&self) -> Option<String> {
        self.http_proxy_username.read().unwrap().clone()
    }

    fn http_proxy_password(&self) -> Option<String> {
        self.http_proxy_password.read().unwrap().clone()
    }

    fn retry_sleep_millis(&self) -> i32 {
        *self.retry_sleep_millis.read().unwrap()
    }

    fn set_retry_sleep_millis(&self, retry_sleep_millis: i32) {
        // 对应 Java `WxChannelDefaultConfigImpl.setRetrySleepMillis`（服务端
        // `BaseWxChannelServiceImpl.setRetrySleepMillis` 在 Rust 中委托配置）
        *self.retry_sleep_millis.write().unwrap() = retry_sleep_millis;
    }

    fn max_retry_times(&self) -> i32 {
        *self.max_retry_times.read().unwrap()
    }

    fn set_max_retry_times(&self, max_retry_times: i32) {
        *self.max_retry_times.write().unwrap() = max_retry_times;
    }

    fn host_config(&self) -> WxChannelHostConfig {
        self.host_config.read().unwrap().clone()
    }

    fn set_host_config(&self, host_config: WxChannelHostConfig) {
        *self.host_config.write().unwrap() = host_config;
    }

    fn api_host_url(&self) -> Option<String> {
        self.api_host_url.read().unwrap().clone()
    }

    fn set_api_host_url(&self, api_host_url: &str) {
        *self.api_host_url.write().unwrap() = Some(api_host_url.to_string());
    }

    fn access_token_url(&self) -> Option<String> {
        self.access_token_url.read().unwrap().clone()
    }

    fn set_access_token_url(&self, access_token_url: &str) {
        *self.access_token_url.write().unwrap() = Some(access_token_url.to_string());
    }
}
