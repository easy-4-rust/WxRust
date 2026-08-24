//! 微信支付 HTTP 正向代理配置。
//!
//! 对应 Java `com.github.binarywang.wxpay.config.WxPayHttpProxy`
//! （`Serializable` 四字段 DTO：host/port/username/password）。
//!
//! Java 侧由 `HttpProxyUtils.initHttpProxy` 注入 Apache HttpClient 的
//! proxy/credentials；Rust 侧由宿主据此配置 reqwest Client 代理
//! （`reqwest::Proxy::all` + `.basic_auth`），故本类型只承载配置数据。

/// HTTP 正向代理配置（对应 Java `WxPayHttpProxy`）。
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WxPayHttpProxy {
    /// 代理主机（对应 Java 字段 `httpProxyHost`）。
    #[serde(
        rename = "httpProxyHost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub http_proxy_host: Option<String>,
    /// 代理端口（对应 Java 字段 `httpProxyPort`）。
    #[serde(
        rename = "httpProxyPort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub http_proxy_port: Option<i32>,
    /// 代理用户名（对应 Java 字段 `httpProxyUsername`）。
    #[serde(
        rename = "httpProxyUsername",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub http_proxy_username: Option<String>,
    /// 代理密码（对应 Java 字段 `httpProxyPassword`）。
    #[serde(
        rename = "httpProxyPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub http_proxy_password: Option<String>,
}

impl WxPayHttpProxy {
    /// 全参构造（对应 Java
    /// `WxPayHttpProxy(String, Integer, String, String)`）。
    pub fn new(
        http_proxy_host: Option<String>,
        http_proxy_port: Option<i32>,
        http_proxy_username: Option<String>,
        http_proxy_password: Option<String>,
    ) -> Self {
        Self {
            http_proxy_host,
            http_proxy_port,
            http_proxy_username,
            http_proxy_password,
        }
    }

    /// 代理主机（对应 Java `getHttpProxyHost()`）。
    pub fn http_proxy_host(&self) -> Option<&str> {
        self.http_proxy_host.as_deref()
    }

    /// 代理端口（对应 Java `getHttpProxyPort()`）。
    pub fn http_proxy_port(&self) -> Option<i32> {
        self.http_proxy_port
    }

    /// 代理用户名（对应 Java `getHttpProxyUsername()`）。
    pub fn http_proxy_username(&self) -> Option<&str> {
        self.http_proxy_username.as_deref()
    }

    /// 代理密码（对应 Java `getHttpProxyPassword()`）。
    pub fn http_proxy_password(&self) -> Option<&str> {
        self.http_proxy_password.as_deref()
    }

    /// 是否为可用代理配置（host 非空且端口 > 0；对应 Java
    /// `HttpProxyUtils.initHttpProxy` 的 `isNotBlank(host) && port > 0`
    /// 判断）。
    pub fn is_effective(&self) -> bool {
        self.http_proxy_host
            .as_deref()
            .is_some_and(|h| !h.trim().is_empty())
            && self.http_proxy_port.is_some_and(|p| p > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_accessors_and_effective_check() {
        let proxy = WxPayHttpProxy::new(
            Some("127.0.0.1".into()),
            Some(8888),
            Some("user".into()),
            Some("pass".into()),
        );
        assert_eq!(proxy.http_proxy_host(), Some("127.0.0.1"));
        assert_eq!(proxy.http_proxy_port(), Some(8888));
        assert_eq!(proxy.http_proxy_username(), Some("user"));
        assert_eq!(proxy.http_proxy_password(), Some("pass"));
        assert!(proxy.is_effective());

        // 对应 Java：host 空 / 端口非正 → 不配置代理
        assert!(!WxPayHttpProxy::default().is_effective());
        assert!(!WxPayHttpProxy::new(Some("h".into()), Some(0), None, None).is_effective());
    }
}
