//! 对应 Java `com.github.binarywang.wxpay.service.MarketingMediaService`。
//!
//! 微信支付营销媒体接口（v3 图片上传）。Java 两个 `imageUploadV3` 重载
//! （`File` / `InputStream` 变体）在 Rust 合并为 `(file_name, file_data)`
//! 单一签名（`ADAPTED`，同 [`crate::api::MerchantMediaService`]）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// 微信支付营销媒体服务（对应 Java `MarketingMediaService`）。
#[async_trait]
pub trait MarketingMediaService: Send + Sync {
    /// 微信支付营销媒体图片上传（对应 Java `imageUploadV3(File)` /
    /// `imageUploadV3(InputStream, String)`，接口地址 `/v3/marketing/favor/media/image-upload`）。
    ///
    /// `ADAPTED`：Java `File`/`InputStream` 重载合并为 `(文件名, 文件字节)`。
    async fn image_upload_v3(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<MarketingImageUploadResult, WxErrorException>;
}
