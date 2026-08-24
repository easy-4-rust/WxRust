//! v3 响应验签接口。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.Validator`：
//!
//! ```java
//! public interface Validator {
//!   boolean validate(CloseableHttpResponse response) throws IOException;
//! }
//! ```
//!
//! ADAPTED：Java 以 Apache HttpClient 的 `CloseableHttpResponse` 为参数；
//! Rust 以 [`ValidationResponse`] 值对象承载验签所需的五个头与响应体。
//! 默认实现见 [`crate::v3::auth::WxPayValidator`]。

/// 待验签的 v3 响应描述（对应 Java `CloseableHttpResponse` 的验签输入）。
#[derive(Debug, Clone, Default)]
pub struct ValidationResponse {
    /// `Content-Type` 头（对应 `response.getFirstHeader("Content-Type")`）。
    pub content_type: Option<String>,
    /// `Wechatpay-Serial` 头（平台证书/公钥序列号）。
    pub wechatpay_serial: Option<String>,
    /// `Wechatpay-Signature` 头（Base64 签名）。
    pub wechatpay_signature: Option<String>,
    /// `Wechatpay-TimeStamp` 头。
    pub wechatpay_timestamp: Option<String>,
    /// `Wechatpay-Nonce` 头。
    pub wechatpay_nonce: Option<String>,
    /// 响应体原文（对应 `EntityUtils.toString(entity)`）。
    pub body: String,
}

impl ValidationResponse {
    /// 以五个验签头 + 响应体构造。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        content_type: Option<&str>,
        wechatpay_serial: &str,
        wechatpay_signature: &str,
        wechatpay_timestamp: &str,
        wechatpay_nonce: &str,
        body: &str,
    ) -> Self {
        Self {
            content_type: content_type.map(str::to_string),
            wechatpay_serial: Some(wechatpay_serial.to_string()),
            wechatpay_signature: Some(wechatpay_signature.to_string()),
            wechatpay_timestamp: Some(wechatpay_timestamp.to_string()),
            wechatpay_nonce: Some(wechatpay_nonce.to_string()),
            body: body.to_string(),
        }
    }
}

/// v3 响应验签器（对应 Java `v3/Validator` 接口）。
pub trait Validator: Send + Sync {
    /// 校验微信支付 v3 响应签名（对应 Java `validate(response)`）。
    fn validate(&self, response: &ValidationResponse) -> bool;
}
