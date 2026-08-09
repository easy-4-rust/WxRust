//! WxMpDevice服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpDeviceService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::device::{
    TransMsgResp, WxDeviceAuthorize, WxDeviceAuthorizeResult, WxDeviceBind,
    WxDeviceBindDeviceResult, WxDeviceBindResult, WxDeviceMsg, WxDeviceOpenIdResult,
    WxDeviceQrCodeResult,
};

/// WxMpDevice服务。
#[async_trait]
pub trait WxMpDeviceService: Send + Sync {
    async fn trans_msg(&self, msg: &WxDeviceMsg) -> Result<TransMsgResp, WxErrorException>;

    async fn get_qr_code(&self, product_id: &str)
    -> Result<WxDeviceQrCodeResult, WxErrorException>;

    async fn authorize(
        &self,
        authorize: &WxDeviceAuthorize,
    ) -> Result<WxDeviceAuthorizeResult, WxErrorException>;

    async fn bind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException>;

    async fn compel_bind(
        &self,
        bind: &WxDeviceBind,
    ) -> Result<WxDeviceBindResult, WxErrorException>;

    async fn unbind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException>;

    async fn compel_unbind(
        &self,
        bind: &WxDeviceBind,
    ) -> Result<WxDeviceBindResult, WxErrorException>;

    async fn get_open_id(
        &self,
        device_type: &str,
        device_id: &str,
    ) -> Result<WxDeviceOpenIdResult, WxErrorException>;

    async fn get_bind_device(
        &self,
        open_id: &str,
    ) -> Result<WxDeviceBindDeviceResult, WxErrorException>;
}
