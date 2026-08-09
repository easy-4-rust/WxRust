//! 公众号 API 域名配置。
//!
//! 对应 Java `me.chanjar.weixin.mp.config.WxMpHostConfig`。

/// 微信 API 默认域名。
pub const API_DEFAULT_HOST_URL: &str = "https://api.weixin.qq.com";
/// 微信 MP 默认域名。
pub const MP_DEFAULT_HOST_URL: &str = "https://mp.weixin.qq.com";
/// 微信开放平台默认域名。
pub const OPEN_DEFAULT_HOST_URL: &str = "https://open.weixin.qq.com";

/// 微信接口地址域名自定义设置。
///
/// 对应 Java `WxMpHostConfig`：可分别覆盖 api/mp/open 三类域名。
#[derive(Debug, Clone, Default)]
pub struct WxMpHostConfig {
    /// api 域名（默认 `https://api.weixin.qq.com`）
    pub api_host: String,
    /// mp 域名（默认 `https://mp.weixin.qq.com`）
    pub mp_host: String,
    /// open 域名（默认 `https://open.weixin.qq.com`）
    pub open_host: String,
}

impl WxMpHostConfig {
    /// 使用默认域名构建配置。
    pub fn new() -> Self {
        Self {
            api_host: API_DEFAULT_HOST_URL.to_string(),
            mp_host: MP_DEFAULT_HOST_URL.to_string(),
            open_host: OPEN_DEFAULT_HOST_URL.to_string(),
        }
    }
}
