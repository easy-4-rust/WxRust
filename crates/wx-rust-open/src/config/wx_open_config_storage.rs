//! 开放平台（第三方平台）配置存储接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenConfigStorage`。
//!
//! 与 mp/ma 不同：Java 的 `WxOpenConfigStorage` 是独立接口（不继承
//! common `WxConfigStorage`），其核心是第三方平台的 component 凭据链
//! （appid/secret/token/aesKey/verify_ticket → component_access_token）
//! 与按 authorizer appId 分桶的 token/ticket 缓存。Rust trait 以只读
//! 方法 + 可变方法表达同一契约（`&self` 可变性由内部锁保证）。

use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::config::WxConfigStorage;

use crate::bean::{WxOpenAuthorizerAccessToken, WxOpenComponentAccessToken};
use crate::config::WxOpenHostConfig;

/// 开放平台（第三方平台）配置存储。
///
/// 全部方法对应 Java `WxOpenConfigStorage` 同名 getter/setter：
/// component 三凭证 + verify_ticket + component_access_token 缓存、
/// 按 appId 分桶的授权方 token/jsapi/card ticket、代理、重试与锁。
pub trait WxOpenConfigStorage: Send + Sync {
    // ---- 第三方平台 component 基础信息（对应 Java get/setComponentAppId 等） ----

    /// 第三方平台 appid（对应 Java `getComponentAppId()`）。
    fn component_app_id(&self) -> Option<String>;

    /// 设置第三方平台 appid（对应 Java `setComponentAppId(String)`）。
    fn set_component_app_id(&self, component_app_id: &str);

    /// 第三方平台 appsecret（对应 Java `getComponentAppSecret()`）。
    fn component_app_secret(&self) -> Option<String>;

    /// 设置第三方平台 appsecret（对应 Java `setComponentAppSecret(String)`）。
    fn set_component_app_secret(&self, component_app_secret: &str);

    /// 消息校验 Token（对应 Java `getComponentToken()`）。
    fn component_token(&self) -> Option<String>;

    /// 设置消息校验 Token（对应 Java `setComponentToken(String)`）。
    fn set_component_token(&self, component_token: &str);

    /// 消息加解密 Key（对应 Java `getComponentAesKey()`）。
    fn component_aes_key(&self) -> Option<String>;

    /// 设置消息加解密 Key（对应 Java `setComponentAesKey(String)`）。
    fn set_component_aes_key(&self, component_aes_key: &str);

    /// 推送的 verify ticket（对应 Java `getComponentVerifyTicket()`）。
    fn component_verify_ticket(&self) -> Option<String>;

    /// 设置推送的 verify ticket（对应 Java `setComponentVerifyTicket(String)`）。
    fn set_component_verify_ticket(&self, component_verify_ticket: &str);

    /// 设置第三方平台基础信息（对应 Java `setWxOpenInfo` 四参便捷方法）。
    fn set_wx_open_info(
        &self,
        component_app_id: &str,
        component_app_secret: &str,
        component_token: &str,
        component_aes_key: &str,
    ) {
        self.set_component_app_id(component_app_id);
        self.set_component_app_secret(component_app_secret);
        self.set_component_token(component_token);
        self.set_component_aes_key(component_aes_key);
    }

    // ---- component_access_token 缓存（对应 Java get/isExpired/expire/update） ----

    /// 当前缓存的 component_access_token（对应 Java `getComponentAccessToken()`）。
    fn component_access_token(&self) -> Option<String>;

    /// component_access_token 是否已过期（对应 Java `isComponentAccessTokenExpired()`）。
    fn is_component_access_token_expired(&self) -> bool;

    /// 强制将 component_access_token 过期（对应 Java `expireComponentAccessToken()`）。
    fn expire_component_access_token(&self);

    /// 线程安全地更新 component_access_token（对应 Java
    /// `updateComponentAccessToken(WxOpenComponentAccessToken)`）。
    fn update_component_access_token(&self, component_access_token: &WxOpenComponentAccessToken) {
        self.update_component_access_token_with_expiry(
            component_access_token.component_access_token(),
            component_access_token.expires_in(),
        );
    }

    /// 线程安全地更新 component_access_token（对应 Java
    /// `updateComponentAccessToken(String, int)` 重载）。
    fn update_component_access_token_with_expiry(
        &self,
        component_access_token: &str,
        expires_in_seconds: i32,
    );

