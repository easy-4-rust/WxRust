//! 企业微信第三方应用素材服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpMediaServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面。上传接口使用授权企业的
//! access_token；Java 以 `MediaUploadRequestExecutor`（multipart）执行，
//! Rust 以 reqwest multipart 表达（ADAPTED：同一线格式
//! `multipart/form-data` 的 `media` 文件字段）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::error::WxErrorException;

use crate::enums::url_media;
use crate::tp::service::{WxCpTpMediaService, WxCpTpService};

/// 企业微信第三方应用素材服务实现。
pub struct WxCpTpMediaServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpMediaServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 以 multipart 上传文件到指定 url（对应 Java
    /// `MediaUploadRequestExecutor` 的 `media` 字段 + 文件体）。
    async fn upload_file(
        &self,
        url: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let service = self.service()?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "upload".to_string());
        let bytes = tokio::fs::read(file_path)
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let part = reqwest::multipart::Part::bytes(bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .map_err(|e| WxErrorException::from_code(-99, format!("构造 multipart 失败: {e}")))?;
        let form = reqwest::multipart::Form::new().part("media", part);
        let response = service
            .http_client()
            .post(url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        let body = response
            .text()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        // 对应 Java 执行器对 errcode!=0 抛 WxErrorException
        let error = wx_rust_common::error::WxError::from_json_with_type(
            &body,
            Some(wx_rust_common::enums::WxType::Cp),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        // WxMediaUploadResult 无 from_json 辅助，直接 serde 解析
        serde_json::from_str(&body).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
#[async_trait]
impl WxCpTpMediaService for WxCpTpMediaServiceImpl {
    async fn upload(
        &self,
        media_type: &str,
        _file_type: &str,
        file_path: &str,
        corp_id: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        // Java: getApiUrl(MEDIA_UPLOAD + mediaType) + "&access_token=" + token
        // （MEDIA_UPLOAD 含 "?type="，故追加 &）
        let url = format!(
            "{}{media_type}&access_token={}",
            config.api_url(url_media::MEDIA_UPLOAD),
            config.access_token(corp_id).unwrap_or_default()
        );
        self.upload_file(&url, file_path).await
    }

    async fn upload_img(&self, file_path: &str, corp_id: &str) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = format!(
            "{}?access_token={}",
            config.api_url(url_media::IMG_UPLOAD),
            config.access_token(corp_id).unwrap_or_default()
        );
        let result = self.upload_file(&url, file_path).await?;
        Ok(result.url)
    }
}
