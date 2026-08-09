//! 对应 Java `com.github.binarywang.wxpay.service.MerchantMediaService`。
//!
//! 微信支付通用媒体接口（v3 媒体文件上传）。Java 两个 `imageUploadV3` /
//! `videoUploadV3` 重载（`File` / `InputStream` 变体）在 Rust 合并为
//! `(file_name, file_data)` 单一签名：文件名 + 文件字节（`ADAPTED`，
//! `File`/`InputStream` 两种来源统一由调用方读出字节）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// 微信支付通用媒体服务（对应 Java `MerchantMediaService`）。
#[async_trait]
pub trait MerchantMediaService: Send + Sync {
    /// 通用接口-图片上传API（对应 Java `imageUploadV3(File)` /
    /// `imageUploadV3(InputStream, String)`，接口地址 `/v3/merchant/media/upload`）。
    ///
    /// `ADAPTED`：Java `File`/`InputStream` 重载合并为 `(文件名, 文件字节)`。
    async fn image_upload_v3(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<ImageUploadResult, WxErrorException>;

    /// 通用接口-视频上传API（对应 Java `videoUploadV3(File)` /
    /// `videoUploadV3(InputStream, String)`，接口地址 `/v3/merchant/media/video_upload`）。
    ///
    /// `ADAPTED`：同图片上传。
    async fn video_upload_v3(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<VideoUploadResult, WxErrorException>;
}
