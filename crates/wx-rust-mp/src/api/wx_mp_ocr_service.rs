//! WxMpOcrService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpOcrService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::enums::wx_mp_api_url::ocr as ocr_url;
use wx_rust_common::bean::ocr::{
    WxOcrBankCardResult, WxOcrBizLicenseResult, WxOcrCommResult, WxOcrDrivingLicenseResult,
    WxOcrDrivingResult, WxOcrIdCardResult,
};

/// 公众号OcrService。
#[async_trait]
pub trait WxMpOcrService: Send + Sync {
    async fn id_card(&self, img_url: &str) -> Result<WxOcrIdCardResult, WxErrorException>;

    async fn bank_card(&self, img_url: &str) -> Result<WxOcrBankCardResult, WxErrorException>;

    async fn driving(&self, img_url: &str) -> Result<WxOcrDrivingResult, WxErrorException>;

    async fn driving_license(
        &self,
        img_url: &str,
    ) -> Result<WxOcrDrivingLicenseResult, WxErrorException>;

    async fn biz_license(&self, img_url: &str) -> Result<WxOcrBizLicenseResult, WxErrorException>;

    async fn comm(&self, img_url: &str) -> Result<WxOcrCommResult, WxErrorException>;
}
