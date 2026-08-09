//! 错误模型。
//!
//! 对应 Java `me.chanjar.weixin.common.error` 包。
//! 提供 `WxError`（错误码对象）、统一错误枚举 [`WxErrorException`] 与分平台错误码翻译表。

pub mod wx_channel_error_msg_enum;
pub mod wx_cp_error_msg_enum;
pub mod wx_error;
pub mod wx_error_exception;
pub mod wx_ma_error_msg_enum;
pub mod wx_mp_error_msg_enum;
pub mod wx_open_error_msg_enum;
pub mod wx_runtime_exception;

pub use wx_error::WxError;
pub use wx_error_exception::{DEFAULT_ERROR_CODE, WxErrorError};
pub use wx_runtime_exception::WxRuntimeError;

use crate::enums::WxType;

/// 微信接口调用统一错误类型。
///
/// 对应 Java `WxErrorException`（checked）与 `WxRuntimeException`（unchecked）的合并；
/// 所有 Service 方法以 `Result<T, WxErrorException>` 返回。
#[derive(Debug, Clone, thiserror::Error)]
pub enum WxErrorException {
    /// 微信接口返回的业务错误（对应 Java `WxErrorException(WxError)`）
    #[error("{0}")]
    Wx(#[from] WxErrorError),

    /// 运行时错误（对应 Java `WxRuntimeException`：重试超限、token 超时等）
    #[error("{0}")]
    Runtime(#[from] WxRuntimeError),

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(String),

    /// HTTP 请求错误
    #[error("HTTP 错误: {0}")]
    Http(String),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serde(String),
}

impl WxErrorException {
    /// 返回微信错误码；非业务错误时返回 `None`。
    pub fn error_code(&self) -> Option<i32> {
        match self {
            WxErrorException::Wx(e) => Some(e.error.error_code),
            _ => None,
        }
    }

    /// 返回微信错误对象引用（仅业务错误变体）。
    pub fn wx_error(&self) -> Option<&WxError> {
        match self {
            WxErrorException::Wx(e) => Some(&e.error),
            _ => None,
        }
    }

    /// 由微信错误码与错误信息构建业务错误。
    pub fn from_code(code: i32, msg: impl Into<String>) -> Self {
        WxErrorException::Wx(WxErrorError::new(WxError::new(code, msg)))
    }
}

impl From<std::io::Error> for WxErrorException {
    fn from(e: std::io::Error) -> Self {
        WxErrorException::Io(e.to_string())
    }
}

impl From<reqwest::Error> for WxErrorException {
    fn from(e: reqwest::Error) -> Self {
        WxErrorException::Http(e.to_string())
    }
}

impl From<serde_json::Error> for WxErrorException {
    fn from(e: serde_json::Error) -> Self {
        WxErrorException::Serde(e.to_string())
    }
}

/// 按平台查找错误码的中文翻译。
///
/// # 参数
/// - `wx_type`：微信平台类型
/// - `code`：微信错误码
///
/// # 返回
/// 该平台错误码对应的中文信息；未收录时返回 `None`。
pub fn translate_error_msg(wx_type: WxType, code: i32) -> Option<&'static str> {
    match wx_type {
        WxType::Mp => wx_mp_error_msg_enum::find_msg_by_code(code),
        WxType::Cp => wx_cp_error_msg_enum::find_msg_by_code(code),
        WxType::MiniApp => wx_ma_error_msg_enum::find_msg_by_code(code),
        WxType::Open => wx_open_error_msg_enum::find_msg_by_code(code),
        WxType::Channel => wx_channel_error_msg_enum::find_msg_by_code(code),
        // 支付无独立错误码翻译表（与 Java 一致：Pay 走 default 分支不翻译）
        WxType::Pay => None,
    }
}
