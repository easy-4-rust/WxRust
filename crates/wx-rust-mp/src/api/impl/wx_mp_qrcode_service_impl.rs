//! 二维码服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpQrcodeServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpQrcodeService, WxMpService};
use crate::bean::result::WxMpQrCodeTicket;
use crate::enums::wx_mp_api_url::qrcode as qrcode_url;

/// 二维码服务实现。
pub struct WxMpQrcodeServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpQrcodeServiceImpl {
    /// 构建二维码服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpQrcodeService for WxMpQrcodeServiceImpl {
    async fn qrcode_create_ticket(
        &self,
        action_name: &str,
        scene_str: &str,
    ) -> Result<WxMpQrCodeTicket, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = qrcode_url::qrcode_create(config.as_ref());
        // 微信扫码接口：临时/永久二维码的场景值结构
        let body = serde_json::json!({
            "action_name": action_name,
            "action_info": {
                "scene": {
                    "scene_str": scene_str
                }
            }
        });
        let response = svc.post(&url, &body.to_string()).await?;
        WxMpQrCodeTicket::from_json(&response).map_err(WxErrorException::Serde)
    }
}
