//! 企业微信默认内存配置存储。
//!
//! 对应 Java `WxCpDefaultConfigImpl`：单应用的 token/ticket 内存缓存 +
//! 每类型锁 + 过期判断（预留 200 秒提前过期，Java `expiresInSeconds - 200`
//! 语义，见 `updateAccessToken`/`updateJsapiTicket`/`updateContactAccessToken`
//! 等），线程安全。
//!
//! 与 Java 的字段对应关系：
//! - `corpId`/`corpSecret` → common `WxConfigStorage` 的 `app_id`/`secret`
//! - `accessToken`/`expiresTime`/`accessTokenLock` → common 的 access_token
//!   缓存/锁（过期时刻换算：Java 毫秒 → Rust `TokenEntry` 秒）
//! - `jsapiTicket`/`jsapiTicketLock` → common 的 `TicketType::Jsapi` ticket
//!   缓存/锁
//! - `agentJsapiTicket`/`contactAccessToken`/`msgAuditAccessToken` →
//!   本实现 `TokenCell`（值 + 过期时刻 + 独立 AsyncMutex）

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::{TokenEntry, WxConfigStorage};
use wx_rust_common::enums::TicketType;

use crate::config::{WxCpConfigStorage, WxCpHostConfig};

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// token 缓存单元（值 + 过期时刻 + 独立锁）。
///
/// 对应 Java 每类 token 的「字段 + Lock」组合（agentJsapiTicket/
/// contactAccessToken/msgAuditAccessToken）。
#[derive(Debug)]
struct TokenCell {
    entry: Mutex<Option<TokenEntry>>,
    lock: Arc<AsyncMutex<()>>,
}

