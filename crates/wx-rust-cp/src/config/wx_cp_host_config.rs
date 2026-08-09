//! 企业微信 API 域名配置。
//!
//! 对应 Java `WxCpConfigStorage.getApiUrl(String)` 的 baseUrl 语义的
//! Rust 结构化表达（miniapp/mp 统一 host config 模式）：
//! Java 以单个 `baseApiUrl`（默认 `https://qyapi.weixin.qq.com`）覆盖全部
//! 接口域名；Rust 侧同样保留 `base_api_url` 单域名覆盖（私有化部署语义），
//! 并额外提供 api/mp/open 三类域名分别覆盖的能力（ADAPTED）。

use crate::enums::url_core::DEFAULT_CP_BASE_URL;

/// 企业微信开放平台默认域名。
pub const OPEN_DEFAULT_HOST_URL: &str = "https://open.weixin.qq.com";
/// 微信公众平台默认域名（企微二维码登录等页面使用）。
pub const MP_DEFAULT_HOST_URL: &str = "https://mp.weixin.qq.com";

/// 企业微信接口地址域名自定义设置。
///
/// 对应 mp/miniapp 的 `Wx*HostConfig`：可分别覆盖 api/mp/open 三类域名；
/// `api_host` 默认 `https://qyapi.weixin.qq.com`。
#[derive(Debug, Clone, Default)]
pub struct WxCpHostConfig {
    /// api 域名（默认 `https://qyapi.weixin.qq.com`）
    pub api_host: String,
    /// mp 域名（默认 `https://mp.weixin.qq.com`）
    pub mp_host: String,
    /// open 域名（默认 `https://open.weixin.qq.com`）
    pub open_host: String,
}

impl WxCpHostConfig {
    /// 使用默认域名构建配置。
    pub fn new() -> Self {
        Self {
            api_host: DEFAULT_CP_BASE_URL.to_string(),
            mp_host: MP_DEFAULT_HOST_URL.to_string(),
            open_host: OPEN_DEFAULT_HOST_URL.to_string(),
        }
    }
}
