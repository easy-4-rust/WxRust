//! 微信客户端配置存储抽象。
//!
//! 对应 Java 各模块 `ConfigStorage` 接口的公共部分（token/ticket 缓存、
//! appId/secret、锁语义），上提到 common 供业务模块复用。

use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;

/// access token 缓存条目。
#[derive(Debug, Clone)]
pub struct TokenEntry {
    /// token 值
    pub value: String,
    /// 过期时刻（UNIX 秒）；`None` 表示永不过期（未设置）
    pub expires_at: Option<i64>,
}

impl TokenEntry {
    /// 是否已过期。
    ///
    /// # 参数
    /// - `now`：当前时间（UNIX 秒）
    pub fn is_expired(&self, now: i64) -> bool {
        match self.expires_at {
            Some(t) => t <= now,
            None => false,
        }
    }
}

/// 微信客户端配置存储。
///
/// 对应 Java 各业务模块 `Wx*ConfigStorage` 接口的公共契约（token 缓存、
/// 过期判断、更新与锁）。业务模块在此基础上扩展自身字段
/// （如公众号的 `WxMpConfigStorage` 增加 jsapi ticket 等）。
pub trait WxConfigStorage: Send + Sync {
    /// 返回 appId。
    fn app_id(&self) -> &str;

    /// 返回 appSecret。
    fn secret(&self) -> &str;

    /// 返回当前缓存的 access token。
    fn access_token(&self) -> Option<String>;

    /// 判断 access token 是否已过期。
    fn is_access_token_expired(&self) -> bool;

    /// 强制将 access token 过期掉。
    fn expire_access_token(&self);

    /// 更新 access token（线程安全）。
    ///
    /// # 参数
    /// - `access_token`：新的 access token 值
    /// - `expires_in_seconds`：过期时间（秒）
    fn update_access_token(&self, access_token: &str, expires_in_seconds: i32);

    /// 返回 access token 锁（多线程只刷新一次语义）。
    fn access_token_lock(&self) -> Arc<AsyncMutex<()>>;

    /// 是否使用稳定版 access token 接口（公众号稳定版）。
    fn is_stable_access_token(&self) -> bool {
        false
    }

    /// token 过期时是否自动刷新（默认 true）。
    fn auto_refresh_token(&self) -> bool {
        true
    }

    /// 获取指定类型的 ticket 值。
    fn ticket(&self, _ticket_type: TicketType) -> Option<String> {
        None
    }

    /// 判断指定类型的 ticket 是否已过期。
    fn is_ticket_expired(&self, _ticket_type: TicketType) -> bool {
        true
    }

    /// 更新指定类型的 ticket（线程安全）。
    fn update_ticket(&self, _ticket_type: TicketType, _ticket: &str, _expires_in_seconds: i32) {}

    /// 返回指定类型 ticket 的锁（对应 Java `getTicketLock(TicketType)` 按类型分配锁）。
    fn ticket_lock(&self, _ticket_type: TicketType) -> Arc<AsyncMutex<()>> {
        Arc::new(AsyncMutex::new(()))
    }

    /// 强制将指定类型的 ticket 过期掉（对应 Java `expireTicket`）。
    fn expire_ticket(&self, _ticket_type: TicketType) {}

    /// 返回代理主机（`None` 为不使用代理）。
    fn http_proxy_host(&self) -> Option<&str> {
        None
    }

    /// 返回代理端口。
    fn http_proxy_port(&self) -> Option<u16> {
        None
    }

    /// 返回临时目录。
    fn tmp_dir(&self) -> Option<&str> {
        None
    }
}

/// 默认内存配置存储实现。
///
/// 对应 Java `Wx*DefaultConfigImpl` 的内存语义：token 缓存 + 过期判断 + 锁。
/// 业务模块可用组合方式继承此实现并扩展自身字段。
#[derive(Debug)]
pub struct WxDefaultConfig {
    pub app_id: String,
    pub secret: String,
    /// access token 缓存（含过期时刻）
    access_token: Mutex<Option<TokenEntry>>,
    /// access token 锁
    access_token_lock: Arc<AsyncMutex<()>>,
    /// 是否使用稳定版 token 接口
    pub stable_access_token: bool,
    /// token 过期自动刷新
    pub auto_refresh: bool,
    /// 代理主机
    pub http_proxy_host: Option<String>,
    /// 代理端口
    pub http_proxy_port: Option<u16>,
}

impl WxDefaultConfig {
    /// 构建默认配置。
    ///
    /// # 参数
    /// - `app_id`：appId
    /// - `secret`：appSecret
    pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            access_token: Mutex::new(None),
            access_token_lock: Arc::new(AsyncMutex::new(())),
            stable_access_token: false,
            auto_refresh: true,
            http_proxy_host: None,
            http_proxy_port: None,
        }
    }

    /// 当前时间（UNIX 秒）。
    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }
}

impl WxConfigStorage for WxDefaultConfig {
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
            Some(t) => t.is_expired(Self::now()),
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
            expires_at: Some(Self::now() + expires_in_seconds as i64),
        });
    }

    fn access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.access_token_lock.clone()
    }

    fn is_stable_access_token(&self) -> bool {
        self.stable_access_token
    }

    fn auto_refresh_token(&self) -> bool {
        self.auto_refresh
    }

    fn http_proxy_host(&self) -> Option<&str> {
        self.http_proxy_host.as_deref()
    }

    fn http_proxy_port(&self) -> Option<u16> {
        self.http_proxy_port
    }
}

pub use crate::enums::TicketType;