impl TokenCell {
    fn new() -> Self {
        Self {
            entry: Mutex::new(None),
            lock: Arc::new(AsyncMutex::new(())),
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
}

/// 企业微信默认配置存储（内存实现）。
#[derive(Debug)]
pub struct WxCpDefaultConfig {
    app_id: String,
    secret: String,
    agent_id: RwLock<Option<i32>>,
    token: RwLock<Option<String>>,
    aes_key: RwLock<Option<String>>,
    oauth2_redirect_uri: RwLock<Option<String>>,
    http_proxy_username: RwLock<Option<String>>,
    http_proxy_password: RwLock<Option<String>>,
    tmp_dir_file: RwLock<Option<String>>,
    webhook_key: RwLock<Option<String>>,
    retry_sleep_millis: i32,
    max_retry_times: i32,
    /// 自定义 baseUrl（对应 Java `baseApiUrl`，`None` 走默认域名）
    base_api_url: RwLock<Option<String>>,
    host_config: RwLock<WxCpHostConfig>,
    auto_refresh_token: AtomicBool,
    /// 主 access token 缓存（含过期时刻，对应 Java `accessToken`/`expiresTime`）
    access_token: Mutex<Option<TokenEntry>>,
    access_token_lock: Arc<AsyncMutex<()>>,
    /// jsapi ticket 缓存（`TicketType::Jsapi`，对应 Java `jsapiTicket`）
    tickets: Mutex<HashMap<TicketType, TokenEntry>>,
    ticket_locks: Mutex<HashMap<TicketType, Arc<AsyncMutex<()>>>>,
    /// 应用 jsapi ticket（对应 Java `agentJsapiTicket`）
    agent_jsapi_ticket: TokenCell,
    /// 通讯录同步 access token（对应 Java `contactAccessToken`）
    contact_access_token: TokenCell,
    /// 会话存档 access token（对应 Java `msgAuditAccessToken`）
    msg_audit_access_token: TokenCell,
    contact_secret: RwLock<Option<String>>,
    msg_audit_secret: RwLock<Option<String>>,
    msg_audit_pri_key: RwLock<Option<String>>,
    msg_audit_lib_path: RwLock<Option<String>>,
}

impl WxCpDefaultConfig {
    /// 构建默认配置。
    ///
    /// # 参数
    /// - `corp_id`：企业 corpid（对应 Java `setCorpId`）
    /// - `corp_secret`：企业 secret（对应 Java `setCorpSecret`）
    pub fn new(corp_id: impl Into<String>, corp_secret: impl Into<String>) -> Self {
        Self {
            app_id: corp_id.into(),
            secret: corp_secret.into(),
            agent_id: RwLock::new(None),
            token: RwLock::new(None),
            aes_key: RwLock::new(None),
            oauth2_redirect_uri: RwLock::new(None),
            http_proxy_username: RwLock::new(None),
            http_proxy_password: RwLock::new(None),
            tmp_dir_file: RwLock::new(None),
            webhook_key: RwLock::new(None),
            retry_sleep_millis: 1000,
            max_retry_times: 5,
            base_api_url: RwLock::new(None),
            host_config: RwLock::new(WxCpHostConfig::new()),
            auto_refresh_token: AtomicBool::new(true),
            access_token: Mutex::new(None),
            access_token_lock: Arc::new(AsyncMutex::new(())),
            tickets: Mutex::new(HashMap::new()),
            ticket_locks: Mutex::new(HashMap::new()),
            agent_jsapi_ticket: TokenCell::new(),
            contact_access_token: TokenCell::new(),
            msg_audit_access_token: TokenCell::new(),
            contact_secret: RwLock::new(None),
            msg_audit_secret: RwLock::new(None),
            msg_audit_pri_key: RwLock::new(None),
            msg_audit_lib_path: RwLock::new(None),
        }
    }

    /// 设置应用 agentid（对应 Java `setAgentId(Integer)`）。
    pub fn set_agent_id(&mut self, agent_id: Option<i32>) -> &mut Self {
        *self.agent_id.write().unwrap() = agent_id;
        self
    }

    /// 设置消息校验 token（对应 Java `setToken(String)`）。
    pub fn set_token(&mut self, token: impl Into<String>) -> &mut Self {
        *self.token.write().unwrap() = Some(token.into());
        self
    }

    /// 设置消息加解密 aes key（对应 Java `setAesKey(String)`）。
    pub fn set_aes_key(&mut self, aes_key: impl Into<String>) -> &mut Self {
        *self.aes_key.write().unwrap() = Some(aes_key.into());
        self
    }

    /// 设置 OAuth2 回调地址（对应 Java `setOauth2redirectUri(String)`）。
    pub fn set_oauth2_redirect_uri(&mut self, oauth2_redirect_uri: impl Into<String>) -> &mut Self {
        *self.oauth2_redirect_uri.write().unwrap() = Some(oauth2_redirect_uri.into());
        self
    }

    /// 设置 HTTP 代理用户名（对应 Java `setHttpProxyUsername(String)`）。
    pub fn set_http_proxy_username(&mut self, username: impl Into<String>) -> &mut Self {
        *self.http_proxy_username.write().unwrap() = Some(username.into());
        self
    }

    /// 设置 HTTP 代理密码（对应 Java `setHttpProxyPassword(String)`）。
    pub fn set_http_proxy_password(&mut self, password: impl Into<String>) -> &mut Self {
        *self.http_proxy_password.write().unwrap() = Some(password.into());
        self
    }

    /// 设置临时文件目录（对应 Java `setTmpDirFile(File)`）。
    pub fn set_tmp_dir_file(&mut self, tmp_dir_file: impl Into<String>) -> &mut Self {
        *self.tmp_dir_file.write().unwrap() = Some(tmp_dir_file.into());
        self
    }

    /// 设置群机器人 webhook 的 key（对应 Java `setWebhookKey(String)`）。
    pub fn set_webhook_key(&mut self, webhook_key: impl Into<String>) -> &mut Self {
        *self.webhook_key.write().unwrap() = Some(webhook_key.into());
        self
    }

    /// 设置通讯录同步的 secret（对应 Java `setContactSecret(String)`）。
    pub fn set_contact_secret(&mut self, contact_secret: impl Into<String>) -> &mut Self {
        *self.contact_secret.write().unwrap() = Some(contact_secret.into());
        self
    }

    /// 设置会话存档的 secret（对应 Java `setMsgAuditSecret(String)`）。
    pub fn set_msg_audit_secret(&mut self, msg_audit_secret: impl Into<String>) -> &mut Self {
        *self.msg_audit_secret.write().unwrap() = Some(msg_audit_secret.into());
        self
    }

    /// 设置企微会话存档私钥（对应 Java `setMsgAuditPriKey(String)`）。
    pub fn set_msg_audit_pri_key(&mut self, msg_audit_pri_key: impl Into<String>) -> &mut Self {
        *self.msg_audit_pri_key.write().unwrap() = Some(msg_audit_pri_key.into());
        self
    }

    /// 设置企微会话存档系统库绝对路径（对应 Java `setMsgAuditLibPath(String)`）。
    pub fn set_msg_audit_lib_path(&mut self, msg_audit_lib_path: impl Into<String>) -> &mut Self {
        *self.msg_audit_lib_path.write().unwrap() = Some(msg_audit_lib_path.into());
        self
    }
}

impl WxConfigStorage for WxCpDefaultConfig {
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
        // Java WxCpDefaultConfigImpl.updateAccessToken：预留 200 秒提前过期
        let mut guard = self.access_token.lock().unwrap();
        *guard = Some(TokenEntry {
            value: access_token.to_string(),
            expires_at: Some(now() + (expires_in_seconds - 200).max(0) as i64),
        });
    }

    fn access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.access_token_lock.clone()
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
        // Java WxCpDefaultConfigImpl.updateJsapiTicket：预留 200 秒提前过期
        let mut guard = self.tickets.lock().unwrap();
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
}

impl WxCpConfigStorage for WxCpDefaultConfig {
    fn set_base_api_url(&self, base_url: &str) {
        *self.base_api_url.write().unwrap() = Some(base_url.to_string());
    }