    /// component_access_token 锁（对应 Java `getComponentAccessTokenLock()`）。
    fn component_access_token_lock(&self) -> Arc<AsyncMutex<()>>;

    /// 按 key 取锁（对应 Java `getLockByKey(String)`，并发仅刷新一次语义）。
    fn lock_by_key(&self, key: &str) -> Arc<AsyncMutex<()>>;

    /// token 过期时是否自动刷新（对应 Java `autoRefreshToken()`，默认 true）。
    fn auto_refresh_token(&self) -> bool {
        true
    }

    // ---- 授权方（authorizer）token/ticket 缓存（按 appId 分桶） ----

    /// 授权方 refresh_token（对应 Java `getAuthorizerRefreshToken(String)`）。
    fn authorizer_refresh_token(&self, app_id: &str) -> Option<String>;

    /// 设置授权方 refresh_token（对应 Java `setAuthorizerRefreshToken(String, String)`）。
    fn set_authorizer_refresh_token(&self, app_id: &str, authorizer_refresh_token: &str);

    /// 更新授权方 refresh_token（对应 Java `updateAuthorizerRefreshToken(String, String)`
    /// 重载方法，Java 语义与 set 相同）。
    fn update_authorizer_refresh_token(&self, app_id: &str, authorizer_refresh_token: &str) {
        self.set_authorizer_refresh_token(app_id, authorizer_refresh_token);
    }

    /// 授权方 access_token（对应 Java `getAuthorizerAccessToken(String)`）。
    fn authorizer_access_token(&self, app_id: &str) -> Option<String>;

    /// 授权方 access_token 是否过期（对应 Java `isAuthorizerAccessTokenExpired(String)`）。
    fn is_authorizer_access_token_expired(&self, app_id: &str) -> bool;

    /// 强制将授权方 access_token 过期（对应 Java `expireAuthorizerAccessToken(String)`）。
    fn expire_authorizer_access_token(&self, app_id: &str);

    /// 线程安全地更新授权方 access_token（对应 Java
    /// `updateAuthorizerAccessToken(String, WxOpenAuthorizerAccessToken)`）。
    fn update_authorizer_access_token(
        &self,
        app_id: &str,
        authorizer_access_token: &WxOpenAuthorizerAccessToken,
    ) {
        self.update_authorizer_access_token_with_expiry(
            app_id,
            authorizer_access_token.authorizer_access_token(),
            authorizer_access_token.expires_in(),
        );
    }

    /// 线程安全地更新授权方 access_token（对应 Java
    /// `updateAuthorizerAccessToken(String, String, int)` 重载）。
    fn update_authorizer_access_token_with_expiry(
        &self,
        app_id: &str,
        authorizer_access_token: &str,
        expires_in_seconds: i32,
    );

    /// 授权方 jsapi ticket（对应 Java `getJsapiTicket(String)`）。
    fn jsapi_ticket(&self, app_id: &str) -> Option<String>;

    /// 授权方 jsapi ticket 是否过期（对应 Java `isJsapiTicketExpired(String)`）。
    fn is_jsapi_ticket_expired(&self, app_id: &str) -> bool;

    /// 强制将授权方 jsapi ticket 过期（对应 Java `expireJsapiTicket(String)`）。
    fn expire_jsapi_ticket(&self, app_id: &str);

    /// 线程安全地更新授权方 jsapi ticket（对应 Java
    /// `updateJsapiTicket(String, String, int)`）。
    fn update_jsapi_ticket(&self, app_id: &str, jsapi_ticket: &str, expires_in_seconds: i32);

    /// 授权方卡券 api ticket（对应 Java `getCardApiTicket(String)`）。
    fn card_api_ticket(&self, app_id: &str) -> Option<String>;

    /// 授权方卡券 api ticket 是否过期（对应 Java `isCardApiTicketExpired(String)`）。
    fn is_card_api_ticket_expired(&self, app_id: &str) -> bool;

    /// 强制将授权方卡券 api ticket 过期（对应 Java `expireCardApiTicket(String)`）。
    fn expire_card_api_ticket(&self, app_id: &str);

    /// 线程安全地更新授权方卡券 api ticket（对应 Java
    /// `updateCardApiTicket(String, String, int)`）。
    fn update_card_api_ticket(&self, app_id: &str, card_api_ticket: &str, expires_in_seconds: i32);

