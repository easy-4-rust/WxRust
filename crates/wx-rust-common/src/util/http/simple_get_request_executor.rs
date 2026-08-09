//! 简单 GET 请求执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.SimpleGetRequestExecutor`。
//! Java 提供 apache/okhttp/jodd/httpcomponents 四后端；Rust 以 reqwest 统一实现。

use async_trait::async_trait;

use crate::enums::WxType;
use crate::error::{WxError, WxErrorError, WxErrorException};
use crate::util::http::{RequestExecutor, ResponseHandler};

/// 简单的 GET 请求执行器。
///
/// 请求参数是 `String`（query 串），返回结果也是 `String`。
/// 请求前自动校验微信错误码（`errcode != 0` 抛异常）。
#[derive(Debug, Clone)]
pub struct SimpleGetRequestExecutor {
    /// reqwest 客户端
    client: reqwest::Client,
}

impl SimpleGetRequestExecutor {
    /// 构建 GET 执行器。
    ///
    /// # 参数
    /// - `client`：reqwest 客户端
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// 通用响应校验：从响应内容解析 `WxError`，错误码非 0 时抛异常。
    ///
    /// # 参数
    /// - `wx_type`：微信模块类型（用于错误码翻译）
    /// - `response_content`：响应内容
    ///
    /// # 返回
    /// 原始响应内容；错误码非 0 时返回错误。
    pub fn handle_response(
        wx_type: WxType,
        response_content: &str,
    ) -> Result<String, WxErrorException> {
        let error = WxError::from_json_with_type(response_content, Some(wx_type));
        if error.error_code != 0 {
            // 保留完整 `WxError`（含原始报文 `json`），供上层从错误报文回解析业务数据
            // （对应 Java `SimpleGetRequestExecutor.handleResponse` 抛出的
            // `new WxErrorException(error)`；如 miniapp `createRoom` 对 300036
            // 从 `error.getJson()` 回解析 roomId）。
            return Err(WxErrorException::Wx(WxErrorError::new(error)));
        }
        Ok(response_content.to_string())
    }
}

#[async_trait]
impl RequestExecutor<String, String> for SimpleGetRequestExecutor {
    async fn execute(
        &self,
        uri: &str,
        data: String,
        wx_type: WxType,
    ) -> Result<String, WxErrorException> {
        // data 为 query 参数串（如 "a=1&b=2"），拼接到 uri
        let url = if data.is_empty() {
            uri.to_string()
        } else if uri.contains('?') {
            format!("{uri}&{data}")
        } else {
            format!("{uri}?{data}")
        };
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Self::handle_response(wx_type, &body)
    }
}

#[async_trait]
impl ResponseHandler<String> for SimpleGetRequestExecutor {
    async fn handle(&self, response: String) {
        let _ = response;
    }
}
