//! WxMpImgProcService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpImgProcServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpImgProcService, WxMpService};
use crate::enums::wx_mp_api_url::img_proc;
use wx_rust_common::bean::imgproc::{
    WxImgProcAiCropResult, WxImgProcQrCodeResult, WxImgProcSuperResolutionResult,
};

/// 公众号ImgProcService实现。
pub struct WxMpImgProcServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpImgProcServiceImpl {
    /// 构建 公众号ImgProcService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpImgProcService for WxMpImgProcServiceImpl {
    async fn qr_code(&self, img_url: &str) -> Result<WxImgProcQrCodeResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"img_url": img_url});
        let response = svc
            .post(&img_proc::qr_code(config.as_ref()), &body.to_string())
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn super_resolution(
        &self,
        img_url: &str,
    ) -> Result<WxImgProcSuperResolutionResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"img_url": img_url});
        let response = svc
            .post(
                &img_proc::super_resolution(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn ai_crop(
        &self,
        img_url: &str,
        ratios: Option<&str>,
    ) -> Result<WxImgProcAiCropResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert("img_url".into(), serde_json::json!(img_url));
        if let Some(r) = ratios {
            body.insert("ratios".into(), serde_json::json!(r));
        }
        let response = svc
            .post(
                &img_proc::ai_crop(config.as_ref()),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
