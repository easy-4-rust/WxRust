//! 小程序默认内存配置存储。
//!
//! 对应 Java `WxMaDefaultConfigImpl`：单 appid 的 token/ticket 内存缓存 +
//! 每类型锁 + 过期判断（预留 200 秒提前过期，Java `expiresAheadInMillis`），
//! 线程安全。token 语义与 common `WxDefaultConfig` 一致。

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::{TokenEntry, WxConfigStorage};
use wx_rust_common::enums::TicketType;

use crate::config::{WxMaConfig, WxMaHostConfig};

/// 当前时间（UNIX 秒）。
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// 小程序默认配置存储（内存实现）。
#[derive(Debug)]
pub struct WxMaDefaultConfig {
    app_id: String,
    secret: String,
    token: Option<String>,
    aes_key: Option<String>,
    original_id: Option<String>,
    cloud_env: Option<String>,
    msg_data_format: Option<String>,
    retry_sleep_millis: i32,
    max_retry_times: i32,
    host_config: RwLock<WxMaHostConfig>,
    /// 自定义 apiHost 地址（对应 Java `apiHostUrl`）
    api_host_url: RwLock<Option<String>>,
    /// 自定义获取 accessToken 地址（对应 Java `accessTokenUrl`）
    access_token_url: RwLock<Option<String>>,
    stable_access_token: AtomicBool,
    auto_refresh_token: AtomicBool,
    /// 是否使用微信云托管内网模式（对应 Java `useWxCloudRun`）
    use_wx_cloud_run: AtomicBool,
    access_token: Mutex<Option<TokenEntry>>,
    access_token_lock: Arc<AsyncMutex<()>>,
    tickets: Mutex<HashMap<TicketType, TokenEntry>>,
    ticket_locks: Mutex<HashMap<TicketType, Arc<AsyncMutex<()>>>>,
    /// 服务端 API 签名 RSA 私钥（pkcs8，对应 Java `apiSignatureRsaPrivateKey`）
    api_signature_rsa_private_key: RwLock<Option<String>>,
    /// 服务端 API 签名 AES 密钥（对应 Java `apiSignatureAesKey`）
    api_signature_aes_key: RwLock<Option<String>>,
    /// API 签名 RSA 私钥序号（对应 Java `apiSignatureRsaPrivateKeySn`）
    api_signature_rsa_private_key_sn: RwLock<Option<String>>,
    /// API 签名 AES 密钥序号（对应 Java `apiSignatureAesKeySn`）
    api_signature_aes_key_sn: RwLock<Option<String>>,
    /// 签名用小程序 ID（普通小程序为 appId，对应 Java `wechatMpAppid`）
    wechat_mp_appid: RwLock<Option<String>>,
}

impl WxMaDefaultConfig {
    /// 构建默认配置。
    ///
    /// # 参数
    /// - `app_id`：小程序 appId
    /// - `secret`：appSecret
    pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            token: None,
            aes_key: None,
            original_id: None,
            cloud_env: None,
            msg_data_format: None,
            retry_sleep_millis: 1000,
            max_retry_times: 5,
            host_config: RwLock::new(WxMaHostConfig::new()),
            api_host_url: RwLock::new(None),
            access_token_url: RwLock::new(None),
            stable_access_token: AtomicBool::new(false),
            auto_refresh_token: AtomicBool::new(true),
            use_wx_cloud_run: AtomicBool::new(false),
            access_token: Mutex::new(None),
            access_token_lock: Arc::new(AsyncMutex::new(())),
            tickets: Mutex::new(HashMap::new()),
            ticket_locks: Mutex::new(HashMap::new()),
            api_signature_rsa_private_key: RwLock::new(None),
            api_signature_aes_key: RwLock::new(None),
            api_signature_rsa_private_key_sn: RwLock::new(None),
            api_signature_aes_key_sn: RwLock::new(None),
            wechat_mp_appid: RwLock::new(None),
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

