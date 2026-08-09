//! 企业微信配置存储接口。
//!
//! 对应 Java `me.chanjar.weixin.cp.config.WxCpConfigStorage`，在
//! `wx-rust-common::config::WxConfigStorage`（token/ticket/锁/代理）基础上
//! 扩展企业微信专属配置项。
//!
//! 说明：
//! - Java 的 `getCorpId()`/`getCorpSecret()` 映射为 common
//!   `WxConfigStorage::app_id()`/`secret()`（企业微信中 corpId 即应用
//!   体系中的 appId，与 mp/miniapp 同一契约）；
//! - Java 接口的 `getAccessToken()`/`getAccessTokenLock()`/
//!   `isAccessTokenExpired()`/`expireAccessToken()`/`updateAccessToken(...)`
//!   由 common `WxConfigStorage` 的 access_token 语义覆盖；
//! - Java 接口的 `getJsapiTicket()`/`getJsapiTicketLock()`/
//!   `isJsapiTicketExpired()`/`expireJsapiTicket()`/`updateJsapiTicket(...)`
//!   由 common `WxConfigStorage` 的 `TicketType::Jsapi` ticket 语义覆盖；
//! - Java 的 `autoRefreshToken()`（恒 true）由 common `auto_refresh_token()`
//!   覆盖；`getHttpProxyHost()`/`getHttpProxyPort()` 由 common
//!   `http_proxy_host()`/`http_proxy_port()` 覆盖；
//! - 会话存档 SDK 相关方法在 Java 侧已 `@Deprecated`（生命周期改由
//!   `WxCpMsgAuditServiceImpl` 内部 ThreadLocal 模式管理），此处保留接口
//!   形态并默认空实现，语义与 Java 现状一致。

use std::sync::Arc;

use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::WxConfigStorage;

use crate::config::WxCpHostConfig;

/// 企业微信配置存储。
///
/// Java 接口以 getter/setter 形式暴露；Rust trait 以只读方法 + 可变方法
/// 表达同一契约。token/ticket 语义继承自 common 的 `WxConfigStorage`。
pub trait WxCpConfigStorage: WxConfigStorage + Send + Sync {
    // ---- baseUrl / host config（对应 Java `setBaseApiUrl`/`getApiUrl`） ----

    /// 设置企业微信服务器 baseUrl（对应 Java `setBaseApiUrl(String)`）。
    ///
    /// 默认值为 `https://qyapi.weixin.qq.com`；使用默认值时无需调用。
    fn set_base_api_url(&self, base_url: &str);

    /// 读取自定义企业微信 API Url（对应 Java `getBaseApiUrl`）。
    ///
    /// 返回 `None` 表示未设置（走默认域名）。
    fn base_api_url(&self) -> Option<String>;

    /// 读取企业微信 API Url（对应 Java `getApiUrl(String)`）。
    ///
    /// 支持私有化企业微信服务器：未设置 baseUrl 时按
    /// `https://qyapi.weixin.qq.com`（`WxCpApiPathConsts.DEFAULT_CP_BASE_URL`）
    /// + path 拼接（Java 语义：`baseApiUrl + path`）。
    fn api_url(&self, path: &str) -> String {
        if let Some(base) = self.base_api_url() {
            if !base.is_empty() {
                return format!("{base}{path}");
            }
        }
        let h = self.host_config();
        if !h.api_host.is_empty() {
            return format!("{}{path}", h.api_host);
        }
        format!("{}{path}", crate::enums::url_core::DEFAULT_CP_BASE_URL)
    }

    /// 自定义接口域名配置（Rust 扩展：miniapp/mp 统一模式）。
    fn host_config(&self) -> WxCpHostConfig;

    /// 设置自定义接口域名配置。
    fn set_host_config(&self, host_config: WxCpHostConfig);

    // ---- 应用配置（对应 Java getCorpId/getCorpSecret/getAgentId 等） ----
    // corp_id/corp_secret 由 common `WxConfigStorage::app_id()/secret()` 覆盖。

    /// 应用 agentid（对应 Java `getAgentId()`）。
    fn agent_id(&self) -> Option<i32>;

    /// 设置应用 agentid（对应 Java `setAgentId(Integer)`）。
    fn set_agent_id(&self, agent_id: Option<i32>);

    /// 消息校验 token（对应 Java `getToken()`）。
    ///
    /// 返回 owned 值（Rust 适配：实现侧可能以锁保护字段，无法返回借用）。
    fn token(&self) -> Option<String>;

