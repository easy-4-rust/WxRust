//! 对应 Java `me.chanjar.weixin.qidian.bean.WxQidianHostConfig.java`。

/// 微信 API 默认主机地址（对应 Java `API_DEFAULT_HOST_URL`）。
pub const API_DEFAULT_HOST_URL: &str = "https://api.weixin.qq.com";
/// 微信开放平台默认主机地址（对应 Java `OPEN_DEFAULT_HOST_URL`）。
pub const OPEN_DEFAULT_HOST_URL: &str = "https://open.weixin.qq.com";
/// 腾讯企点 API 默认主机地址（对应 Java `QIDIAN_DEFAULT_HOST_URL`）。
pub const QIDIAN_DEFAULT_HOST_URL: &str = "https://api.qidian.qq.com";

/// 企点接口地址域名部分的自定义设置信息。
///
/// 对应 Java `WxQidianHostConfig`：三个主机地址分别对应
/// `https://api.weixin.qq.com` / `https://open.weixin.qq.com` /
/// `https://api.qidian.qq.com` 前缀的替换。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WxQidianHostConfig {
    /// 对应于 `https://api.weixin.qq.com`
    pub api_host: Option<String>,
    /// 对应于 `https://open.weixin.qq.com`
    pub open_host: Option<String>,
    /// 对应于 `https://api.qidian.qq.com`
    pub qidian_host: Option<String>,
}

impl WxQidianHostConfig {
    /// 构建空配置（全部使用默认主机）。
    pub fn new() -> Self {
        Self::default()
    }

    /// 根据主机配置构建完整接口地址（对应 Java 静态方法
    /// `buildUrl(WxQidianHostConfig, prefix, path)`）。
    ///
    /// 仅当 `host_config` 非空且对应前缀已被自定义时替换主机部分，
    /// 否则原样返回 `prefix + path`。
    pub fn build_url(host_config: Option<&WxQidianHostConfig>, prefix: &str, path: &str) -> String {
        match host_config {
            None => format!("{prefix}{path}"),
            Some(config) => {
                if let Some(host) = &config.api_host {
                    if prefix == API_DEFAULT_HOST_URL {
                        return format!("{host}{path}");
                    }
                }
                if let Some(host) = &config.qidian_host {
                    if prefix == QIDIAN_DEFAULT_HOST_URL {
                        return format!("{host}{path}");
                    }
                }
                if let Some(host) = &config.open_host {
                    if prefix == OPEN_DEFAULT_HOST_URL {
                        return format!("{host}{path}");
                    }
                }
                format!("{prefix}{path}")
            }
        }
    }
}
