//! v3 请求凭据接口。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.Credentials`：
//!
//! ```java
//! public interface Credentials {
//!   String getSchema();
//!   String getToken(HttpRequestWrapper request) throws IOException;
//! }
//! ```
//!
//! ADAPTED：Java 以 Apache HttpClient 的 `HttpRequestWrapper` 为参数；
//! Rust 以 [`CredentialsRequest`] 值对象承载同一信息（请求行方法、URI
//! path/query、请求体），trait 对象可在异步上下文自由传递。默认实现见
//! [`crate::v3::auth::WxPayCredentials`]（`WECHATPAY2-SHA256-RSA2048`）。

use crate::util::crypto::WxV3CryptoError;

/// 待签名的 v3 请求描述（对应 Java `HttpRequestWrapper` 携带的信息）。
#[derive(Debug, Clone)]
pub struct CredentialsRequest {
    /// HTTP 方法（大写，对应 `request.getRequestLine().getMethod()`）。
    pub method: String,
    /// URI rawPath（对应 `uri.getRawPath()`，未经解码，保留百分号编码）。
    pub path: String,
    /// URI rawQuery（对应 `uri.getRawQuery()`；无查询串时为 `None`）。
    pub query: Option<String>,
    /// 请求体原文（对应 `EntityUtils.toString(entity)`；GET 为空串）。
    pub body: String,
}

impl CredentialsRequest {
    /// 构造请求描述。
    pub fn new(
        method: impl Into<String>,
        path: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            query: None,
            body: body.into(),
        }
    }

    /// 附加查询串（对应 `uri.getQuery() != null` 分支）。
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
}

/// v3 请求凭据（对应 Java `v3/Credentials` 接口）。
///
/// 职责：为每个出站 v3 请求生成 Authorization 头的 schema 与 token。
pub trait Credentials: Send + Sync {
    /// Authorization 头 schema（对应 Java `getSchema()`，如
    /// `WECHATPAY2-SHA256-RSA2048`）。
    fn get_schema(&self) -> &'static str;

    /// 生成 Authorization token（对应 Java `getToken(request)`）：
    /// `mchid="..",nonce_str="..",timestamp="..",serial_no="..",signature=".."`。
    fn get_token(&self, request: &CredentialsRequest) -> Result<String, WxV3CryptoError>;
}