    fn base_api_url(&self) -> Option<String> {
        self.base_api_url.read().unwrap().clone()
    }

    fn host_config(&self) -> WxCpHostConfig {
        self.host_config.read().unwrap().clone()
    }

    fn set_host_config(&self, host_config: WxCpHostConfig) {
        *self.host_config.write().unwrap() = host_config;
    }

    fn agent_id(&self) -> Option<i32> {
        *self.agent_id.read().unwrap()
    }

    fn set_agent_id(&self, agent_id: Option<i32>) {
        *self.agent_id.write().unwrap() = agent_id;
    }

    fn token(&self) -> Option<String> {
        self.token.read().unwrap().clone()
    }

    fn aes_key(&self) -> Option<String> {
        self.aes_key.read().unwrap().clone()
    }

    fn expires_time(&self) -> i64 {
        // Java `expiresTime` 为毫秒；Rust `TokenEntry` 内部为 UNIX 秒
        let guard = self.access_token.lock().unwrap();
        guard
            .as_ref()
            .and_then(|t| t.expires_at)
            .map(|secs| secs * 1000)
            .unwrap_or(0)
    }

    fn set_expires_time(&self, expires_time: i64) {
        let mut guard = self.access_token.lock().unwrap();
        *guard = match guard.take() {
            Some(mut t) => {
                t.expires_at = Some(expires_time / 1000);
                Some(t)
            }
            None => Some(TokenEntry {
                value: String::new(),
                expires_at: Some(expires_time / 1000),
            }),
        };
    }

    fn oauth2_redirect_uri(&self) -> Option<String> {
        self.oauth2_redirect_uri.read().unwrap().clone()
    }

    fn set_oauth2_redirect_uri(&self, oauth2_redirect_uri: &str) {
        *self.oauth2_redirect_uri.write().unwrap() = Some(oauth2_redirect_uri.to_string());
    }

    fn http_proxy_username(&self) -> Option<String> {
        self.http_proxy_username.read().unwrap().clone()
    }

    fn set_http_proxy_username(&self, username: &str) {
        *self.http_proxy_username.write().unwrap() = Some(username.to_string());
    }

    fn http_proxy_password(&self) -> Option<String> {
        self.http_proxy_password.read().unwrap().clone()
    }

    fn set_http_proxy_password(&self, password: &str) {
        *self.http_proxy_password.write().unwrap() = Some(password.to_string());
    }

