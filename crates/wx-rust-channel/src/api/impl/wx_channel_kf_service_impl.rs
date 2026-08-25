//! WxChannelKfServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelKfServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_kf_service::WxChannelKfService;
use crate::bean::kf::{WxChannelKfSendMsgParam, WxChannelKfSendMsgResponse};
use crate::enums::url_kf as url;

/// 商家客服服务实现。
pub struct WxChannelKfServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelKfServiceImpl {
    /// 构建商家客服服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelKfService for WxChannelKfServiceImpl {
    async fn upload_media(
        &self,
        _open_id: String,
        _msg_type: String,
        _file_name: String,
        _file: Vec<u8>,
    ) -> Result<String, WxErrorException> {
        // TODO: 实现文件上传逻辑
        Err(WxErrorException::from_code(-99, "文件上传暂未实现"))
    }

    async fn send_message(
        &self,
        param: WxChannelKfSendMsgParam,
    ) -> Result<WxChannelKfSendMsgResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SEND_MSG_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
