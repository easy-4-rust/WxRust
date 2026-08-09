//! 临时素材服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaMediaServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::{WxError, WxErrorException};
use wx_rust_common::util::http::MediaUploadRequestExecutor;

use crate::api::{WxMaMediaService, WxMaService};
use crate::enums::url_g1_core::media as media_url;

/// 临时素材服务实现。
pub struct WxMaMediaServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaMediaServiceImpl {
    /// 构建临时素材服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 以 multipart（字段名 `media`）上传素材并解析上传结果。
    ///
    /// 对应 Java `uploadMedia`：`wxMaService.upload(url,
    /// CommonUploadParam.fromFile("media", file))` 走执行引擎（token 注入 +
    /// 自动刷新 + 重试），再 `WxMediaUploadResult.fromJson(result)`。
    async fn upload_media_common(
        svc: &dyn WxMaService,
        media_type: &str,
        data: CommonUploadData,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let config = svc.wx_ma_config();
        let url = media_url::media_upload_url(config.as_ref(), media_type);
        let executor = MediaUploadRequestExecutor::new(svc.http_client().clone());
        let param = CommonUploadParam::new("media", data);
        let response = crate::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            svc, &executor, &url, param,
        )
        .await?;
        serde_json::from_str::<WxMediaUploadResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxMaMediaService for WxMaMediaServiceImpl {
    async fn upload_media(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `uploadMedia(String, File)`：文件内容 + 文件名（`File.getName()`）
        let content = std::fs::read(file_path).map_err(|e| WxErrorException::Io(e.to_string()))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(file_name, content),
        )
        .await
    }

    async fn upload_media_with_stream(
        &self,
        media_type: &str,
        file_type: &str,
        input: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `uploadMedia(String, String, InputStream)`：先
        // `FileUtils.createTmpFile(inputStream, UUID, fileType)` 落临时文件再上传；
        // Rust 直接以上传字节表达（ADAPTED），文件名取时间戳 + 随机后缀 + fileType
        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let file_name = Some(format!("{now_millis}_{}.{file_type}", rand_suffix()));
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(file_name, input),
        )
        .await
    }

    async fn get_media(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getMedia`：`BaseMediaDownloadRequestExecutor` 执行
        // `MEDIA_GET_URL`（`https://api.weixin.qq.com/cgi-bin/media/get`），
        // 查询参数 `media_id=`；响应为 JSON 时视为微信错误报文并抛错，否则
        // 保存到本地临时目录并返回 File（Rust 直接返回文件字节，ADAPTED）。
        let config = svc.wx_ma_config();
        let token = svc.get_access_token().await?;
        let url = media_url::media_get_url(config.as_ref());
        let url = format!("{url}?access_token={token}&media_id={media_id}");
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
            let error =
                WxError::from_json_with_type(&body, Some(wx_rust_common::enums::WxType::MiniApp));
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

/// 生成随机文件名后缀（对应 Java `UUID.randomUUID()` 语义，简化实现）。
fn rand_suffix() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    nanos ^ (std::process::id() as u64).wrapping_mul(0x9E3779B97F4A7C15)
}
