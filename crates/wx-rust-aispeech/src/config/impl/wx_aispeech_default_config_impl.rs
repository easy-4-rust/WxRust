//! 智能对话默认内存配置存储。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.config.impl.WxAispeechDefaultConfigImpl`：
//! 全字段内存缓存（appid/token/aesKey/openAiToken/secretKey/代理等），
//! 线程安全。`open_ai_token` 由 `getAccessToken` 成功后运行时写入
//! （对应 Java `setOpenAiToken`，以 `Mutex` 承载内部可变性），其余字段在
//! 构建期配置（对应 Java 字段初始值）。

use std::sync::Mutex;

use crate::config::WxAispeechConfigStorage;
use crate::config::wx_aispeech_config_storage::{
    DEFAULT_DIALOG_API_BASE_URL, DEFAULT_KNOWLEDGE_API_BASE_URL,
};

/// 智能对话默认配置（内存实现）。
pub struct WxAispeechDefaultConfig {
    appid: Option<String>,
    token: Option<String>,
    aes_key: Option<String>,
    open_ai_token: Mutex<Option<String>>,
    secret_key: Option<String>,
    dialog_api_base_url: String,
    knowledge_api_base_url: String,
    http_proxy_host: Option<String>,
    http_proxy_port: i32,
    http_proxy_username: Option<String>,
    http_proxy_password: Option<String>,
}

impl WxAispeechDefaultConfig {
    /// 构建默认配置。
    pub fn new() -> Self {
        Self {
            appid: None,
            token: None,
            aes_key: None,
            open_ai_token: Mutex::new(None),
            secret_key: None,
            dialog_api_base_url: DEFAULT_DIALOG_API_BASE_URL.to_string(),
            knowledge_api_base_url: DEFAULT_KNOWLEDGE_API_BASE_URL.to_string(),
            http_proxy_host: None,
            http_proxy_port: 0,
            http_proxy_username: None,
            http_proxy_password: None,
        }
    }

    /// 设置 appid。
    pub fn set_appid(&mut self, appid: impl Into<String>) -> &mut Self {
        self.appid = Some(appid.into());
        self
    }

    /// 设置消息校验 token。
    pub fn set_token(&mut self, token: impl Into<String>) -> &mut Self {
        self.token = Some(token.into());
        self
    }

    /// 设置对话查询 AES 密钥（base64 编码）。
    pub fn set_aes_key(&mut self, aes_key: impl Into<String>) -> &mut Self {
        self.aes_key = Some(aes_key.into());
        self
    }

    /// 设置知识库签名密钥。
    pub fn set_secret_key(&mut self, secret_key: impl Into<String>) -> &mut Self {
        self.secret_key = Some(secret_key.into());
        self
    }

    /// 设置对话机器人 API 基地址。
    pub fn set_dialog_api_base_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.dialog_api_base_url = url.into();
        self
    }

    /// 设置知识库助理 API 基地址。
    pub fn set_knowledge_api_base_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.knowledge_api_base_url = url.into();
        self
    }

    /// 设置 HTTP 代理主机。
    pub fn set_http_proxy_host(&mut self, host: impl Into<String>) -> &mut Self {
        self.http_proxy_host = Some(host.into());
        self
    }

    /// 设置 HTTP 代理端口。
    pub fn set_http_proxy_port(&mut self, port: i32) -> &mut Self {
        self.http_proxy_port = port;
        self
    }

    /// 设置 HTTP 代理用户名。
    pub fn set_http_proxy_username(&mut self, username: impl Into<String>) -> &mut Self {
        self.http_proxy_username = Some(username.into());
        self
    }

    /// 设置 HTTP 代理密码。
    pub fn set_http_proxy_password(&mut self, password: impl Into<String>) -> &mut Self {
        self.http_proxy_password = Some(password.into());
        self
    }
}

impl Default for WxAispeechDefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl WxAispeechConfigStorage for WxAispeechDefaultConfig {
    fn appid(&self) -> Option<&str> {
        self.appid.as_deref()
    }

    fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    fn aes_key(&self) -> Option<&str> {
        self.aes_key.as_deref()
    }

    fn open_ai_token(&self) -> Option<String> {
        self.open_ai_token.lock().unwrap().clone()
    }

    fn set_open_ai_token(&self, open_ai_token: &str) {
        *self.open_ai_token.lock().unwrap() = Some(open_ai_token.to_string());
    }

    fn secret_key(&self) -> Option<&str> {
        self.secret_key.as_deref()
    }

    fn dialog_api_base_url(&self) -> String {
        self.dialog_api_base_url.clone()
    }

    fn knowledge_api_base_url(&self) -> String {
        self.knowledge_api_base_url.clone()
    }

    fn http_proxy_host(&self) -> Option<&str> {
        self.http_proxy_host.as_deref()
    }

    fn http_proxy_port(&self) -> i32 {
        self.http_proxy_port
    }

    fn http_proxy_username(&self) -> Option<&str> {
        self.http_proxy_username.as_deref()
    }

    fn http_proxy_password(&self) -> Option<&str> {
        self.http_proxy_password.as_deref()
    }
}
