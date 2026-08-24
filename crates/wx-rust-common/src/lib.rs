//! WxRust 公共基础设施 crate。
//!
//! 对应 Java `me.chanjar.weixin.common` 模块，为各业务 crate 提供：
//! - 错误模型（`WxError` / `WxErrorException` / 分平台错误码翻译表）
//! - 数据对象（access token、jsapi 签名、OCR、菜单、订阅消息等）
//! - 会话管理、重复消息检查、请求执行器抽象、配置存储抽象
//! - HTTP 传输抽象与统一执行管线（token 注入 + 失效单次重放）
//! - 加密工具（SHA1 / 微信消息加解密 / PKCS7）
//!
//! 设计约束：全 crate `#![forbid(unsafe_code)]`，HTTP 统一使用 `reqwest`。

#![forbid(unsafe_code)]

pub mod annotation;
pub mod api;
pub mod bean;
pub mod config;
pub mod enums;
pub mod error;
pub mod executor;
pub mod http;
pub mod pipeline;
pub mod redis;
pub mod service;
pub mod session;
pub mod util;

pub use error::{WxError, WxErrorException, WxRuntimeError};
