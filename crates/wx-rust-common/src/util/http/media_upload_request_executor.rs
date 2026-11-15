//! 媒体上传请求执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.MediaUploadRequestExecutor`
//! （及 Apache/OkHttp/Jodd/HttpComponents 四后端，均为 `PLATFORM_NA`）。
//! Rust 以 reqwest multipart 统一实现。

use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use std::sync::Arc;

use crate::bean::CommonUploadParam;
use crate::enums::WxType;
use crate::error::WxErrorException;
use crate::http::{HttpRequestInterceptor, send_with_interceptors};
use crate::util::http::RequestExecutor;
use crate::util::http::simple_get_request_executor::SimpleGetRequestExecutor;

/// 媒体上传请求执行器。
///
/// 使用 multipart/form-data 上传媒体文件（对应 Java `MediaUploadRequestExecutor`）。
#[derive(Clone)]
pub struct MediaUploadRequestExecutor {
    /// reqwest 客户端
    client: reqwest::Client,
    /// 有序的 per-request interceptor 链
    interceptors: Vec<HttpRequestInterceptor>,
}

impl MediaUploadRequestExecutor {
    /// 构建上传执行器。
    pub fn new(client: reqwest::Client) -> Self {
        Self::new_with_interceptors(client, Vec::new())
    }

    /// 使用调用方配置的 per-request interceptor 链构建上传执行器。
    pub fn new_with_interceptors(
        client: reqwest::Client,
        interceptors: Vec<HttpRequestInterceptor>,
    ) -> Self {
        Self {
            client,
            interceptors,
        }
    }

    /// 以 multipart 形式上传。
    ///
    /// # 参数
    /// - `uri`：上传接口地址
    /// - `param`：上传参数（文件参数名 + 数据 + 额外表单字段）
    /// - `wx_type`：微信模块类型
    pub async fn upload(
        &self,
        uri: &str,
        param: CommonUploadParam,
        wx_type: WxType,
    ) -> Result<String, WxErrorException> {
        let mut form = Form::new();
        let file_name = param
            .data
            .file_name
            .clone()
            .unwrap_or_else(|| "file".to_string());
        let part = Part::bytes(param.data.content)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .map_err(|error| WxErrorException::Http(error.to_string()))?;
        form = form.part(param.name, part);
        if let Some(fields) = param.form_fields {
            for (name, value) in fields {
                let mut part_headers = reqwest::header::HeaderMap::new();
                part_headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("text/plain; charset=UTF-8"),
                );
                let text_part = Part::text(value).headers(part_headers);
                form = form.part(name, text_part);
            }
        }
        // Apache HttpComponents EXTENDED emits an outer ISO-8859-1 charset.
        // Keep Reqwest's generated boundary unchanged while matching that header.
        let mut interceptors = Vec::with_capacity(self.interceptors.len() + 1);
        interceptors.push(Arc::new(|request: &mut reqwest::Request| {
            let current = request
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned);
            if let Some(current) = current {
                if let Some((_, boundary)) = current.split_once("boundary=") {
                    let compatible = reqwest::header::HeaderValue::from_str(&format!(
                        "multipart/form-data; charset=ISO-8859-1; boundary={boundary}"
                    ))
                    .map_err(|error| WxErrorException::Http(error.to_string()))?;
                    request
                        .headers_mut()
                        .insert(reqwest::header::CONTENT_TYPE, compatible);
                }
            }
            Ok(())
        }) as HttpRequestInterceptor);
        interceptors.extend(self.interceptors.iter().cloned());
        let resp = send_with_interceptors(
            &self.client,
            self.client.post(uri).multipart(form),
            &interceptors,
        )
        .await?;
        let body = resp.text().await?;
        SimpleGetRequestExecutor::handle_response(wx_type, &body)
    }
}

#[async_trait]
impl RequestExecutor<String, CommonUploadParam> for MediaUploadRequestExecutor {
    async fn execute(
        &self,
        uri: &str,
        data: CommonUploadParam,
        wx_type: WxType,
    ) -> Result<String, WxErrorException> {
        self.upload(uri, data, wx_type).await
    }
}
