//! 通用执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.executor` 包。
//! Java 的 `CommonUploadRequestExecutor`（抽象）及 Apache/OkHttp/Jodd/
//! HttpComponents 四后端实现；Rust 中以 reqwest 统一实现
//! [`crate::util::http::MediaUploadRequestExecutor`] 承载（多后端 `PLATFORM_NA`）。
//! 本模块为兼容入口，重导出统一上传执行器。

pub use crate::util::http::MediaUploadRequestExecutor as CommonUploadRequestExecutor;