    /// 消息加解密 aes key（对应 Java `getAesKey()`）。
    fn aes_key(&self) -> Option<String>;

    /// access token 过期时刻（毫秒，对应 Java `getExpiresTime()`）。
    fn expires_time(&self) -> i64;

    /// 设置 access token 过期时刻（毫秒，对应 Java `setExpiresTime(long)`）。
    fn set_expires_time(&self, expires_time: i64);

    /// OAuth2 回调地址（对应 Java `getOauth2redirectUri()`）。
    fn oauth2_redirect_uri(&self) -> Option<String>;

    /// 设置 OAuth2 回调地址（对应 Java `setOauth2redirectUri(String)`）。
    fn set_oauth2_redirect_uri(&self, oauth2_redirect_uri: &str);

    /// HTTP 代理用户名（对应 Java `getHttpProxyUsername()`）。
    fn http_proxy_username(&self) -> Option<String>;

    /// 设置 HTTP 代理用户名（对应 Java `setHttpProxyUsername(String)`）。
    fn set_http_proxy_username(&self, username: &str);

    /// HTTP 代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<String>;

    /// 设置 HTTP 代理密码（对应 Java `setHttpProxyPassword(String)`）。
    fn set_http_proxy_password(&self, password: &str);

    /// 临时文件目录（对应 Java `getTmpDirFile()`，Java 返回 `File`；
    /// Rust 以路径字符串表达，`None` 表示未设置）。
    fn tmp_dir_file(&self) -> Option<String>;

    /// 设置临时文件目录（对应 Java `setTmpDirFile(File)`）。
    fn set_tmp_dir_file(&self, tmp_dir_file: &str);

    /// 群机器人 webhook 的 key（对应 Java `getWebhookKey()`）。
    fn webhook_key(&self) -> Option<String>;

    /// 设置群机器人 webhook 的 key（对应 Java `setWebhookKey(String)`）。
    fn set_webhook_key(&self, webhook_key: &str);

    /// HTTP 请求重试间隔（毫秒），对应 Java `getRetrySleepMillis()`。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// HTTP 请求最大重试次数，对应 Java `getMaxRetryTimes()`。
    fn max_retry_times(&self) -> i32 {
        5
    }

    // ---- 通讯录同步 access token（对应 Java getContactSecret 等） ----

    /// 通讯录同步的 secret（对应 Java `getContactSecret()`）。
    fn contact_secret(&self) -> Option<String>;

    /// 设置通讯录同步的 secret（对应 Java `setContactSecret(String)`）。
    fn set_contact_secret(&self, contact_secret: &str);

    /// 通讯录同步的 access token（对应 Java `getContactAccessToken()`）。
    fn contact_access_token(&self) -> Option<String>;

    /// 通讯录同步 access token 的锁（对应 Java `getContactAccessTokenLock()`）。
    fn contact_access_token_lock(&self) -> Arc<AsyncMutex<()>>;

    /// 检查通讯录同步 access token 是否已过期（对应 Java `isContactAccessTokenExpired()`）。
    fn is_contact_access_token_expired(&self) -> bool;

    /// 强制将通讯录同步 access token 过期掉（对应 Java `expireContactAccessToken()`）。
    fn expire_contact_access_token(&self);

    /// 更新通讯录同步 access token（对应 Java
    /// `updateContactAccessToken(String, int)`）。
    fn update_contact_access_token(&self, access_token: &str, expires_in_seconds: i32);

    // ---- 会话存档（对应 Java getMsgAuditSecret/getMsgAuditPriKey 等） ----

    /// 会话存档的 secret（对应 Java `getMsgAuditSecret()`）。
    fn msg_audit_secret(&self) -> Option<String>;

    /// 设置会话存档的 secret（对应 Java `setMsgAuditSecret(String)`）。
    fn set_msg_audit_secret(&self, msg_audit_secret: &str);

    /// 企微会话存档私钥（对应 Java `getMsgAuditPriKey()`）。
    fn msg_audit_pri_key(&self) -> Option<String>;

    /// 设置企微会话存档私钥（对应 Java `setMsgAuditPriKey(String)`）。
    fn set_msg_audit_pri_key(&self, msg_audit_pri_key: &str);

    /// 企微会话存档系统库绝对路径（对应 Java `getMsgAuditLibPath()`）。
    fn msg_audit_lib_path(&self) -> Option<String>;

    /// 设置企微会话存档系统库绝对路径（对应 Java `setMsgAuditLibPath(String)`）。
    fn set_msg_audit_lib_path(&self, msg_audit_lib_path: &str);

