//! 模板消息服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpTemplateMsgServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpTemplateMsgService};
use crate::bean::template::WxMpTemplateMessage;
use crate::enums::wx_mp_api_url::template_msg as template_url;

/// 模板消息服务实现。
pub struct WxMpTemplateMsgServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpTemplateMsgServiceImpl {
    /// 构建模板消息服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpTemplateMsgService for WxMpTemplateMsgServiceImpl {
    async fn send_template_msg(
        &self,
        message: &WxMpTemplateMessage,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = template_url::message_template_send(config.as_ref());
        let body = message.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&url, &body).await
    }
}
