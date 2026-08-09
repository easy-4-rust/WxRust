//! HTTP 客户端抽象（对应 Java `RequestHttp<H, P>`）。
//!
//! Java 中以泛型 `RequestHttp<H, P>` 抽象多 HTTP 后端（`H`=客户端、`P`=代理）。
//! WxRust 统一使用 reqwest，此 trait 保留 getter 语义，实现由各模块持有
//! `reqwest::Client` 的结构体提供。

use super::HttpClientType;

/// HTTP 客户端抽象。
///
/// # 注意
/// Java 泛型 `H`（客户端类型）与 `P`（代理类型）在 Rust 中由 `reqwest::Client`
/// 统一承载；此 trait 仅保留类型查询语义。
pub trait RequestHttp: Send + Sync {
    /// 返回 HTTP 客户端类型。
    fn request_type(&self) -> HttpClientType;
}
