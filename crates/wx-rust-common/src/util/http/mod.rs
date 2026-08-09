//! HTTP 抽象与执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http` 包。
//! Java 的 apache/okhttp/jodd/httpcomponents 四后端执行器均为 `PLATFORM_NA`
//! （Rust 以 reqwest 统一）；本模块保留接口抽象与 reqwest 实现。

pub mod http_client_type;
pub mod http_response_proxy;
pub mod input_stream_data;
pub mod media_download_request_executor;
pub mod media_upload_request_executor;
pub mod request_executor;
pub mod request_http;
pub mod simple_get_request_executor;
pub mod simple_post_request_executor;
pub mod uri_util;
pub mod wx_dns_resolver;

pub use http_client_type::HttpClientType;
pub use http_response_proxy::HttpResponseProxy;
pub use input_stream_data::InputStreamData;
pub use media_download_request_executor::MediaDownloadRequestExecutor;
pub use media_upload_request_executor::MediaUploadRequestExecutor;
pub use request_executor::{RequestExecutor, ResponseHandler};
pub use request_http::RequestHttp;
pub use simple_get_request_executor::SimpleGetRequestExecutor;
pub use simple_post_request_executor::SimplePostRequestExecutor;
pub use uri_util::UriUtil;
