//! 简单 POST 请求执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.SimplePostRequestExecutor`。
//! Java 提供 apache/okhttp/jodd/httpcomponents 四后端；Rust 以 reqwest 统一实现。

use async_trait::async_trait;

use crate::enums::WxType;
use crate::error::WxErrorException;
use crate::util::http::simple_get_request_executor::SimpleGetRequestExecutor;
use crate::util::http::{RequestExecutor, ResponseHandler};

/// 简单的 POST 请求执行器。
///
/// 请求参数是 `String`（JSON 或 form 内容），返回结果也是 `String`。
#[derive(Debug, Clone)]
pub struct SimplePostRequestExecutor {
    /// reqwest 客户端
    client: reqwest::Client,
}

impl SimplePostRequestExecutor {
    /// 构建 POST 执行器。
    ///
    /// # 参数
    /// - `client`：reqwest 客户端
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RequestExecutor<String, String> for SimplePostRequestExecutor {
    async fn execute(
        &self,
        uri: &str,
        data: String,
        wx_type: WxType,
    ) -> Result<String, WxErrorException> {
        let resp = self.client.post(uri).body(data).send().await?;
        let body = resp.text().await?;
        SimpleGetRequestExecutor::handle_response(wx_type, &body)
    }
}

#[async_trait]
impl ResponseHandler<String> for SimplePostRequestExecutor {
    async fn handle(&self, response: String) {
        let _ = response;
    }
}
