//! 微信支付签名探测异常。
//!
//! 对应 Java `com.github.binarywang.wxpay.exception.WxSignTestException`
//! （`extends WxPayException`，仅两个构造器、无新增字段）：用于签名探测
//! （`WxSignQueryRequest`/sign test 流程）失败时区分于普通支付异常。
//!
//! ADAPTED：Rust 无继承，以携带 [`WxPayException`] 的结构体承载同一
//! "子类 + 自定义文案" 语义，经 [`From`] 可作为父类异常使用。

use wx_rust_common::error::{WxErrorException, WxRuntimeError};

use super::WxPayException;

/// 签名探测异常（对应 Java `WxSignTestException extends WxPayException`）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WxSignTestException {
    /// 父类形态载体（对应 Java 继承的 `WxPayException` 部分）。
    pub inner: WxPayException,
}

impl WxSignTestException {
    /// 以自定义文案构造（对应 Java
    /// `WxSignTestException(String customErrorMsg)`）。
    pub fn new(custom_error_msg: impl Into<String>) -> Self {
        Self {
            inner: WxPayException::new(custom_error_msg),
        }
    }
}

impl std::fmt::Display for WxSignTestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.build_error_msg())
    }
}

impl std::error::Error for WxSignTestException {}

impl From<WxSignTestException> for WxPayException {
    /// 子类 → 父类（对应 Java 继承的向上转型）。
    fn from(e: WxSignTestException) -> Self {
        e.inner
    }
}

impl From<WxSignTestException> for WxErrorException {
    fn from(e: WxSignTestException) -> Self {
        WxErrorException::Runtime(WxRuntimeError::new(e.inner.build_error_msg()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_custom_message_and_upcasts() {
        let e = WxSignTestException::new("签名探测失败");
        assert_eq!(e.to_string(), "签名探测失败");

        // 向上转型 + 错误通道
        let parent: WxPayException = e.into();
        assert_eq!(parent.custom_error_msg(), Some("签名探测失败"));
        let e2 = WxSignTestException::new("again");
        let wx_err: WxErrorException = e2.into();
        assert!(wx_err.to_string().contains("again"));
    }
}
