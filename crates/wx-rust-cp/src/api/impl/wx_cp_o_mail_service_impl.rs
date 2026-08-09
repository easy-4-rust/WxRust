//! 企业微信企业邮箱服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOMailServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor`
//! 注入 `cpService`），三个发送方法统一走 `mailSend` 私有通道——
//! POST `{base}/cgi-bin/exmail/app/compose_send`，响应解析为
//! `WxCpBaseResp`。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOaMailService, WxCpService};
use crate::bean::{
    WxCpBaseResp, WxCpMailCommonSendRequest, WxCpMailMeetingSendRequest,
    WxCpMailScheduleSendRequest,
};
use crate::enums::url_oa;

/// 企业微信企业邮箱服务实现。
pub struct WxCpOMailServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOMailServiceImpl {
    /// 构建企业邮箱服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `cpService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpService 引用已失效"))
    }

    /// 发送邮件公共通道（对应 Java `mailSend(String)`：POST
    /// `EXMAIL_APP_COMPOSE_SEND`，响应解析为 `WxCpBaseResp`）。
    async fn mail_send(&self, request: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_config_storage();
        let api_url = config.api_url(url_oa::EXMAIL_APP_COMPOSE_SEND);
        let response_content = service.post(&api_url, request).await?;
        WxCpBaseResp::from_json(&response_content).map_err(WxErrorException::Serde)
    }
}

#[async_trait]
impl WxCpOaMailService for WxCpOMailServiceImpl {
    async fn mail_common_send(
        &self,
        request: &WxCpMailCommonSendRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let json = request.to_json().map_err(WxErrorException::Serde)?;
        self.mail_send(&json).await
    }

    async fn mail_schedule_send(
        &self,
        request: &WxCpMailScheduleSendRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let json = request.to_json().map_err(WxErrorException::Serde)?;
        self.mail_send(&json).await
    }

    async fn mail_meeting_send(
        &self,
        request: &WxCpMailMeetingSendRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let json = request.to_json().map_err(WxErrorException::Serde)?;
        self.mail_send(&json).await
    }
}