    /// 会话存档的 access token（对应 Java `getMsgAuditAccessToken()`）。
    fn msg_audit_access_token(&self) -> Option<String>;

    /// 会话存档 access token 的锁（对应 Java `getMsgAuditAccessTokenLock()`）。
    fn msg_audit_access_token_lock(&self) -> Arc<AsyncMutex<()>>;

    /// 检查会话存档 access token 是否已过期（对应 Java `isMsgAuditAccessTokenExpired()`）。
    fn is_msg_audit_access_token_expired(&self) -> bool;

    /// 强制将会话存档 access token 过期掉（对应 Java `expireMsgAuditAccessToken()`）。
    fn expire_msg_audit_access_token(&self);

    /// 更新会话存档 access token（对应 Java
    /// `updateMsgAuditAccessToken(String, int)`）。
    fn update_msg_audit_access_token(&self, access_token: &str, expires_in_seconds: i32);

    // ---- 应用 jsapi ticket（对应 Java getAgentJsapiTicket 等） ----

    /// 应用的 jsapi ticket（对应 Java `getAgentJsapiTicket()`）。
    fn agent_jsapi_ticket(&self) -> Option<String>;

    /// 应用 jsapi ticket 的锁（对应 Java `getAgentJsapiTicketLock()`）。
    fn agent_jsapi_ticket_lock(&self) -> Arc<AsyncMutex<()>>;

    /// 检查应用 jsapi ticket 是否已过期（对应 Java `isAgentJsapiTicketExpired()`）。
    fn is_agent_jsapi_ticket_expired(&self) -> bool;

    /// 强制将应用 jsapi ticket 过期掉（对应 Java `expireAgentJsapiTicket()`）。
    fn expire_agent_jsapi_ticket(&self);

    /// 更新应用 jsapi ticket（对应 Java `updateAgentJsapiTicket(String, int)`）。
    fn update_agent_jsapi_ticket(&self, jsapi_ticket: &str, expires_in_seconds: i32);

    // ---- 会话存档 SDK（历史接口，Java 侧 @Deprecated） ----

    /// 获取会话存档 SDK（历史接口，对应 Java `getMsgAuditSdk()`）。
    ///
    /// 生命周期已改由 `WxCpMsgAuditService` 内部 ThreadLocal 模式管理；
    /// 此处仅为兼容旧代码保留。默认返回 0（未初始化）。
    fn msg_audit_sdk(&self) -> i64 {
        0
    }

    /// 检查会话存档 SDK 是否已过期（历史接口，对应 Java `isMsgAuditSdkExpired()`）。
    fn is_msg_audit_sdk_expired(&self) -> bool {
        true
    }

    /// 更新会话存档 SDK（历史接口，对应 Java `updateMsgAuditSdk(long, int)`）。
    fn update_msg_audit_sdk(&self, _sdk: i64, _expires_in_seconds: i32) {}

    /// 使会话存档 SDK 过期（历史接口，对应 Java `expireMsgAuditSdk()`）。
    fn expire_msg_audit_sdk(&self) {}

    /// 增加会话存档 SDK 的引用计数（历史接口，对应 Java
    /// `incrementMsgAuditSdkRefCount(long)`）。SDK 不匹配时返回 -1。
    fn increment_msg_audit_sdk_ref_count(&self, _sdk: i64) -> i32 {
        -1
    }

    /// 减少会话存档 SDK 的引用计数（历史接口，对应 Java
    /// `decrementMsgAuditSdkRefCount(long)`）。SDK 不匹配或引用计数已为 0
    /// 时返回 -1。
    fn decrement_msg_audit_sdk_ref_count(&self, _sdk: i64) -> i32 {
        -1
    }

    /// 获取会话存档 SDK 的引用计数（历史接口，对应 Java
    /// `getMsgAuditSdkRefCount(long)`）。SDK 不匹配时返回 -1。
    fn get_msg_audit_sdk_ref_count(&self, _sdk: i64) -> i32 {
        -1
    }

    /// 获取当前 SDK 并增加引用计数（历史接口，原子操作，对应 Java
    /// `acquireMsgAuditSdk()`）。SDK 未初始化或已过期时返回 0。
    fn acquire_msg_audit_sdk(&self) -> i64 {
        0
    }

    /// 减少 SDK 引用计数并在必要时释放（历史接口，原子操作，对应 Java
    /// `releaseMsgAuditSdk(long)`）。
    fn release_msg_audit_sdk(&self, _sdk: i64) {}
}
