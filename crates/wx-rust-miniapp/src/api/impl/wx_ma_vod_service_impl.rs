//! 小程序短剧管理（视频点播）服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaVodServiceImpl`：
//! JSON 接口逐方法对齐；`uploadSingleFile`/`uploadPart` 的 multipart 上传
//! 对应 Java `VodSingleUploadRequestExecutor`/`VodUploadPartRequestExecutor`
//! （字段名 `media_data`/`media_type`/`media_name`/`cover_type`/`cover_data`/
//! `source_context` 与 `data`/`upload_id`/`part_number`/`resource_type`
//! 照搬），Rust 侧经 `svc.http_client()` + reqwest multipart 直连
//! （参照 mp `WxMpMaterialServiceImpl` 上传模式）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, WxErrorException};

use crate::api::WxMaService;
use crate::api::g4_services::WxMaVodService;
use crate::bean::WxMaBaseResponse;
use crate::bean::vod::{
    WxMaVodApplyUploadRequest, WxMaVodApplyUploadResponse, WxMaVodAuditDramaRequest,
    WxMaVodCommitUploadRequest, WxMaVodCommitUploadResponse, WxMaVodDeleteMediaRequest,
    WxMaVodDramaInfo, WxMaVodGetCdnLogRequest, WxMaVodGetCdnLogResponse, WxMaVodGetCdnUsageRequest,
    WxMaVodGetCdnUsageResponse, WxMaVodGetDramaRequest, WxMaVodGetMediaLinkRequest,
    WxMaVodGetMediaRequest, WxMaVodGetTaskRequest, WxMaVodGetTaskResponse, WxMaVodListDramaRequest,
    WxMaVodListMediaRequest, WxMaVodMediaInfo, WxMaVodMediaPlaybackInfo, WxMaVodPullUploadRequest,
    WxMaVodPullUploadResponse, WxMaVodSingleFileUploadResult, WxMaVodUploadPartResult,
};
use crate::config::DEFAULT_API_HOST_URL;
use crate::enums::g4_urls::url_g4_ability::vod as vod_url;

