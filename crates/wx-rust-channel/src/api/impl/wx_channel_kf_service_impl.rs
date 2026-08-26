//! WxChannelKfServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelKfServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_kf_service::WxChannelKfService;
use crate::bean::kf::{
    WxChannelKfCosUploadResponse, WxChannelKfSendMsgParam, WxChannelKfSendMsgResponse,
};
use crate::enums::url_kf as url;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};

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
        open_id: String,
        msg_type: String,
        file_name: String,
        file: Vec<u8>,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let data = CommonUploadData::new(Some(file_name), file);
        let mut form_fields = std::collections::HashMap::new();
        form_fields.insert("open_id".to_string(), open_id);
        form_fields.insert("msg_type".to_string(), msg_type);
        let param = CommonUploadParam::with_form_fields("file", data, form_fields);
        let response = svc.upload(url::COS_UPLOAD_URL, param).await?;
        let resp: WxChannelKfCosUploadResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(resp.cos_url)
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
