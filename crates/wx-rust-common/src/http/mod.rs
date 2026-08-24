//! HTTP 传输抽象层。
//!
//! RUST_OBLIGATION：传输可注入性。Java 通过可替换 HttpClient 后端实现传输
//! 注入；WxRust 以 [`HttpTransport`] trait 承接：执行管线依赖 trait 对象而非
//! 具体 reqwest 客户端，生产注入 [`ReqwestTransport`]，测试注入零网络的
//! [`MockTransport`]。

pub mod transport;

pub use transport::{
    HttpTransport, MockTransport, ReqwestTransport, TransportBody, TransportMethod,
    TransportRequest, TransportResponse,
};
