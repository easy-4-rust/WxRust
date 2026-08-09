//! 视频号小店配置存储接口。
//!
//! 对应 Java `me.chanjar.weixin.channel.config.WxChannelConfig`，在
//! `wx-rust-common::config::WxConfigStorage`（token/过期/锁/代理）基础上
//! 扩展视频号小店专属配置项。Java 接口以 getter/setter 暴露；Rust trait
//! 以只读方法 + 可变方法表达同一契约。

use wx_rust_common::config::WxConfigStorage;

use crate::config::WxChannelHostConfig;

/// 微信 API 默认主机地址（对应 Java `WxChannelConfig` 默认
/// `https://api.weixin.qq.com`，即 `WxChannelApiUrlConstants` 各 URL 的域名前缀）。
pub const DEFAULT_API_HOST_URL: &str = "https://api.weixin.qq.com";
/// 默认 access_token 接口地址（`%s` 依次为 appid/secret，对应 Java
/// `WxChannelApiUrlConstants.GET_ACCESS_TOKEN_URL`）。
pub const DEFAULT_ACCESS_TOKEN_URL: &str =
    "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=%s&secret=%s";

/// 视频号小店配置存储。
///
/// app_id/secret/access_token 缓存/过期/锁语义继承自 common `WxConfigStorage`。
pub trait WxChannelConfig: WxConfigStorage + Send + Sync {
    /// 消息校验 token（对应 Java `getToken()`）。
    fn token(&self) -> Option<&str>;

    /// 消息加解密 aes key（对应 Java `getAesKey()`）。
    fn aes_key(&self) -> Option<&str>;

    /// 消息数据格式（对应 Java `getMsgDataFormat()`）。
    fn msg_data_format(&self) -> Option<&str>;

    /// access token 过期时刻（毫秒时间戳，对应 Java `getExpiresTime()`）。
    fn expires_time(&self) -> i64;

    /// HTTP 请求代理用户名（对应 Java `getHttpProxyUsername()`；返回 owned 值，
    /// 实现侧可能以锁保护字段）。
    fn http_proxy_username(&self) -> Option<String>;

    /// HTTP 请求代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<String>;

    /// HTTP 请求重试间隔（毫秒），对应 Java `getRetrySleepMillis()`。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// 设置 HTTP 请求重试间隔（毫秒；对应 Java 服务端 `setRetrySleepMillis(int)`
    /// 与配置实现侧 `setRetrySleepMillis(int)`）。
    fn set_retry_sleep_millis(&self, _retry_sleep_millis: i32) {}

    /// HTTP 请求最大重试次数，对应 Java `getMaxRetryTimes()`。
    fn max_retry_times(&self) -> i32 {
        5
    }

    /// 设置 HTTP 请求最大重试次数（对应 Java 服务端 `setMaxRetryTimes(int)`
    /// 与配置实现侧 `setMaxRetryTimes(int)`）。
    fn set_max_retry_times(&self, _max_retry_times: i32) {}

    /// 自定义接口域名配置（Rust 统一结构；Java 侧以 `getApiHostUrl()` 表达）。
    fn host_config(&self) -> WxChannelHostConfig;

    /// 设置自定义接口域名配置。
    fn set_host_config(&self, host_config: WxChannelHostConfig);

    /// 自定义 apiHost 地址（对应 Java `getApiHostUrl()`，用于替换请求中的
    /// `https://api.weixin.qq.com`；返回 owned 值：实现侧可能以锁保护字段）。
    fn api_host_url(&self) -> Option<String>;

    /// 设置自定义 apiHost 地址（对应 Java `setApiHostUrl(String)`）。
    fn set_api_host_url(&self, api_host_url: &str);

    /// 自定义获取 accessToken 地址（对应 Java `getAccessTokenUrl()`）。
    fn access_token_url(&self) -> Option<String>;

    /// 设置自定义获取 accessToken 地址（对应 Java `setAccessTokenUrl(String)`）。
    fn set_access_token_url(&self, access_token_url: &str);
}
