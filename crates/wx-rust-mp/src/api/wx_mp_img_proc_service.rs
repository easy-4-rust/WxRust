//! WxMpImgProcService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpImgProcService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use wx_rust_common::bean::imgproc::{
    WxImgProcAiCropResult, WxImgProcQrCodeResult, WxImgProcSuperResolutionResult,
};

/// 公众号ImgProcService。
#[async_trait]
pub trait WxMpImgProcService: Send + Sync {
    async fn qr_code(&self, img_url: &str) -> Result<WxImgProcQrCodeResult, WxErrorException>;

    async fn super_resolution(
        &self,
        img_url: &str,
    ) -> Result<WxImgProcSuperResolutionResult, WxErrorException>;

    async fn ai_crop(
        &self,
        img_url: &str,
        ratios: Option<&str>,
    ) -> Result<WxImgProcAiCropResult, WxErrorException>;
}
