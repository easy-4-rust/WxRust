//! 企业微信第三方应用消息推送服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpMessageServiceImpl`：
//! 代授权企业发送应用消息，所有方法均传入授权企业的 corpId 并以
//! `config.getAccessToken(corpId)` 拼接 access_token（不带 suite token）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::message::{
    WxCpLinkedCorpMessage, WxCpLinkedCorpMessageSendResult, WxCpMessage, WxCpMessageSendResult,
    WxCpMessageSendStatistics, WxCpSchoolContactMessage, WxCpSchoolContactMessageSendResult,
};
use crate::enums::url_message;
use crate::tp::service::{WxCpTpMessageService, WxCpTpService};

/// 企业微信第三方应用消息推送服务实现。
pub struct WxCpTpMessageServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpMessageServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 拼接带授权企业 access_token 的 URL（对应 Java 各方法内
    /// `getApiUrl(path) + "?access_token=" + getAccessToken(corpId)`）。
    fn url_with_corp_token(
        &self,
        service: &dyn WxCpTpService,
        corp_id: &str,
        path: &str,
    ) -> String {
        let config = service.wx_cp_tp_config_storage();
        format!(
            "{}?access_token={}",
            config.api_url(path),
            config.access_token(corp_id).unwrap_or_default()
        )
    }
}

#[async_trait]
impl WxCpTpMessageService for WxCpTpMessageServiceImpl {
    async fn send(
        &self,
        message: &WxCpMessage,
        corp_id: &str,
    ) -> Result<WxCpMessageSendResult, WxErrorException> {
        let service = self.service()?;
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_message::MESSAGE_SEND);
        let response = service
            .post_without_suite_token(&url, &message.to_json(), true)
            .await?;
        WxCpMessageSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_statistics(
        &self,
        time_type: i32,
        corp_id: &str,
    ) -> Result<WxCpMessageSendStatistics, WxErrorException> {
        let service = self.service()?;
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_message::GET_STATISTICS);
        let body = serde_json::json!({ "time_type": time_type }).to_string();
        let response = service.post_without_suite_token(&url, &body, true).await?;
        WxCpMessageSendStatistics::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn send_linked_corp_message(
        &self,
        message: &WxCpLinkedCorpMessage,
        corp_id: &str,
    ) -> Result<WxCpLinkedCorpMessageSendResult, WxErrorException> {
        let service = self.service()?;
        let url = self.url_with_corp_token(
            service.as_ref(),
            corp_id,
            url_message::LINKEDCORP_MESSAGE_SEND,
        );
        let json = message.to_json();
        let response = service.post_without_suite_token(&url, &json, true).await?;
        WxCpLinkedCorpMessageSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn send_school_contact_message(
        &self,
        message: &WxCpSchoolContactMessage,
        corp_id: &str,
    ) -> Result<WxCpSchoolContactMessageSendResult, WxErrorException> {
        let service = self.service()?;
        let url = self.url_with_corp_token(
            service.as_ref(),
            corp_id,
            url_message::EXTERNAL_CONTACT_MESSAGE_SEND,
        );
        let json = message.to_json();
        let response = service.post_without_suite_token(&url, &json, true).await?;
        WxCpSchoolContactMessageSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn recall(&self, msg_id: &str, corp_id: &str) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({ "msgid": msg_id }).to_string();
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_message::MESSAGE_RECALL);
        service.post_without_suite_token(&url, &body, true).await?;
        Ok(())
    }
}
