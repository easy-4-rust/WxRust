//! HTTP 传输抽象：请求/响应数据类型与 [`HttpTransport`] trait 的两个实现
//! （生产用 [`ReqwestTransport`]、测试用 [`MockTransport`]）。
//!
//! RUST_OBLIGATION：传输可注入性。Java 侧通过 `WxService` 持有可替换的
//! HttpClient（apache/okhttp 后端可换）实现传输注入；WxRust 以本 trait 承接
//! 同一义务——执行管线依赖 `dyn HttpTransport` 而非具体 reqwest 客户端，
//! 从而支持测试零网络（MockTransport）与后续传输增强（熔断、流式）注入。

use std::sync::Arc;

use async_trait::async_trait;

use crate::error::WxErrorException;

/// 传输请求：与具体 HTTP 后端无关的请求描述。
///
/// RUST_OBLIGATION：传输可注入性——请求以数据形式表达（方法 + URL + 头 + 体），
/// 由 [`HttpTransport`] 实现负责映射到具体后端。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportRequest {
    /// 请求方法与可能内联在方法中的载荷（JSON/XML/表单）
    pub method: TransportMethod,
    /// 完整请求 URL
    pub url: String,
    /// 附加请求头（键值对）
    pub headers: Vec<(String, String)>,
    /// 原始请求体（与 `method` 内联载荷互斥使用，`method` 优先）
    pub body: TransportBody,
}

/// 传输方法与内联载荷。
///
/// `PostJson` / `PostXml` / `PostForm` 携带各自载荷并隐含对应 Content-Type
/// （`application/json` / `text/xml` / 表单编码），与 Java 各执行器
/// （`SimplePostRequestExecutor` 等）的请求语义对齐。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportMethod {
    /// HTTP GET
    Get,
    /// HTTP POST（体由 [`TransportBody`] 提供）
    Post,
    /// POST JSON 字符串（Content-Type: application/json）
    PostJson(String),
    /// POST XML 字符串（Content-Type: text/xml）
    PostXml(String),
    /// POST 表单（application/x-www-form-urlencoded）
    PostForm(Vec<(String, String)>),
}

/// 传输请求体。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportBody {
    /// 无请求体
    None,
    /// 文本请求体
    Text(String),
    /// 字节请求体（二进制上传等）
    Bytes(Vec<u8>),
}

/// 传输响应：状态码 + 响应头 + 字节体（与具体 HTTP 后端无关）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportResponse {
    /// HTTP 状态码
    pub status: u16,
    /// 响应头（键值对）
    pub headers: Vec<(String, String)>,
    /// 响应体字节
    pub body: Vec<u8>,
}

/// HTTP 传输抽象。
///
/// RUST_OBLIGATION：传输可注入性。执行管线与各 Service 依赖本 trait，
/// 生产环境注入 [`ReqwestTransport`]，测试注入 [`MockTransport`]（零网络）。
#[async_trait]
pub trait HttpTransport: Send + Sync {
    /// 发送请求并返回响应。
    ///
    /// # 错误
    /// 传输失败（连接/超时/协议错误）时返回 [`WxErrorException`]（Http 变体）。
    async fn send(&self, req: TransportRequest) -> Result<TransportResponse, WxErrorException>;
}

/// 基于 reqwest 的生产传输实现。
///
/// 对应 Java 侧各 HTTP 后端（apache/okhttp）的默认实现；WxRust 统一 reqwest。
/// 复用调用方构造好的 `reqwest::Client`（连接池/超时/DNS 配置随客户端走）。
pub struct ReqwestTransport {
    /// 复用的 reqwest 客户端
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// 用既有 reqwest 客户端构建传输。
    ///
    /// # 参数
    /// - `client`：调用方配置好的客户端（超时/代理/连接池等在此客户端上生效）
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// 把 [`TransportRequest`] 映射为 reqwest 请求构造器。
    fn build_request(&self, req: &TransportRequest) -> reqwest::RequestBuilder {
        let mut builder = match &req.method {
            TransportMethod::Get => self.client.get(&req.url),
            TransportMethod::Post => self.client.post(&req.url),
            TransportMethod::PostJson(payload) => self
                .client
                .post(&req.url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload.clone()),
            TransportMethod::PostXml(payload) => self
                .client
                .post(&req.url)
                .header(reqwest::header::CONTENT_TYPE, "text/xml")
                .body(payload.clone()),
            // reqwest 未启用 form feature，沿用仓库既有 url::form_urlencoded 编码
            // （与 Java/reqwest `.form()` 的 application/x-www-form-urlencoded 语义一致）
            TransportMethod::PostForm(pairs) => {
                let form_body = url::form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(pairs)
                    .finish();
                self.client
                    .post(&req.url)
                    .header(
                        reqwest::header::CONTENT_TYPE,
                        "application/x-www-form-urlencoded",
                    )
                    .body(form_body)
            }
        };
        for (name, value) in &req.headers {
            builder = builder.header(name.as_str(), value.as_str());
        }
        // method 未内联载荷时（Get/Post）应用原始请求体
        builder = match (&req.method, req.body.clone()) {
            (TransportMethod::Get | TransportMethod::Post, TransportBody::Text(text)) => {
                builder.body(text)
            }
            (TransportMethod::Get | TransportMethod::Post, TransportBody::Bytes(bytes)) => {
                builder.body(bytes)
            }
            _ => builder,
        };
        builder
    }
}

#[async_trait]
impl HttpTransport for ReqwestTransport {
    async fn send(&self, req: TransportRequest) -> Result<TransportResponse, WxErrorException> {
        let resp = self.build_request(&req).send().await?;
        let status = resp.status().as_u16();
        let headers = resp
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_string(),
                    String::from_utf8_lossy(value.as_bytes()).into_owned(),
                )
            })
            .collect();
        let body = resp.bytes().await?.to_vec();
        Ok(TransportResponse {
            status,
            headers,
            body,
        })
    }
}

/// Mock 应答闭包类型（Arc 共享，便于 clone 到并发测试任务）。
type MockHandler =
    Arc<dyn Fn(&TransportRequest) -> Result<TransportResponse, WxErrorException> + Send + Sync>;

/// 测试用 mock 传输：以闭包应答请求，零网络。
///
/// RUST_OBLIGATION：传输可注入性——测试通过注入本实现脱离真实网络，
/// 断言请求构造与管线行为（token 注入/重放/熔断）。
pub struct MockTransport {
    /// 应答闭包
    handler: MockHandler,
}

impl MockTransport {
    /// 用应答闭包构建 mock 传输。
    ///
    /// # 参数
    /// - `f`：接收请求引用、返回应答（或错误）的闭包
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&TransportRequest) -> Result<TransportResponse, WxErrorException>
            + Send
            + Sync
            + 'static,
    {
        Self {
            handler: Arc::new(f),
        }
    }

    /// 便捷构造：固定应答 200 + 指定 JSON 文本。
    ///
    /// # 参数
    /// - `body`：应答 JSON 文本（按字节原样返回）
    pub fn ok_json(body: &str) -> Self {
        let body = body.to_string();
        Self::new(move |_| {
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: body.clone().into_bytes(),
            })
        })
    }
}

#[async_trait]
impl HttpTransport for MockTransport {
    async fn send(&self, req: TransportRequest) -> Result<TransportResponse, WxErrorException> {
        (self.handler)(&req)
    }
}
