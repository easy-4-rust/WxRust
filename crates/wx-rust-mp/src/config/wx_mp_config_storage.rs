//! 公众号配置存储接口。
//!
//! 对应 Java `me.chanjar.weixin.mp.config.WxMpConfigStorage`，在
//! `wx-rust-common::config::WxConfigStorage`（token/ticket/锁/代理）基础上
//! 扩展公众号专属配置项。

use wx_rust_common::config::WxConfigStorage;

use crate::config::WxMpHostConfig;

/// 公众号配置存储。
///
/// Java 接口以 getter/setter 形式暴露；Rust trait 以只读方法 + 可变方法
/// 表达同一契约。token/ticket 语义继承自 common 的 `WxConfigStorage`。
pub trait WxMpConfigStorage: WxConfigStorage + Send + Sync {
    /// 设置是否使用稳定版 access token 接口（查询见 common `WxConfigStorage`）。
    fn use_stable_access_token(&self, use_stable_access_token: bool);

    /// 消息校验 token。
    fn token(&self) -> Option<&str>;

    /// 消息加解密 aes key。
    fn aes_key(&self) -> Option<&str>;

    /// 模板消息模板 id。
    fn template_id(&self) -> Option<&str>;

    /// OAuth2 回调地址。
    fn oauth2_redirect_url(&self) -> Option<&str>;

    /// 扫码连接回调地址。
    fn qr_connect_redirect_url(&self) -> Option<&str>;

    /// HTTP 请求重试间隔（毫秒）。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// HTTP 请求最大重试次数。
    fn max_retry_times(&self) -> i32 {
        5
    }

    /// 自定义接口域名配置。
    fn host_config(&self) -> WxMpHostConfig;

    /// 设置自定义接口域名配置。
    fn set_host_config(&self, host_config: WxMpHostConfig);
}
