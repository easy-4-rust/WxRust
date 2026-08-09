//! 企业微信企业互联默认内存配置存储。
//!
//! 对应 Java `me.chanjar.weixin.cp.config.impl.WxCpCorpGroupDefaultConfigImpl`：
//! 以「企业 ID + 应用 ID」为 key 的 corp access token 内存缓存（值 +
//! 过期时刻 + 独立锁），过期时刻预留 200 秒提前过期（Java
//! `expiresInSeconds - 200` 语义），线程安全。
//!
//! 与 Java 的字段对应关系：
//! - `corpAccessTokenMap`/`corpAccessTokenExpireTimeMap`/
//!   `corpAccessTokenLocker` → `corp_access_tokens: Mutex<HashMap<String,
//!   TokenCell>>`，key 为 `generateAccessTokenKey`（Java：
//!   `corpId:agentId:reqCorpId:reqAgentId`，即本企业 corpId/agentId 与
//!   目标企业 corpId/agentId 的四元组）
//! - `corpId`/`agentId` 为本次互联调用发起方的企业配置（对应 Java
//!   `corpId`/`agentId` 字段）

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::bean::WxAccessToken;
use wx_rust_common::config::TokenEntry;

use crate::config::WxCpCorpGroupConfigStorage;

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// token 缓存单元（值 + 过期时刻 + 独立锁）。
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

/// 企业微信企业互联默认配置存储（内存实现）。
#[derive(Debug)]
pub struct WxCpCorpGroupDefaultConfig {
    /// 微信企业号 corpId（对应 Java `corpId`）。
    corp_id: RwLock<Option<String>>,
    /// 微信企业号应用 ID（对应 Java `agentId`）。
    agent_id: RwLock<Option<i32>>,
    /// 按 access token key 区分的缓存（对应 Java `corpAccessTokenMap` 等）。
    corp_access_tokens: Mutex<HashMap<String, Arc<TokenCell>>>,
    /// 自定义 baseUrl（对应 Java `baseApiUrl`，`None` 走默认域名）。
    base_api_url: RwLock<Option<String>>,
    http_proxy_host: RwLock<Option<String>>,
    http_proxy_port: RwLock<i32>,
    http_proxy_username: RwLock<Option<String>>,
    http_proxy_password: RwLock<Option<String>>,
}

impl WxCpCorpGroupDefaultConfig {
    /// 构建默认配置。
    pub fn new() -> Self {
        Self {
            corp_id: RwLock::new(None),
            agent_id: RwLock::new(None),
            corp_access_tokens: Mutex::new(HashMap::new()),
            base_api_url: RwLock::new(None),
            http_proxy_host: RwLock::new(None),
            http_proxy_port: RwLock::new(0),
            http_proxy_username: RwLock::new(None),
            http_proxy_password: RwLock::new(None),
        }
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

    /// 生成 access token key（对应 Java `generateAccessTokenKey`：
    /// `String.join(":", corpId, agentId, reqCorpId, reqAgentId)`；
    /// 本企业 agentId 为 null 时 Java `String.valueOf(null)` 输出
    /// `"null"`，Rust 对齐）。
    fn generate_access_token_key(&self, corp_id: &str, agent_id: Option<i32>) -> String {
        format!(
            "{}:{}:{}:{}",
            self.corp_id.read().unwrap().clone().unwrap_or_default(),
            self.agent_id
                .read()
                .unwrap()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "null".to_string()),
            corp_id,
            agent_id
                .map(|v| v.to_string())
                .unwrap_or_else(|| "null".to_string()),
        )
    }

    /// 按 key 取缓存单元（不存在时创建，返回 Arc 克隆以脱离锁守卫
    /// 生命周期）。
    fn cell(&self, key: &str) -> Arc<TokenCell> {
        self.corp_access_tokens
            .lock()
            .unwrap()
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(TokenCell::new()))
            .clone()
    }
}

impl Default for WxCpCorpGroupDefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl WxCpCorpGroupConfigStorage for WxCpCorpGroupDefaultConfig {
    fn set_base_api_url(&self, base_url: &str) {
        *self.base_api_url.write().unwrap() = Some(base_url.to_string());
    }

    fn base_api_url(&self) -> Option<String> {
        self.base_api_url.read().unwrap().clone()
    }

    fn update_corp_access_token(
        &self,
        corp_id: &str,
        agent_id: Option<i32>,
        corp_access_token: &str,
        expires_in_seconds: i32,
    ) {
        let key = self.generate_access_token_key(corp_id, agent_id);
        self.cell(&key)
            .update(corp_access_token, expires_in_seconds);
    }

    fn corp_access_token(&self, corp_id: &str, agent_id: Option<i32>) -> Option<String> {
        let key = self.generate_access_token_key(corp_id, agent_id);
        self.cell(&key).get()
    }

    fn corp_access_token_entity(&self, corp_id: &str, agent_id: Option<i32>) -> WxAccessToken {
        // Java：token 缺省按空串、expire 缺省按 0L，
        // expiresIn = (expire - now) / 1000 + 200
        let key = self.generate_access_token_key(corp_id, agent_id);
        let cell = self.cell(&key);
        let remain = cell
            .entry
            .lock()
            .unwrap()
            .as_ref()
            .map(|t| t.expires_at.unwrap_or(0) - now())
            .unwrap_or(-now());
        WxAccessToken {
            access_token: cell.get().unwrap_or_default(),
            expires_in: (remain + 200) as i32,
        }
    }

    fn is_corp_access_token_expired(&self, corp_id: &str, agent_id: Option<i32>) -> bool {
        let key = self.generate_access_token_key(corp_id, agent_id);
        self.cell(&key).is_expired()
    }

    fn expire_corp_access_token(&self, corp_id: &str, agent_id: Option<i32>) {
        let key = self.generate_access_token_key(corp_id, agent_id);
        self.cell(&key).expire();
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

    fn set_corp_id(&self, corp_id: &str) {
        *self.corp_id.write().unwrap() = Some(corp_id.to_string());
    }

    fn corp_id(&self) -> Option<String> {
        self.corp_id.read().unwrap().clone()
    }

    fn set_agent_id(&self, agent_id: Option<i32>) {
        *self.agent_id.write().unwrap() = agent_id;
    }

    fn agent_id(&self) -> Option<i32> {
        *self.agent_id.read().unwrap()
    }

    fn corp_access_token_lock(&self, corp_id: &str, agent_id: Option<i32>) -> Arc<AsyncMutex<()>> {
        let key = self.generate_access_token_key(corp_id, agent_id);
        self.cell(&key).lock.clone()
    }
}
