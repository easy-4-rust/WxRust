//! 小程序短剧管理（视频点播）服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaVodService`
//! （`impl.WxMaVodServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::vod::{
    WxMaVodApplyUploadRequest, WxMaVodApplyUploadResponse, WxMaVodAuditDramaRequest,
    WxMaVodCommitUploadRequest, WxMaVodCommitUploadResponse, WxMaVodDeleteMediaRequest,
    WxMaVodDramaInfo, WxMaVodGetCdnLogRequest, WxMaVodGetCdnLogResponse, WxMaVodGetCdnUsageRequest,
    WxMaVodGetCdnUsageResponse, WxMaVodGetDramaRequest, WxMaVodGetMediaLinkRequest,
    WxMaVodGetMediaRequest, WxMaVodGetTaskRequest, WxMaVodGetTaskResponse, WxMaVodListDramaRequest,
    WxMaVodListMediaRequest, WxMaVodMediaInfo, WxMaVodMediaPlaybackInfo, WxMaVodPullUploadRequest,
    WxMaVodPullUploadResponse, WxMaVodSingleFileUploadResult, WxMaVodUploadPartResult,
};

/// 小程序短剧管理服务。
///
/// 对应 Java `WxMaVodService`：媒体列表/详情/播放链接/删除、剧集列表/详情/
/// 审核、CDN 用量/日志、拉取上传/任务状态、单文件上传/分片上传。
#[async_trait]
pub trait WxMaVodService: Send + Sync {
    /// 获取媒体列表（对应 Java `listMedia`）。
    ///
    /// 响应无 `media_info_list` 字段时返回 `None`（Java 返回 null）。
    async fn list_media(
        &self,
        request: &WxMaVodListMediaRequest,
    ) -> Result<Option<Vec<WxMaVodMediaInfo>>, WxErrorException>;

    /// 获取剧集列表（对应 Java `listDrama`）。
    ///
    /// 响应无 `drama_info_list` 字段时返回 `None`（Java 返回 null）。
    async fn list_drama(
        &self,
        request: &WxMaVodListDramaRequest,
    ) -> Result<Option<Vec<WxMaVodDramaInfo>>, WxErrorException>;

    /// 获取媒体播放链接（对应 Java `getMediaLink`）。
    async fn get_media_link(
        &self,
        request: &WxMaVodGetMediaLinkRequest,
    ) -> Result<WxMaVodMediaPlaybackInfo, WxErrorException>;

    /// 获取媒体详情（对应 Java `getMedia`）。
    async fn get_media(
        &self,
        request: &WxMaVodGetMediaRequest,
    ) -> Result<WxMaVodMediaInfo, WxErrorException>;

    /// 删除媒体文件（对应 Java `deleteMedia`）。
    async fn delete_media(
        &self,
        request: &WxMaVodDeleteMediaRequest,
    ) -> Result<bool, WxErrorException>;

    /// 获取剧集详情（对应 Java `getDrama`）。
    async fn get_drama(
        &self,
        request: &WxMaVodGetDramaRequest,
    ) -> Result<WxMaVodDramaInfo, WxErrorException>;

    /// 审核剧集（对应 Java `auditDrama`，返回审核任务 ID）。
    async fn audit_drama(
        &self,
        request: &WxMaVodAuditDramaRequest,
    ) -> Result<i32, WxErrorException>;

    /// 获取 CDN 用量数据（对应 Java `getCdnUsageData`）。
    async fn get_cdn_usage_data(
        &self,
        request: &WxMaVodGetCdnUsageRequest,
    ) -> Result<WxMaVodGetCdnUsageResponse, WxErrorException>;

    /// 获取 CDN 日志（对应 Java `getCdnLogs`）。
    async fn get_cdn_logs(
        &self,
        request: &WxMaVodGetCdnLogRequest,
    ) -> Result<WxMaVodGetCdnLogResponse, WxErrorException>;

    /// 获取任务状态（对应 Java `getTask`）。
    async fn get_task(
        &self,
        request: &WxMaVodGetTaskRequest,
    ) -> Result<WxMaVodGetTaskResponse, WxErrorException>;

    /// 拉取上传（对应 Java `pullUpload`）。
    async fn pull_upload(
        &self,
        request: &WxMaVodPullUploadRequest,
    ) -> Result<WxMaVodPullUploadResponse, WxErrorException>;

    /// 单文件上传（简化版，对应 Java
    /// `uploadSingleFile(File, String, String)`）。
    async fn upload_single_file(
        &self,
        file_path: &str,
        media_name: &str,
        media_type: &str,
    ) -> Result<WxMaVodSingleFileUploadResult, WxErrorException>;

    /// 单文件上传（完整版，对应 Java
    /// `uploadSingleFile(File, String, String, String, File, String)`，
    /// 可携带封面与来源上下文）。
    async fn upload_single_file_full(
        &self,
        file_path: &str,
        media_name: &str,
        media_type: &str,
        cover_type: Option<&str>,
        cover_data_path: Option<&str>,
        source_context: Option<&str>,
    ) -> Result<WxMaVodSingleFileUploadResult, WxErrorException>;

    /// 申请上传（对应 Java `applyUpload`）。
    async fn apply_upload(
        &self,
        request: &WxMaVodApplyUploadRequest,
    ) -> Result<WxMaVodApplyUploadResponse, WxErrorException>;

    /// 确认上传（对应 Java `commitUpload`）。
    async fn commit_upload(
        &self,
        request: &WxMaVodCommitUploadRequest,
    ) -> Result<WxMaVodCommitUploadResponse, WxErrorException>;

    /// 上传分片（对应 Java `uploadPart`）。
    async fn upload_part(
        &self,
        file_path: &str,
        upload_id: &str,
        part_number: i32,
        resource_type: i32,
    ) -> Result<WxMaVodUploadPartResult, WxErrorException>;
}
