//! HTTP 客户端类型枚举。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.HttpClientType`。
//! Java 中用于在 apache/okhttp/jodd/httpcomponents 四后端间选择；
//! WxRust 统一使用 reqwest，此枚举保留以兼容配置语义（集成层可据此做兼容映射）。

/// HTTP 客户端类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpClientType {
    /// jodd-http（Java 专属后端，Rust 不实现）
    JoddHttp,
    /// apache httpclient 4.x（Java 专属后端，Rust 不实现）
    ApacheHttp,
    /// okhttp（Java 专属后端，Rust 不实现）
    OkHttp,
    /// apache httpclient 5.x（Java 专属后端，Rust 不实现）
    HttpComponents,
}

impl HttpClientType {
    /// 返回枚举名（与 Java `name()` 对齐）。
    pub fn name(self) -> &'static str {
        match self {
            HttpClientType::JoddHttp => "JODD_HTTP",
            HttpClientType::ApacheHttp => "APACHE_HTTP",
            HttpClientType::OkHttp => "OK_HTTP",
            HttpClientType::HttpComponents => "HTTP_COMPONENTS",
        }
    }
}
