//! 流式执行管线（[`execute_stream`]）。
//!
//! RUST_OBLIGATION：大文件可流式。[`super::execute_pipeline`] 按「JSON
//! 报文 + errcode 校验」聚合全量 body 工作；本模块承接不适配该模型的
//! 大文件下载（pay 对账单先行）：响应体以 [`bytes::Bytes`] 分块流交付，
//! 传输层不聚合全量 body，调用方按需消费（渐进落盘/边收边解压）。
//!
//! 设计决定（Task 7，已裁定）：入口接受 `&ReqwestTransport` 而非
//! `&dyn HttpTransport`，流式能力以 [`crate::http::ReqwestTransport::send_stream`]
//! 实现侧方法承载——
//! - 不给 [`crate::http::HttpTransport`] trait 加默认未实现的
//!   `send_stream`（trait 保持最小，避免「默认 Err(-99)」的死代码路径）；
//! - 当前仅 reqwest 后端需要流式（hyper 分块读）；需要零网络流式的测试
//!   用 `futures_util::stream::iter` 构造分块流即可，无需 MockTransport
//!   支持流式。
//!
//! 错误语义（与 [`crate::http::ReqwestTransport::send_stream`] 一致）：
//! - 传输建立失败 / 非 2xx 状态码（如 500）：直接返回 `Err`（不产出流）；
//! - 流中读取失败：以对应分块 `Err(WxErrorException::Http)` 表达。
//!
//! 内容语义：**原始字节透传**（如 GZIP 对账单不解压）——解压/文本/错误
//! 报文检测由调用方按需处理（pay 侧 `download_bill_stream` 沿用同一约定）。

use bytes::Bytes;
use futures_util::Stream;

use crate::error::WxErrorException;
use crate::http::{ReqwestTransport, TransportRequest};

/// 流式执行下载请求，返回响应体分块流。
///
/// 请求构造与 [`crate::http::HttpTransport::send`] 同构（复用
/// [`ReqwestTransport`] 的统一映射：方法/头/体 → reqwest 请求）；
/// 差异仅在响应消费——分块流交付，不聚合全量 body。
///
/// # 参数
/// - `transport`：reqwest 传输（流式为实现侧能力，见模块文档设计决定）
/// - `req`：完整请求描述（GET 下载传 `TransportMethod::Get` + 空 body；
///   pay 对账单等 POST 下载传 `TransportMethod::PostXml` 等）
///
/// # 返回
/// 分块流：每项为一个响应分块 [`Bytes`]；流中读取失败以对应项 `Err`
/// 表达。流不借用 `transport`（可 `.boxed()` 成 `'static`）。
///
/// # 适用边界
/// 二进制/大文本下载（对账单、媒体文件）；需要「errcode 校验 + token
/// 失效重放」语义的 JSON 接口仍走 [`super::execute_pipeline`]。
pub async fn execute_stream(
    transport: &ReqwestTransport,
    req: TransportRequest,
) -> Result<impl Stream<Item = Result<Bytes, WxErrorException>> + Send + use<>, WxErrorException> {
    transport.send_stream(req).await
}
