//! 视频号小店 API 域名配置。
//!
//! 对应 mp 的 `WxMpHostConfig` 移植（Java 视频号小店侧以
//! `WxChannelConfig.getApiHostUrl()` 表达自定义域名，Rust 统一为 host config
//! 结构）。视频号小店接口域名为 `https://api.weixin.qq.com`，与 mp/open 域名
//! 一并保留以便自定义覆盖。

/// 微信 API 默认域名。
pub const API_DEFAULT_HOST_URL: &str = "https://api.weixin.qq.com";
/// 微信 MP 默认域名。
pub const MP_DEFAULT_HOST_URL: &str = "https://mp.weixin.qq.com";
/// 微信开放平台默认域名。
pub const OPEN_DEFAULT_HOST_URL: &str = "https://open.weixin.qq.com";

/// 微信接口地址域名自定义设置。
///
/// 可分别覆盖 api/mp/open 三类域名；视频号小店实际仅使用 api 域名
/// （`getApiHostUrl()` 语义），其余为兼容保留。
#[derive(Debug, Clone, Default)]
pub struct WxChannelHostConfig {
    /// api 域名（默认 `https://api.weixin.qq.com`）
    pub api_host: String,
    /// mp 域名（默认 `https://mp.weixin.qq.com`）
    pub mp_host: String,
    /// open 域名（默认 `https://open.weixin.qq.com`）
    pub open_host: String,
}

impl WxChannelHostConfig {
    /// 使用默认域名构建配置。
    pub fn new() -> Self {
        Self {
            api_host: API_DEFAULT_HOST_URL.to_string(),
            mp_host: MP_DEFAULT_HOST_URL.to_string(),
            open_host: OPEN_DEFAULT_HOST_URL.to_string(),
        }
    }
}
