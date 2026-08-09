//! 媒体上传请求执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.MediaUploadRequestExecutor`
//! （及 Apache/OkHttp/Jodd/HttpComponents 四后端，均为 `PLATFORM_NA`）。
//! Rust 以 reqwest multipart 统一实现。

use async_trait::async_trait;
use reqwest::multipart::{Form, Part};

use crate::bean::CommonUploadParam;
use crate::enums::WxType;
use crate::error::WxErrorException;
use crate::util::http::RequestExecutor;
use crate::util::http::simple_get_request_executor::SimpleGetRequestExecutor;

/// 媒体上传请求执行器。
///
/// 使用 multipart/form-data 上传媒体文件（对应 Java `MediaUploadRequestExecutor`）。
#[derive(Debug, Clone)]
pub struct MediaUploadRequestExecutor {
    /// reqwest 客户端
    client: reqwest::Client,
}

impl MediaUploadRequestExecutor {
    /// 构建上传执行器。
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
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
        let part = Part::bytes(param.data.content).file_name(file_name);
        form = form.part(param.name, part);
        if let Some(fields) = param.form_fields {
            for (k, v) in fields {
                form = form.text(k, v);
            }
        }
        let resp = self.client.post(uri).multipart(form).send().await?;
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
