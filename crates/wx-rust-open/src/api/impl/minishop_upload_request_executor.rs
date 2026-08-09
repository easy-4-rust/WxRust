//! 小商店图片上传请求执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.MinishopUploadRequestExecutor`
//! （Apache/OkHttp/Jodd/HttpComponents 四后端；Apache 线格式：
//! `MultipartEntityBuilder.addBinaryBody("media", file)`）。
//! Rust 以 reqwest multipart 统一实现：字段名 `media`、errcode != 0 抛业务
//! 错误、成功解析 [`WxMinishopImageUploadResult`]。

use std::path::Path;

use async_trait::async_trait;
use reqwest::multipart::{Form, Part};

use wx_rust_common::bean::result::WxMinishopImageUploadResult;
use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::RequestExecutor;

/// 小商店图片上传请求执行器（multipart/form-data）。
///
/// Java 入参为 `File`，Rust 以文件路径字符串表达（ADAPTED）；返回
/// `WxMinishopImageUploadResult`（对应 Java `RequestExecutor<WxMinishop
/// ImageUploadResult, File>`）。
#[derive(Debug, Clone)]
pub struct MinishopUploadRequestExecutor {
    /// reqwest 客户端
    client: reqwest::Client,
}

impl MinishopUploadRequestExecutor {
    /// 构建上传执行器。
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// 以 multipart 形式上传（对应 Java `execute(String uri, File file,
    /// WxType wxType)`）。
    ///
    /// 流程：`addBinaryBody("media", file)` 语义 → errcode != 0 抛
    /// `WxErrorException`（对应 Java `WxError.fromJson` 检查）→ 解析
    /// `WxMinishopImageUploadResult`。
    pub async fn upload(
        &self,
        uri: &str,
        file_path: &str,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        let bytes = tokio::fs::read(file_path)
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("读取上传文件失败: {e}")))?;
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();
        let part = Part::bytes(bytes).file_name(file_name);
        let form = Form::new().part("media", part);
        let resp = self.client.post(uri).multipart(form).send().await?;
        let body = resp.text().await?;
        Self::handle_response(&body)
    }

    /// 解析响应（对应 Java 执行器尾部的 `WxError.fromJson` +
    /// `WxMinishopImageUploadResult.fromJson`）。
    ///
    /// errcode 检查兼容数字/字符串（对应 Java Gson 宽松语义：字符串
    /// `"0"` 可解析为 0；`WxError` 的 serde 为严格 i32，故此处手工解析）。
    fn handle_response(
        response_content: &str,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response_content) {
            let code = json.get("errcode").and_then(|c| match c {
                serde_json::Value::Number(n) => n.as_i64(),
                serde_json::Value::String(s) => s.parse().ok(),
                _ => None,
            });
            if let Some(code) = code {
                if code != 0 {
                    let msg = json
                        .get("errmsg")
                        .and_then(|m| m.as_str())
                        .unwrap_or_default()
                        .to_string();
                    return Err(WxErrorException::from_code(code as i32, msg));
                }
            }
        }
        serde_json::from_str(response_content).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl RequestExecutor<WxMinishopImageUploadResult, String> for MinishopUploadRequestExecutor {
    async fn execute(
        &self,
        uri: &str,
        data: String,
        _wx_type: WxType,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        self.upload(uri, &data).await
    }
}
