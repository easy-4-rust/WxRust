//! WxMpOcrService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpOcrServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpOcrService, WxMpService};
use crate::enums::wx_mp_api_url::ocr as ocr_url;
use wx_rust_common::bean::ocr::{
    WxOcrBankCardResult, WxOcrBizLicenseResult, WxOcrCommResult, WxOcrDrivingLicenseResult,
    WxOcrDrivingResult, WxOcrIdCardResult,
};

/// 公众号OcrService实现。
pub struct WxMpOcrServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpOcrServiceImpl {
    /// 构建 公众号OcrService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 图片 URL OCR 查询（对应 Java `img_url` 表单参数）。
    async fn post_img(
        svc: &dyn WxMpService,
        url: &str,
        img_url: &str,
    ) -> Result<String, WxErrorException> {
        let body = serde_json::json!({"img_url": img_url});
        svc.post(url, &body.to_string()).await
    }
}

#[async_trait]
impl WxMpOcrService for WxMpOcrServiceImpl {
    async fn id_card(&self, img_url: &str) -> Result<WxOcrIdCardResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response =
            Self::post_img(svc.as_ref(), &ocr_url::id_card(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn bank_card(&self, img_url: &str) -> Result<WxOcrBankCardResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response =
            Self::post_img(svc.as_ref(), &ocr_url::bank_card(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn driving(&self, img_url: &str) -> Result<WxOcrDrivingResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response =
            Self::post_img(svc.as_ref(), &ocr_url::driving(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn driving_license(
        &self,
        img_url: &str,
    ) -> Result<WxOcrDrivingLicenseResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = Self::post_img(
            svc.as_ref(),
            &ocr_url::driving_license(config.as_ref()),
            img_url,
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn biz_license(&self, img_url: &str) -> Result<WxOcrBizLicenseResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = Self::post_img(
            svc.as_ref(),
            &ocr_url::biz_license(config.as_ref()),
            img_url,
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn comm(&self, img_url: &str) -> Result<WxOcrCommResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response =
            Self::post_img(svc.as_ref(), &ocr_url::comm(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
