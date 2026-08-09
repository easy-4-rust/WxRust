//! 微信接口调用异常。
//!
//! 对应 Java `me.chanjar.weixin.common.error.WxErrorException`。

use crate::error::WxError;

/// 微信接口调用异常，携带错误码对象。
///
/// 对应 Java checked `WxErrorException`；在 Rust 中作为 [`crate::error::WxErrorException`] 枚举的
/// 核心变体使用，由各 Service 方法以 `Result<T, WxErrorException>` 返回。
#[derive(Debug, Clone, thiserror::Error)]
#[error("{error}")]
pub struct WxErrorError {
    /// 微信错误对象
    pub error: WxError,
}

impl WxErrorError {
    /// 用指定 `WxError` 构建异常。
    ///
    /// # 参数
    /// - `error`：微信错误对象
    pub fn new(error: WxError) -> Self {
        Self { error }
    }

    /// 用错误信息构建异常，错误码为默认值 -99。
    ///
    /// # 参数
    /// - `message`：错误信息
    pub fn from_message(message: impl Into<String>) -> Self {
        Self {
            error: WxError::new(-99, message),
        }
    }

    /// 返回错误码对象。
    pub fn error(&self) -> &WxError {
        &self.error
    }
}

/// 微信错误码默认值（消息型异常使用）。
pub const DEFAULT_ERROR_CODE: i32 = -99;
