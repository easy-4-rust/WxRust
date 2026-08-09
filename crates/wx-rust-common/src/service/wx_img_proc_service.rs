//! 图片处理服务接口。
//!
//! 对应 Java `me.chanjar.weixin.common.service.WxImgProcService`。

use async_trait::async_trait;

use crate::bean::imgproc::{
    WxImgProcAiCropResult, WxImgProcQrCodeResult, WxImgProcSuperResolutionResult,
};
use crate::error::WxErrorException;

/// 图片处理服务接口（AI 抠图/二维码识别/超分）。
#[async_trait]
pub trait WxImgProcService: Send + Sync {
    /// AI 抠图。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    /// - `ratio`：原图压缩比例
    ///
    /// # 返回
    /// AI 抠图结果
    async fn ai_crop(
        &self,
        img_url: &str,
        ratio: f64,
    ) -> Result<WxImgProcAiCropResult, WxErrorException>;

    /// 二维码识别。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    ///
    /// # 返回
    /// 二维码识别结果
    async fn qrcode(&self, img_url: &str) -> Result<WxImgProcQrCodeResult, WxErrorException>;

    /// 图片高清化（超分）。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    ///
    /// # 返回
    /// 超分结果（含处理后的 mediaId）
    async fn super_resolution(
        &self,
        img_url: &str,
    ) -> Result<WxImgProcSuperResolutionResult, WxErrorException>;
}
