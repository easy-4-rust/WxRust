//! HTTP 请求执行器抽象。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.RequestExecutor`。

use async_trait::async_trait;

use crate::enums::WxType;
use crate::error::WxErrorException;

/// HTTP 响应处理器（对应 Java `ResponseHandler<T>`）。
#[async_trait]
pub trait ResponseHandler<T>: Send + Sync {
    /// 处理 HTTP 响应。
    ///
    /// # 参数
    /// - `response`：响应结果
    async fn handle(&self, response: T);
}

/// HTTP 请求执行器策略。
///
/// 对应 Java `RequestExecutor<T, E>` 接口；在 Rust 中以 async trait 表达。
/// 每个 Java 执行器类（SimpleGet/Post、MediaUpload/Download 等）对应一个
/// Rust 实现结构体，持有 `reqwest::Client`。
///
/// # 类型参数
/// - `T`：返回值类型
/// - `E`：请求参数类型
#[async_trait]
pub trait RequestExecutor<T, E>: Send + Sync {
    /// 执行 HTTP 请求。
    ///
    /// # 参数
    /// - `uri`：请求 URI（已含 access_token）
    /// - `data`：请求数据
    /// - `wx_type`：微信模块类型
    ///
    /// # 返回
    /// 响应结果。
    async fn execute(&self, uri: &str, data: E, wx_type: WxType) -> Result<T, WxErrorException>;
}