    /// 设置原始 ID。
    pub fn set_original_id(&mut self, original_id: impl Into<String>) -> &mut Self {
        self.original_id = Some(original_id.into());
        self
    }

    /// 设置云开发环境标识。
    pub fn set_cloud_env(&mut self, cloud_env: impl Into<String>) -> &mut Self {
        self.cloud_env = Some(cloud_env.into());
        self
    }

    /// 设置消息数据格式（如 JSON/XML）。
    pub fn set_msg_data_format(&mut self, msg_data_format: impl Into<String>) -> &mut Self {
        self.msg_data_format = Some(msg_data_format.into());
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

    /// 设置服务端 API 签名 RSA 私钥（pkcs8 格式）。
    pub fn set_api_signature_rsa_private_key(&mut self, key: impl Into<String>) -> &mut Self {
        *self.api_signature_rsa_private_key.write().unwrap() = Some(key.into());
        self
    }

    /// 设置服务端 API 签名 AES 密钥。
    pub fn set_api_signature_aes_key(&mut self, key: impl Into<String>) -> &mut Self {
        *self.api_signature_aes_key.write().unwrap() = Some(key.into());
        self
    }

    /// 设置 API 签名 RSA 私钥序号。
    pub fn set_api_signature_rsa_private_key_sn(&mut self, sn: impl Into<String>) -> &mut Self {
        *self.api_signature_rsa_private_key_sn.write().unwrap() = Some(sn.into());
        self
    }

    /// 设置 API 签名 AES 密钥序号。
    pub fn set_api_signature_aes_key_sn(&mut self, sn: impl Into<String>) -> &mut Self {
        *self.api_signature_aes_key_sn.write().unwrap() = Some(sn.into());
        self
    }

    /// 设置签名用小程序 ID（普通小程序为 appId，托管第三方平台为 componentAppId）。
    pub fn set_wechat_mp_appid(&mut self, appid: impl Into<String>) -> &mut Self {
        *self.wechat_mp_appid.write().unwrap() = Some(appid.into());
        self
    }
}

impl WxConfigStorage for WxMaDefaultConfig {
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

impl WxMaConfig for WxMaDefaultConfig {
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

    fn original_id(&self) -> Option<&str> {
        self.original_id.as_deref()
    }

    fn cloud_env(&self) -> Option<&str> {
        self.cloud_env.as_deref()
    }

    fn msg_data_format(&self) -> Option<&str> {
        self.msg_data_format.as_deref()
    }

    fn retry_sleep_millis(&self) -> i32 {
        self.retry_sleep_millis
    }

    fn max_retry_times(&self) -> i32 {
        self.max_retry_times
    }

    fn host_config(&self) -> WxMaHostConfig {
        self.host_config.read().unwrap().clone()
    }

    fn set_host_config(&self, host_config: WxMaHostConfig) {
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

    fn api_signature_rsa_private_key(&self) -> Option<String> {
        self.api_signature_rsa_private_key.read().unwrap().clone()
    }

    fn api_signature_aes_key(&self) -> Option<String> {
        self.api_signature_aes_key.read().unwrap().clone()
    }

    fn api_signature_aes_key_sn(&self) -> Option<String> {
        self.api_signature_aes_key_sn.read().unwrap().clone()
    }

    fn api_signature_rsa_private_key_sn(&self) -> Option<String> {
        self.api_signature_rsa_private_key_sn
            .read()
            .unwrap()
            .clone()
    }

    fn wechat_mp_appid(&self) -> Option<String> {
        self.wechat_mp_appid.read().unwrap().clone()
    }

    fn is_use_wx_cloud_run(&self) -> bool {
        self.use_wx_cloud_run.load(Ordering::SeqCst)
    }

    fn set_use_wx_cloud_run(&self, use_wx_cloud_run: bool) {
        self.use_wx_cloud_run
            .store(use_wx_cloud_run, Ordering::SeqCst);
    }
}
