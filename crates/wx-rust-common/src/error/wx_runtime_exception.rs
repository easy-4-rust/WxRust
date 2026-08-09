//! WxRust 专用运行时异常。
//!
//! 对应 Java `me.chanjar.weixin.common.error.WxRuntimeException`。

/// WxRust 专用运行时异常。
///
/// 用于执行引擎的致命错误（重试超限、token 获取超时、中断等）——这些错误
/// 在 Java 中抛 `WxRuntimeException`（unchecked），Rust 中作为 [`crate::error::WxErrorException`]
/// 的 `Runtime` 变体承载。
#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub struct WxRuntimeError {
    /// 运行时错误信息
    pub message: String,
}

impl WxRuntimeError {
    /// 用错误信息构建运行时异常。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
