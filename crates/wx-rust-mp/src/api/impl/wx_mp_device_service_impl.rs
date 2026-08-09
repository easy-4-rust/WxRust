//! WxMpDeviceService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpDeviceServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpDeviceService, WxMpService};

use crate::bean::device::{
    TransMsgResp, WxDeviceAuthorize, WxDeviceAuthorizeResult, WxDeviceBind,
    WxDeviceBindDeviceResult, WxDeviceBindResult, WxDeviceMsg, WxDeviceOpenIdResult,
    WxDeviceQrCodeResult,
};
use crate::enums::wx_mp_api_url::device;

/// WxMpDevice服务实现。
pub struct WxMpDeviceServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpDeviceServiceImpl {
    /// 构建 WxMpDevice服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpDeviceService for WxMpDeviceServiceImpl {
    async fn trans_msg(&self, msg: &WxDeviceMsg) -> Result<TransMsgResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(msg).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&device::trans_msg(config.as_ref()), &body).await?;
        TransMsgResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_qr_code(
        &self,
        product_id: &str,
    ) -> Result<WxDeviceQrCodeResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let query = format!("product_id={product_id}");
        let response = svc
            .get(&device::get_qrcode(config.as_ref()), &query)
            .await?;
        WxDeviceQrCodeResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn authorize(
        &self,
        authorize: &WxDeviceAuthorize,
    ) -> Result<WxDeviceAuthorizeResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(authorize).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&device::authorize(config.as_ref()), &body).await?;
        WxDeviceAuthorizeResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn bind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(bind).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&device::bind(config.as_ref()), &body).await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn compel_bind(
        &self,
        bind: &WxDeviceBind,
    ) -> Result<WxDeviceBindResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(bind).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&device::compel_bind(config.as_ref()), &body)
            .await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn unbind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(bind).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&device::unbind(config.as_ref()), &body).await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn compel_unbind(
        &self,
        bind: &WxDeviceBind,
    ) -> Result<WxDeviceBindResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(bind).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&device::compel_unbind(config.as_ref()), &body)
            .await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_open_id(
        &self,
        device_type: &str,
        device_id: &str,
    ) -> Result<WxDeviceOpenIdResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let query = format!("device_type={device_type}&device_id={device_id}");
        let response = svc
            .get(&device::get_openid(config.as_ref()), &query)
            .await?;
        WxDeviceOpenIdResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_bind_device(
        &self,
        open_id: &str,
    ) -> Result<WxDeviceBindDeviceResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let query = format!("openid={open_id}");
        let response = svc
            .get(&device::get_bind_device(config.as_ref()), &query)
            .await?;
        WxDeviceBindDeviceResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}