/// 小程序短剧管理服务实现。
pub struct WxMaVodServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaVodServiceImpl {
    /// 构建短剧管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 序列化请求对象为 JSON（对应 Java `request.toJson()`）。
    fn to_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(request).map_err(WxErrorException::from)
    }

    /// 构建带 access_token 的上传 URL（对应 Java 执行引擎的 token 注入 +
    /// 自定义域名替换）。
    async fn build_upload_url(
        svc: &dyn WxMaService,
        url: &str,
    ) -> Result<String, WxErrorException> {
        let config = svc.wx_ma_config();
        let access_token = svc.get_access_token().await?;
        let effective_host = config.effective_api_host_url();
        let url = if effective_host != DEFAULT_API_HOST_URL {
            url.replace(DEFAULT_API_HOST_URL, &effective_host)
        } else {
            url.to_string()
        };
        Ok(format!("{url}?access_token={access_token}"))
    }

    /// 校验 multipart 上传响应 errcode 并返回原始文本。
    async fn send_multipart(
        svc: &dyn WxMaService,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<String, WxErrorException> {
        let upload_url = Self::build_upload_url(svc, url).await?;
        let text = svc
            .http_client()
            .post(&upload_url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?
            .text()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?;
        let error = WxError::from_json_with_type(&text, Some(WxType::MiniApp));
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(text)
    }
}

#[async_trait]
impl WxMaVodService for WxMaVodServiceImpl {
    /// 获取媒体列表（对应 Java `WxMaVodServiceImpl.listMedia`）。
    ///
    /// 响应无 `media_info_list` 字段时返回 `None`（Java 返回 null）。
    async fn list_media(
        &self,
        request: &WxMaVodListMediaRequest,
    ) -> Result<Option<Vec<WxMaVodMediaInfo>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::list_media_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if json.get("media_info_list").is_some() {
            serde_json::from_value(json.get("media_info_list").cloned().unwrap_or_default())
                .map_err(WxErrorException::from)
        } else {
            Ok(None)
        }
    }

    /// 获取剧集列表（对应 Java `WxMaVodServiceImpl.listDrama`）。
    ///
    /// 响应无 `drama_info_list` 字段时返回 `None`（Java 返回 null）。
    async fn list_drama(
        &self,
        request: &WxMaVodListDramaRequest,
    ) -> Result<Option<Vec<WxMaVodDramaInfo>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::list_dramas_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if json.get("drama_info_list").is_some() {
            serde_json::from_value(json.get("drama_info_list").cloned().unwrap_or_default())
                .map_err(WxErrorException::from)
        } else {
            Ok(None)
        }
    }

    /// 获取媒体播放链接（对应 Java `WxMaVodServiceImpl.getMediaLink`）。
    async fn get_media_link(
        &self,
        request: &WxMaVodGetMediaLinkRequest,
    ) -> Result<WxMaVodMediaPlaybackInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::get_media_link_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: crate::bean::vod::WxMaVodGetMediaLinkResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail.media_info)
    }

    /// 获取媒体详情（对应 Java `WxMaVodServiceImpl.getMedia`）。
    async fn get_media(
        &self,
        request: &WxMaVodGetMediaRequest,
    ) -> Result<WxMaVodMediaInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::get_media_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: crate::bean::vod::WxMaVodGetMediaResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail.media_info)
    }

    /// 删除媒体文件（对应 Java `WxMaVodServiceImpl.deleteMedia`）。
    async fn delete_media(
        &self,
        request: &WxMaVodDeleteMediaRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::delete_media_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: WxMaBaseResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(true)
    }

    /// 获取剧集详情（对应 Java `WxMaVodServiceImpl.getDrama`）。
    async fn get_drama(
        &self,
        request: &WxMaVodGetDramaRequest,
    ) -> Result<WxMaVodDramaInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::get_drama_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: crate::bean::vod::WxMaVodGetDramaResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail.drama_info)
    }

    /// 审核剧集（对应 Java `WxMaVodServiceImpl.auditDrama`，返回审核任务 ID）。
    async fn audit_drama(
        &self,
        request: &WxMaVodAuditDramaRequest,
    ) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::audit_drama_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: crate::bean::vod::WxMaVodAuditDramaResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail.drama_id)
    }

    /// 获取 CDN 用量数据（对应 Java `WxMaVodServiceImpl.getCdnUsageData`）。
    async fn get_cdn_usage_data(
        &self,
        request: &WxMaVodGetCdnUsageRequest,
    ) -> Result<WxMaVodGetCdnUsageResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::get_cdn_usage_data_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: WxMaVodGetCdnUsageResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail)
    }

    /// 获取 CDN 日志（对应 Java `WxMaVodServiceImpl.getCdnLogs`）。
    async fn get_cdn_logs(
        &self,
        request: &WxMaVodGetCdnLogRequest,
    ) -> Result<WxMaVodGetCdnLogResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::get_cdn_logs_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: WxMaVodGetCdnLogResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail)
    }

    /// 获取任务状态（对应 Java `WxMaVodServiceImpl.getTask`）。
    async fn get_task(
        &self,
        request: &WxMaVodGetTaskRequest,
    ) -> Result<WxMaVodGetTaskResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::get_task_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: WxMaVodGetTaskResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail)
    }

    /// 拉取上传（对应 Java `WxMaVodServiceImpl.pullUpload`）。
    async fn pull_upload(
        &self,
        request: &WxMaVodPullUploadRequest,
    ) -> Result<WxMaVodPullUploadResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::pull_upload_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: WxMaVodPullUploadResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail)
    }

    /// 单文件上传（简化版，对应 Java
    /// `WxMaVodServiceImpl.uploadSingleFile(File, String, String)`）。
    async fn upload_single_file(
        &self,
        file_path: &str,
        media_name: &str,
        media_type: &str,
    ) -> Result<WxMaVodSingleFileUploadResult, WxErrorException> {
        self.upload_single_file_full(file_path, media_name, media_type, None, None, None)
            .await
    }

    /// 单文件上传（完整版，对应 Java
    /// `WxMaVodServiceImpl.uploadSingleFile(File, String, String, String, File, String)`）。
    ///
    /// multipart 字段照搬 `OkHttpVodSingleUploadRequestExecutor`：
    /// `media_data`（文件字节）/`media_type`/`media_name`，可选
    /// `cover_type`/`cover_data`/`source_context`。
    async fn upload_single_file_full(
        &self,
        file_path: &str,
        media_name: &str,
        media_type: &str,
        cover_type: Option<&str>,
        cover_data_path: Option<&str>,
        source_context: Option<&str>,
    ) -> Result<WxMaVodSingleFileUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let media_data = std::fs::read(file_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let mut form = reqwest::multipart::Form::new()
            .part(
                "media_data",
                reqwest::multipart::Part::bytes(media_data)
                    .file_name(file_name.unwrap_or_default()),
            )
            .text("media_type", media_type.to_string())
            .text("media_name", media_name.to_string());
        if let Some(cover_type) = cover_type {
            form = form.text("cover_type", cover_type.to_string());
        }
        if let Some(cover_data_path) = cover_data_path {
            let cover_name = std::path::Path::new(cover_data_path)
                .file_name()
                .map(|s| s.to_string_lossy().to_string());
            let cover_bytes = std::fs::read(cover_data_path)
                .map_err(|e| WxErrorException::from_code(-99, format!("读取封面文件失败: {e}")))?;
            form = form.part(
                "cover_data",
                reqwest::multipart::Part::bytes(cover_bytes)
                    .file_name(cover_name.unwrap_or_default()),
            );
        }
        if let Some(source_context) = source_context {
            form = form.text("source_context", source_context.to_string());
        }
        let config = svc.wx_ma_config();
        let response_content = Self::send_multipart(
            svc.as_ref(),
            &vod_url::single_file_upload_url(config.as_ref()),
            form,
        )
        .await?;
        WxMaVodSingleFileUploadResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    /// 申请上传（对应 Java `WxMaVodServiceImpl.applyUpload`）。
    async fn apply_upload(
        &self,
        request: &WxMaVodApplyUploadRequest,
    ) -> Result<WxMaVodApplyUploadResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::apply_upload_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: WxMaVodApplyUploadResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail)
    }

    /// 确认上传（对应 Java `WxMaVodServiceImpl.commitUpload`）。
    async fn commit_upload(
        &self,
        request: &WxMaVodCommitUploadRequest,
    ) -> Result<WxMaVodCommitUploadResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &vod_url::commit_upload_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let detail: WxMaVodCommitUploadResponse =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(detail)
    }

    /// 上传分片（对应 Java `WxMaVodServiceImpl.uploadPart`）。
    ///
    /// multipart 字段照搬 `OkHttpVodUploadPartRequestExecutor`：
    /// `data`（文件字节）/`upload_id`/`part_number`/`resource_type`。
    async fn upload_part(
        &self,
        file_path: &str,
        upload_id: &str,
        part_number: i32,
        resource_type: i32,
    ) -> Result<WxMaVodUploadPartResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let data = std::fs::read(file_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let form = reqwest::multipart::Form::new()
            .part(
                "data",
                reqwest::multipart::Part::bytes(data).file_name(file_name.unwrap_or_default()),
            )
            .text("upload_id", upload_id.to_string())
            .text("part_number", part_number.to_string())
            .text("resource_type", resource_type.to_string());
        let config = svc.wx_ma_config();
        let response_content = Self::send_multipart(
            svc.as_ref(),
            &vod_url::upload_part_url(config.as_ref()),
            form,
        )
        .await?;
        WxMaVodUploadPartResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }
}
