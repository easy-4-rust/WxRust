//! 腾讯企点配置存储接口。
//!
//! 对应 Java `me.chanjar.weixin.qidian.config.WxQidianConfigStorage`。
//! Java 接口显式声明 access token/ticket 缓存、锁与过期语义（与 common
//! `WxService` 配套）；Rust 直接继承 `wx_rust_common::config::WxConfigStorage`
//! 承载同一契约（token/ticket/锁），此处仅扩展企点专属配置项。

use crate::bean::WxQidianHostConfig;
use wx_rust_common::config::WxConfigStorage;

/// 腾讯企点配置存储。
pub trait WxQidianConfigStorage: WxConfigStorage + Send + Sync {
    /// 消息校验 token（对应 Java `getToken()`）。
    fn token(&self) -> Option<&str>;

    /// 消息加解密 aes key（对应 Java `getAesKey()`）。
    fn aes_key(&self) -> Option<&str>;

    /// 模板 id（对应 Java `getTemplateId()`）。
    fn template_id(&self) -> Option<&str>;

    /// access token 过期时刻（UNIX 毫秒，对应 Java `getExpiresTime()`；
    /// 与 common `TokenEntry.expires_at` 一致，未设置时为 0）。
    fn expires_time(&self) -> i64;

    /// OAuth2 授权回调地址（对应 Java `getOauth2redirectUri()`）。
    fn oauth2_redirect_uri(&self) -> Option<&str>;

    /// HTTP 代理用户名（对应 Java `getHttpProxyUsername()`）。
    fn http_proxy_username(&self) -> Option<&str>;

    /// HTTP 代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<&str>;

    /// 自定义接口域名配置（对应 Java `getHostConfig()`）。
    fn host_config(&self) -> WxQidianHostConfig;

    /// 设置自定义接口域名配置（对应 Java `setHostConfig`）。
    fn set_host_config(&self, host_config: WxQidianHostConfig);
}
