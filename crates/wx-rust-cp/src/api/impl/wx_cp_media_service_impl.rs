//! 素材（媒体）服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpMediaServiceImpl`。
//! 上传走 common 的 `MediaUploadRequestExecutor`（multipart，字段名
//! `media`）经执行引擎（token 注入 + 自动刷新 + 重试）执行；下载镜像
//! Java `BaseMediaDownloadRequestExecutor` 语义（JSON 响应视为微信错误
//! 报文抛错，否则返回文件字节，Rust 以 `Vec<u8>` 表达 Java 临时 `File`，
//! ADAPTED）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, WxErrorException};
use wx_rust_common::util::http::MediaUploadRequestExecutor;

use crate::api::{WxCpMediaService, WxCpService};
use crate::bean::{MediaUploadByUrlReq, MediaUploadByUrlResult};
use crate::enums::url_media::*;

/// 素材（媒体）服务实现。
pub struct WxCpMediaServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpMediaServiceImpl {
    /// 构建素材服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 以 multipart（字段名 `media`）上传素材并解析上传结果。
    ///
    /// 对应 Java `mainService.execute(MediaInputStreamUploadRequestExecutor /
    /// MediaUploadRequestExecutor, MEDIA_UPLOAD + mediaType, ...)`：走执行引擎
    /// （token 注入 + 自动刷新 + 重试），再 `WxMediaUploadResult.fromJson`。
    async fn upload_media_common(
        svc: &dyn WxCpService,
        media_type: &str,
        data: CommonUploadData,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let config = svc.wx_cp_config_storage();
        let url = format!("{}{media_type}", config.api_url(MEDIA_UPLOAD));
        let executor = MediaUploadRequestExecutor::new(svc.http_client().clone());
        let param = CommonUploadParam::new("media", data);
        let response = crate::api::r#impl::base_wx_cp_service_impl::execute_with_retry(
            svc, &executor, &url, param,
        )
        .await?;
        serde_json::from_str::<WxMediaUploadResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 下载素材到字节（对应 Java `BaseMediaDownloadRequestExecutor`：
    /// GET `url?media_id=...`，Content-Type 为 JSON 时视为微信错误报文并
    /// 抛错，否则返回文件字节）。
    async fn download_media_common(
        svc: &dyn WxCpService,
        api_path: &str,
        media_id: &str,
    ) -> Result<Vec<u8>, WxErrorException> {
        let config = svc.wx_cp_config_storage();
        let token = svc.get_access_token().await?;
        let url = format!(
            "{}?access_token={token}&media_id={media_id}",
            config.api_url(api_path)
        );
        let resp = svc
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
        let is_json = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.starts_with("application/json"))
            .unwrap_or(false);
        if is_json {
            let body = resp
                .text()
                .await
                .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
            let error = WxError::from_json_with_type(&body, Some(WxType::Cp));
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
        Ok(bytes.to_vec())
    }
}

#[async_trait]
impl WxCpMediaService for WxCpMediaServiceImpl {
    async fn upload(
        &self,
        media_type: &str,
        file_type: &str,
        input: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `upload(String, String, InputStream)`：先
        // `FileUtils.createTmpFile(inputStream, UUID, fileType)` 落临时文件，
        // 文件名为 UUID.fileType，再上传；Rust 直接以上传字节表达（ADAPTED）
        let file_name = Some(format!("{}.{file_type}", random_uuid()));
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(file_name, input),
        )
        .await
    }

    async fn upload_with_url(
        &self,
        media_type: &str,
        filename: &str,
        url: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `upload(String, String, String)`：`HttpURLConnection` 下载远端
        // 文件（连接超时 60s + User-Agent 防 403）后以 `InputStreamData`
        // 上传；Rust 以 reqwest 下载字节后上传（ADAPTED）
        let resp = svc
            .http_client()
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                "Mozilla/4.0 (compatible; MSIE 5.0; Windows NT; DigExt)",
            )
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("远端文件下载失败: {e}")))?;
        let input = resp
            .bytes()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("远端文件下载失败: {e}")))?;
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(Some(filename.to_string()), input.to_vec()),
        )
        .await
    }

    async fn upload_with_file_path(
        &self,
        media_type: &str,
        file_path: &str,
        filename: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `upload(String, File, String)`：文件不存在抛
        // `WxRuntimeException("文件[绝对路径]不存在")`（Rust 以 -99 表达）
        if !std::path::Path::new(file_path).exists() {
            return Err(WxErrorException::from_code(
                -99,
                format!("文件[{file_path}]不存在"),
            ));
        }
        let content = std::fs::read(file_path).map_err(|e| WxErrorException::Io(e.to_string()))?;
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(Some(filename.to_string()), content),
        )
        .await
    }

    async fn upload_with_stream(
        &self,
        media_type: &str,
        input: Vec<u8>,
        filename: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `upload(String, InputStream, String)`：直接以 `InputStreamData`
        // 上传
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(Some(filename.to_string()), input),
        )
        .await
    }

    async fn upload_with_file(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `upload(String, File)`：文件名取 `File.getName()`
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let content = std::fs::read(file_path).map_err(|e| WxErrorException::Io(e.to_string()))?;
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(file_name, content),
        )
        .await
    }

    async fn download(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `download`：`BaseMediaDownloadRequestExecutor` 执行
        // `MEDIA_GET`，查询参数 `media_id=`（视频文件下载不了，会返回空）
        Self::download_media_common(svc.as_ref(), MEDIA_GET, media_id).await
    }

    async fn get_jssdk_file(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getJssdkFile`：`BaseMediaDownloadRequestExecutor` 执行
        // `JSSDK_MEDIA_GET`
        Self::download_media_common(svc.as_ref(), JSSDK_MEDIA_GET, media_id).await
    }

    async fn upload_img(&self, file_path: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `uploadImg`：`MediaUploadRequestExecutor` 执行 `IMG_UPLOAD`，
        // 结果取 `url` 字段
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let content = std::fs::read(file_path).map_err(|e| WxErrorException::Io(e.to_string()))?;
        let config = svc.wx_cp_config_storage();
        let executor = MediaUploadRequestExecutor::new(svc.http_client().clone());
        let param = CommonUploadParam::new("media", CommonUploadData::new(file_name, content));
        let response = crate::api::r#impl::base_wx_cp_service_impl::execute_with_retry(
            svc.as_ref(),
            &executor,
            &config.api_url(IMG_UPLOAD),
            param,
        )
        .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("url")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "url 字段缺失"))
    }

    async fn upload_by_url(&self, req: &MediaUploadByUrlReq) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `uploadByUrl(MediaUploadByUrlReq)`：POST `UPLOAD_BY_URL`，
        // 响应取 `jobid`
        let config = svc.wx_cp_config_storage();
        let body = req.to_json().map_err(WxErrorException::Serde)?;
        let response_content = svc.post(&config.api_url(UPLOAD_BY_URL), &body).await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("jobid")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "jobid 字段缺失"))
    }

    async fn upload_by_url_result(
        &self,
        job_id: &str,
    ) -> Result<MediaUploadByUrlResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `uploadByUrl(String)`：POST `GET_UPLOAD_BY_URL_RESULT`
        // `{"jobid":...}`，整体响应 `MediaUploadByUrlResult.fromJson`
        let body = serde_json::json!({ "jobid": job_id }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(GET_UPLOAD_BY_URL_RESULT), &body)
            .await?;
        MediaUploadByUrlResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }
}

/// 生成伪 UUID 字符串（对应 Java `UUID.randomUUID().toString()` 的
/// 临时文件名语义，简化实现）。
fn random_uuid() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{now:x}{:x}", std::process::id())
}
