//! OCR 服务接口。
//!
//! 对应 Java `me.chanjar.weixin.common.service.WxOcrService`。

use async_trait::async_trait;

use crate::bean::ocr::{
    WxOcrBankCardResult, WxOcrBizLicenseResult, WxOcrCommResult, WxOcrDrivingLicenseResult,
    WxOcrDrivingResult, WxOcrIdCardResult,
};
use crate::error::WxErrorException;

/// OCR 识别服务接口。
///
/// 通用 OCR（印刷体/驾驶证/行驶证/银行卡/营业执照/身份证）。
#[async_trait]
pub trait WxOcrService: Send + Sync {
    /// 通用印刷体 OCR 识别。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    async fn ocr_comm(&self, img_url: &str) -> Result<WxOcrCommResult, WxErrorException>;

    /// 驾驶证 OCR 识别。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    async fn ocr_driving_license(
        &self,
        img_url: &str,
    ) -> Result<WxOcrDrivingLicenseResult, WxErrorException>;

    /// 行驶证 OCR 识别。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    async fn ocr_driving(&self, img_url: &str) -> Result<WxOcrDrivingResult, WxErrorException>;

    /// 银行卡 OCR 识别。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    async fn ocr_bank_card(&self, img_url: &str) -> Result<WxOcrBankCardResult, WxErrorException>;

    /// 营业执照 OCR 识别。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    async fn ocr_biz_license(
        &self,
        img_url: &str,
    ) -> Result<WxOcrBizLicenseResult, WxErrorException>;

    /// 身份证 OCR 识别。
    ///
    /// # 参数
    /// - `img_url`：图片 URL
    async fn ocr_id_card(&self, img_url: &str) -> Result<WxOcrIdCardResult, WxErrorException>;
}
