//! 企业微信第三方应用素材服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpMediaService`：
//! 上传多媒体文件/上传图片（使用授权企业的 access_token）。

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::error::WxErrorException;

/// 企业微信第三方应用素材服务。
#[async_trait]
pub trait WxCpTpMediaService: Send + Sync {
    /// 上传多媒体文件（对应 Java `upload(String, String, InputStream,
    /// String)`：mediaType 媒体类型、file_type 文件扩展名、corpId 授权
    /// 企业的 corpid；Rust 以文件路径/字节表达输入流语义，ADAPTED）。
    async fn upload(
        &self,
        media_type: &str,
        file_type: &str,
        file_path: &str,
        corp_id: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 上传图片，得到图片 URL（对应 Java `uploadImg(File, String)`，
    /// 该 URL 永久有效）。
    async fn upload_img(&self, file_path: &str, corp_id: &str) -> Result<String, WxErrorException>;
}
