//! 素材（媒体）服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpMediaService`。

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::error::WxErrorException;

use crate::bean::{MediaUploadByUrlReq, MediaUploadByUrlResult};

/// 素材（媒体）服务。
#[async_trait]
pub trait WxCpMediaService: Send + Sync {
    /// 上传多媒体文件（对应 Java
    /// `WxCpMediaService.upload(String, String, InputStream)`；
    /// Java `InputStream` 以 `Vec<u8>` 表达，ADAPTED）。
    async fn upload(
        &self,
        media_type: &str,
        file_type: &str,
        input: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 上传多媒体文件（远程链接版，对应 Java
    /// `WxCpMediaService.upload(String, String, String)`，第三参为 url）。
    async fn upload_with_url(
        &self,
        media_type: &str,
        filename: &str,
        url: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 上传多媒体文件（对应 Java
    /// `WxCpMediaService.upload(String, File, String)`；Java `File`
    /// 以文件路径 `&str` 表达，ADAPTED）。
    async fn upload_with_file_path(
        &self,
        media_type: &str,
        file_path: &str,
        filename: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 上传多媒体文件（对应 Java
    /// `WxCpMediaService.upload(String, InputStream, String)`；
    /// Java `InputStream` 以 `Vec<u8>` 表达，ADAPTED）。
    async fn upload_with_stream(
        &self,
        media_type: &str,
        input: Vec<u8>,
        filename: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 上传多媒体文件（对应 Java `WxCpMediaService.upload(String, File)`；
    /// Java `File` 以文件路径 `&str` 表达，ADAPTED）。
    async fn upload_with_file(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 下载多媒体文件（对应 Java `WxCpMediaService.download(String)`；
    /// Java 返回本地临时 `File`，Rust 返回文件字节 `Vec<u8>`，ADAPTED；
    /// 视频文件下载不了，会返回空）。
    async fn download(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException>;

    /// 获取高清语音素材（对应 Java `WxCpMediaService.getJssdkFile(String)`；
    /// Java 返回本地临时 `File`，Rust 返回文件字节 `Vec<u8>`，ADAPTED）。
    async fn get_jssdk_file(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException>;

    /// 上传图片得到图片 URL（对应 Java
    /// `WxCpMediaService.uploadImg(File)`；Java `File` 以文件路径
    /// `&str` 表达，ADAPTED）。
    async fn upload_img(&self, file_path: &str) -> Result<String, WxErrorException>;

    /// 生成异步上传任务（对应 Java
    /// `WxCpMediaService.uploadByUrl(MediaUploadByUrlReq)`，返回异步任务 id）。
    async fn upload_by_url(&self, req: &MediaUploadByUrlReq) -> Result<String, WxErrorException>;

    /// 查询异步任务结果（对应 Java
    /// `WxCpMediaService.uploadByUrl(String)`，入参为 jobId）。
    async fn upload_by_url_result(
        &self,
        job_id: &str,
    ) -> Result<MediaUploadByUrlResult, WxErrorException>;
}
