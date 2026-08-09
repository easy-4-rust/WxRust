//! 消息推送服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpMessageServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpMessageService, WxCpService};
use crate::bean::{
    WxCpLinkedCorpMessage, WxCpLinkedCorpMessageSendResult, WxCpMessage, WxCpMessageSendResult,
    WxCpMessageSendStatistics, WxCpSchoolContactMessage, WxCpSchoolContactMessageSendResult,
};
use crate::enums::url_message::*;

/// 消息推送服务实现。
pub struct WxCpMessageServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpMessageServiceImpl {
    /// 构建消息推送服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpMessageService for WxCpMessageServiceImpl {
    async fn send(&self, message: &WxCpMessage) -> Result<WxCpMessageSendResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `send`：`agentId == null` 时用配置里的 agentId 回填
        // （Java 直接改入参；Rust 入参不可变，克隆后回填，ADAPTED）
        let message = fill_agent_id(svc.as_ref(), message, message.agent_id);
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(MESSAGE_SEND), &message.to_json())
            .await?;
        WxCpMessageSendResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn get_statistics(
        &self,
        time_type: i32,
    ) -> Result<WxCpMessageSendStatistics, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getStatistics`：POST `GET_STATISTICS` `{"time_type":...}`
        let body = serde_json::json!({ "time_type": time_type }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc.post(&config.api_url(GET_STATISTICS), &body).await?;
        WxCpMessageSendStatistics::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn send_linked_corp_message(
        &self,
        message: &WxCpLinkedCorpMessage,
    ) -> Result<WxCpLinkedCorpMessageSendResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendLinkedCorpMessage`：`agentId == null` 时回填配置
        let message = fill_agent_id(svc.as_ref(), message, message.agent_id);
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(LINKEDCORP_MESSAGE_SEND), &message.to_json())
            .await?;
        WxCpLinkedCorpMessageSendResult::from_json(&response_content)
            .map_err(WxErrorException::Serde)
    }

    async fn send_school_contact_message(
        &self,
        message: &WxCpSchoolContactMessage,
    ) -> Result<WxCpSchoolContactMessageSendResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendSchoolContactMessage`：`agentId == null` 时回填配置
        let message = fill_agent_id(svc.as_ref(), message, message.agent_id);
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(
                &config.api_url(EXTERNAL_CONTACT_MESSAGE_SEND),
                &message.to_json(),
            )
            .await?;
        WxCpSchoolContactMessageSendResult::from_json(&response_content)
            .map_err(WxErrorException::Serde)
    }

    async fn recall(&self, msg_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `recall`：POST `MESSAGE_RECALL` `{"msgid":...}`
        let body = serde_json::json!({ "msgid": msg_id }).to_string();
        let config = svc.wx_cp_config_storage();
        svc.post(&config.api_url(MESSAGE_RECALL), &body).await?;
        Ok(())
    }
}

/// 消息 agentId 为空时回填配置里的 agentId（对应 Java
/// `message.setAgentId(configStorage.getAgentId())`；Java 修改入参对象，
/// Rust 克隆后回填，ADAPTED）。
fn fill_agent_id<M>(svc: &dyn WxCpService, message: &M, agent_id: Option<i32>) -> M
where
    M: Clone + AgentIdSetter,
{
    if agent_id.is_some() {
        return message.clone();
    }
    let mut message = message.clone();
    message.set_agent_id(svc.wx_cp_config_storage().agent_id());
    message
}

/// 具备可写 `agent_id` 字段的消息（Rust 内私有 trait 约束，避免三份
/// 重复克隆逻辑）。
trait AgentIdSetter {
    fn set_agent_id(&mut self, agent_id: Option<i32>);
}

impl AgentIdSetter for WxCpMessage {
    fn set_agent_id(&mut self, agent_id: Option<i32>) {
        self.agent_id = agent_id;
    }
}

impl AgentIdSetter for WxCpLinkedCorpMessage {
    fn set_agent_id(&mut self, agent_id: Option<i32>) {
        self.agent_id = agent_id;
    }
}

impl AgentIdSetter for WxCpSchoolContactMessage {
    fn set_agent_id(&mut self, agent_id: Option<i32>) {
        self.agent_id = agent_id;
    }
}