    // ---- 授权方代 mp/ma 配置桥接 ----

    /// 按 appId 取代公众号配置存储（对应 Java `getWxMpConfigStorage(String)`）。
    ///
    /// Wave 0 占位：Java 返回 `WxMpConfigStorage`（`WxOpenInnerConfigStorage`
    /// 内层桥接实现），Rust 侧待代 mp 桥接波次接入 `wx-rust-mp` 后返回其
    /// `WxMpConfigStorage` trait 对象，当前统一以 common `WxConfigStorage`
    /// 表达并返回 `None`。
    fn wx_mp_config_storage(&self, _app_id: &str) -> Option<Arc<dyn WxConfigStorage>> {
        None
    }

    /// 按 appId 取代小程序配置存储（对应 Java `getWxMaConfig(String)`）。
    ///
    /// 占位说明同 [`Self::wx_mp_config_storage`]。
    fn wx_ma_config(&self, _app_id: &str) -> Option<Arc<dyn WxConfigStorage>> {
        None
    }

    /// 自定义域名配置（对应 mp/ma 的 host config 语义）。
    ///
    /// Java `WxOpenConfigStorage` 无此方法，以 `WxOpenInMemoryConfigStorage`
    /// 的 `apiHostUrl` 表达同一能力；Rust 侧统一为 host 配置结构，默认
    /// 返回 `None`（使用 `https://api.weixin.qq.com`），默认实现返回
    /// 含自定义 apiHostUrl 的配置。
    fn wx_open_host_config(&self) -> Option<WxOpenHostConfig> {
        None
    }

    // ---- HTTP 代理与重试 ----

    /// HTTP 代理主机（对应 Java `getHttpProxyHost()`）。
    fn http_proxy_host(&self) -> Option<String>;

    /// HTTP 代理端口（对应 Java `getHttpProxyPort()`）。
    fn http_proxy_port(&self) -> i32;

    /// HTTP 代理用户名（对应 Java `getHttpProxyUsername()`）。
    fn http_proxy_username(&self) -> Option<String>;

    /// HTTP 代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<String>;

    /// HTTP 请求重试间隔（毫秒，对应 Java `getRetrySleepMillis()`，默认 1000）。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// HTTP 请求最大重试次数（对应 Java `getMaxRetryTimes()`，默认 5）。
    fn max_retry_times(&self) -> i32 {
        5
    }

    // ---- 服务端 API 签名（RSA/AES 私钥与序号） ----

    /// 第三方平台设置 API 签名 RSA 私钥（对应 Java
    /// `getComponentApiSignatureRsaPrivateKey()`）。
    fn component_api_signature_rsa_private_key(&self) -> Option<String>;

    /// 设置 API 签名 RSA 私钥（对应 Java
    /// `setComponentApiSignatureRsaPrivateKey(String)`）。
    fn set_component_api_signature_rsa_private_key(&self, api_signature_rsa_private_key: &str);

    /// API 签名 AES KEY（对应 Java `getComponentApiSignatureAesKey()`）。
    fn component_api_signature_aes_key(&self) -> Option<String>;

    /// 设置 API 签名 AES KEY（对应 Java `setComponentApiSignatureAesKey(String)`）。
    fn set_component_api_signature_aes_key(&self, api_signature_aes_key: &str);

    /// API 签名 RSA 私钥序号（对应 Java `getComponentApiSignatureRsaPrivateKeySn()`）。
    fn component_api_signature_rsa_private_key_sn(&self) -> Option<String>;

    /// 设置 API 签名 RSA 私钥序号（对应 Java
    /// `setComponentApiSignatureRsaPrivateKeySn(String)`）。
    fn set_component_api_signature_rsa_private_key_sn(
        &self,
        api_signature_rsa_private_key_sn: &str,
    );

    /// API 签名 AES key 序号（对应 Java `getComponentApiSignatureAesKeySn()`）。
    fn component_api_signature_aes_key_sn(&self) -> Option<String>;

    /// 设置 API 签名 AES key 序号（对应 Java
    /// `setComponentApiSignatureAesKeySn(String)`）。
    fn set_component_api_signature_aes_key_sn(&self, api_signature_aes_key_sn: &str);
}