    fn tmp_dir_file(&self) -> Option<String> {
        self.tmp_dir_file.read().unwrap().clone()
    }

    fn set_tmp_dir_file(&self, tmp_dir_file: &str) {
        *self.tmp_dir_file.write().unwrap() = Some(tmp_dir_file.to_string());
    }

    fn webhook_key(&self) -> Option<String> {
        self.webhook_key.read().unwrap().clone()
    }

    fn set_webhook_key(&self, webhook_key: &str) {
        *self.webhook_key.write().unwrap() = Some(webhook_key.to_string());
    }

    fn retry_sleep_millis(&self) -> i32 {
        self.retry_sleep_millis
    }

    fn max_retry_times(&self) -> i32 {
        self.max_retry_times
    }

    fn contact_secret(&self) -> Option<String> {
        self.contact_secret.read().unwrap().clone()
    }

    fn set_contact_secret(&self, contact_secret: &str) {
        *self.contact_secret.write().unwrap() = Some(contact_secret.to_string());
    }

    fn contact_access_token(&self) -> Option<String> {
        self.contact_access_token.get()
    }

    fn contact_access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.contact_access_token.lock.clone()
    }

    fn is_contact_access_token_expired(&self) -> bool {
        self.contact_access_token.is_expired()
    }

    fn expire_contact_access_token(&self) {
        self.contact_access_token.expire();
    }

    fn update_contact_access_token(&self, access_token: &str, expires_in_seconds: i32) {
        // Java：预留 200 秒提前过期
        self.contact_access_token
            .update(access_token, expires_in_seconds);
    }

    fn msg_audit_secret(&self) -> Option<String> {
        self.msg_audit_secret.read().unwrap().clone()
    }

    fn set_msg_audit_secret(&self, msg_audit_secret: &str) {
        *self.msg_audit_secret.write().unwrap() = Some(msg_audit_secret.to_string());
    }

    fn msg_audit_pri_key(&self) -> Option<String> {
        self.msg_audit_pri_key.read().unwrap().clone()
    }

    fn set_msg_audit_pri_key(&self, msg_audit_pri_key: &str) {
        *self.msg_audit_pri_key.write().unwrap() = Some(msg_audit_pri_key.to_string());
    }

    fn msg_audit_lib_path(&self) -> Option<String> {
        self.msg_audit_lib_path.read().unwrap().clone()
    }

    fn set_msg_audit_lib_path(&self, msg_audit_lib_path: &str) {
        *self.msg_audit_lib_path.write().unwrap() = Some(msg_audit_lib_path.to_string());
    }

    fn msg_audit_access_token(&self) -> Option<String> {
        self.msg_audit_access_token.get()
    }

    fn msg_audit_access_token_lock(&self) -> Arc<AsyncMutex<()>> {
        self.msg_audit_access_token.lock.clone()
    }

    fn is_msg_audit_access_token_expired(&self) -> bool {
        self.msg_audit_access_token.is_expired()
    }

    fn expire_msg_audit_access_token(&self) {
        self.msg_audit_access_token.expire();
    }

    fn update_msg_audit_access_token(&self, access_token: &str, expires_in_seconds: i32) {
        // Java：预留 200 秒提前过期
        self.msg_audit_access_token
            .update(access_token, expires_in_seconds);
    }

    fn agent_jsapi_ticket(&self) -> Option<String> {
        self.agent_jsapi_ticket.get()
    }

    fn agent_jsapi_ticket_lock(&self) -> Arc<AsyncMutex<()>> {
        self.agent_jsapi_ticket.lock.clone()
    }

    fn is_agent_jsapi_ticket_expired(&self) -> bool {
        self.agent_jsapi_ticket.is_expired()
    }

    fn expire_agent_jsapi_ticket(&self) {
        self.agent_jsapi_ticket.expire();
    }

    fn update_agent_jsapi_ticket(&self, jsapi_ticket: &str, expires_in_seconds: i32) {
        // Java：预留 200 秒提前过期
        self.agent_jsapi_ticket
            .update(jsapi_ticket, expires_in_seconds);
    }
}
