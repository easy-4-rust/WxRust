//! 任务卡片管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpTaskCardServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpService, WxCpTaskCardService};
use crate::bean::TemplateCardMessage;
use crate::enums::url_task_card::*;

/// 任务卡片管理服务实现。
pub struct WxCpTaskCardServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpTaskCardServiceImpl {
    /// 构建任务卡片服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpTaskCardService for WxCpTaskCardServiceImpl {
    async fn update(
        &self,
        user_ids: &[&str],
        task_id: &str,
        replace_name: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `update`：agentId 取配置；HashMap(4) 组装
        // `userids`/`agentid`/`task_id`/`clicked_key`，POST `UPDATE_TASK_CARD`
        let config = svc.wx_cp_config_storage();
        let mut data = serde_json::Map::new();
        data.insert(
            "userids".to_string(),
            serde_json::Value::Array(
                user_ids
                    .iter()
                    .map(|v| serde_json::Value::from(*v))
                    .collect(),
            ),
        );
        data.insert(
            "agentid".to_string(),
            serde_json::Value::from(config.agent_id().unwrap_or(0)),
        );
        data.insert("task_id".to_string(), serde_json::Value::from(task_id));
        // 文档地址：https://open.work.weixin.qq.com/wwopen/devtool/interface?doc_id=16386
        data.insert(
            "clicked_key".to_string(),
            serde_json::Value::from(replace_name),
        );
        svc.post(
            &config.api_url(UPDATE_TASK_CARD),
            &serde_json::Value::Object(data).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn update_template_card_button(
        &self,
        user_ids: &[&str],
        party_ids: &[i32],
        tag_ids: &[i32],
        at_all: i32,
        response_code: &str,
        replace_name: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `updateTemplateCardButton`：agentId 取配置；HashMap(7) 组装
        // `userids`/`partyids`/`tagids`/`atall`/`agentid`/`response_code`/
        // `button{replace_name}`，POST `UPDATE_TEMPLATE_CARD`
        let config = svc.wx_cp_config_storage();
        let mut data = serde_json::Map::new();
        data.insert(
            "userids".to_string(),
            serde_json::Value::Array(
                user_ids
                    .iter()
                    .map(|v| serde_json::Value::from(*v))
                    .collect(),
            ),
        );
        data.insert(
            "partyids".to_string(),
            serde_json::Value::Array(
                party_ids
                    .iter()
                    .map(|v| serde_json::Value::from(*v))
                    .collect(),
            ),
        );
        data.insert(
            "tagids".to_string(),
            serde_json::Value::Array(
                tag_ids
                    .iter()
                    .map(|v| serde_json::Value::from(*v))
                    .collect(),
            ),
        );
        data.insert("atall".to_string(), serde_json::Value::from(at_all));
        data.insert(
            "agentid".to_string(),
            serde_json::Value::from(config.agent_id().unwrap_or(0)),
        );
        data.insert(
            "response_code".to_string(),
            serde_json::Value::from(response_code),
        );
        let mut btn_map = serde_json::Map::new();
        btn_map.insert(
            "replace_name".to_string(),
            serde_json::Value::from(replace_name),
        );
        data.insert("button".to_string(), serde_json::Value::Object(btn_map));
        svc.post(
            &config.api_url(UPDATE_TEMPLATE_CARD),
            &serde_json::Value::Object(data).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn update_template_card_button_with_message(
        &self,
        template_card_message: &TemplateCardMessage,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `updateTemplateCardButton(TemplateCardMessage)`：直接发送
        // 调用方消息体
        let config = svc.wx_cp_config_storage();
        let body = template_card_message
            .to_json()
            .map_err(WxErrorException::Serde)?;
        svc.post(&config.api_url(UPDATE_TEMPLATE_CARD), &body)
            .await?;
        Ok(())
    }
}
