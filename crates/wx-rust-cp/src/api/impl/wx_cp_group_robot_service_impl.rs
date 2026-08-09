//! 群机器人消息推送服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpGroupRobotServiceImpl`。
//! webhook 发送走 `post_without_token` 通道（webhook 自带鉴权 key，
//! 不自动带 access_token）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpGroupRobotService, WxCpService};
use crate::bean::{NewArticle, WxCpGroupRobotMessage};
use crate::constant::wx_cp_constants::group_robot_msg_type;
use crate::enums::url_core::WEBHOOK_SEND;

/// 群机器人消息推送服务实现。
pub struct WxCpGroupRobotServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpGroupRobotServiceImpl {
    /// 构建群机器人消息推送服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 组装默认 webhook 地址（对应 Java `getWebhookUrl()`：
    /// `webhookKey` 为空抛 `WxErrorException("请先设置WebhookKey")`，
    /// 否则 `apiUrl(WEBHOOK_SEND) + webhookKey`）。
    fn get_webhook_url(svc: &dyn WxCpService) -> Result<String, WxErrorException> {
        let config = svc.wx_cp_config_storage();
        let webhook_key = config.webhook_key().unwrap_or_default();
        if webhook_key.trim().is_empty() {
            return Err(WxErrorException::from_code(-99, "请先设置WebhookKey"));
        }
        Ok(format!("{}{webhook_key}", config.api_url(WEBHOOK_SEND)))
    }
}

#[async_trait]
impl WxCpGroupRobotService for WxCpGroupRobotServiceImpl {
    async fn send_text(
        &self,
        content: &str,
        mentioned_list: &[&str],
        mobile_list: &[&str],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendText`：默认 webhook
        let webhook_url = Self::get_webhook_url(svc.as_ref())?;
        self.send_text_with_webhook_url(webhook_url.as_str(), content, mentioned_list, mobile_list)
            .await
    }

    async fn send_markdown(&self, content: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        let webhook_url = Self::get_webhook_url(svc.as_ref())?;
        self.send_markdown_with_webhook_url(webhook_url.as_str(), content)
            .await
    }

    async fn send_image(&self, base64: &str, md5: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        let webhook_url = Self::get_webhook_url(svc.as_ref())?;
        self.send_image_with_webhook_url(webhook_url.as_str(), base64, md5)
            .await
    }

    async fn send_news(&self, article_list: &[NewArticle]) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        let webhook_url = Self::get_webhook_url(svc.as_ref())?;
        self.send_news_with_webhook_url(webhook_url.as_str(), article_list)
            .await
    }

    async fn send_text_with_webhook_url(
        &self,
        webhook_url: &str,
        content: &str,
        mentioned_list: &[&str],
        mobile_list: &[&str],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendText(String, String, List, List)`：组装
        // `WxCpGroupRobotMessage`（msgtype=text）后 `postWithoutToken`
        let message = WxCpGroupRobotMessage {
            msg_type: Some(group_robot_msg_type::TEXT.to_string()),
            content: Some(content.to_string()),
            mentioned_list: mentioned_list.iter().map(|s| s.to_string()).collect(),
            mentioned_mobile_list: mobile_list.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        };
        svc.post_without_token(webhook_url, &message.to_json())
            .await?;
        Ok(())
    }

    async fn send_markdown_with_webhook_url(
        &self,
        webhook_url: &str,
        content: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendMarkdown(String, String)`：msgtype=markdown
        let message = WxCpGroupRobotMessage {
            msg_type: Some(group_robot_msg_type::MARKDOWN.to_string()),
            content: Some(content.to_string()),
            ..Default::default()
        };
        svc.post_without_token(webhook_url, &message.to_json())
            .await?;
        Ok(())
    }

    async fn send_markdown_v2(&self, content: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        let webhook_url = Self::get_webhook_url(svc.as_ref())?;
        self.send_markdown_v2_with_webhook_url(webhook_url.as_str(), content)
            .await
    }

    async fn send_markdown_v2_with_webhook_url(
        &self,
        webhook_url: &str,
        content: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendMarkdownV2(String, String)`：msgtype=markdown_v2
        let message = WxCpGroupRobotMessage {
            msg_type: Some(group_robot_msg_type::MARKDOWN_V2.to_string()),
            content: Some(content.to_string()),
            ..Default::default()
        };
        svc.post_without_token(webhook_url, &message.to_json())
            .await?;
        Ok(())
    }

    async fn send_image_with_webhook_url(
        &self,
        webhook_url: &str,
        base64: &str,
        md5: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendImage(String, String, String)`：msgtype=image
        let message = WxCpGroupRobotMessage {
            msg_type: Some(group_robot_msg_type::IMAGE.to_string()),
            base64: Some(base64.to_string()),
            md5: Some(md5.to_string()),
            ..Default::default()
        };
        svc.post_without_token(webhook_url, &message.to_json())
            .await?;
        Ok(())
    }

    async fn send_news_with_webhook_url(
        &self,
        webhook_url: &str,
        article_list: &[NewArticle],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendNews(String, List)`：msgtype=news
        let message = WxCpGroupRobotMessage {
            msg_type: Some(group_robot_msg_type::NEWS.to_string()),
            articles: article_list.to_vec(),
            ..Default::default()
        };
        svc.post_without_token(webhook_url, &message.to_json())
            .await?;
        Ok(())
    }

    async fn send_file(&self, webhook_url: &str, media_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendFile`：msgtype=file
        let message = WxCpGroupRobotMessage {
            msg_type: Some(group_robot_msg_type::FILE.to_string()),
            media_id: Some(media_id.to_string()),
            ..Default::default()
        };
        svc.post_without_token(webhook_url, &message.to_json())
            .await?;
        Ok(())
    }

    async fn send_voice(&self, webhook_url: &str, media_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendVoice`：msgtype=voice
        let message = WxCpGroupRobotMessage {
            msg_type: Some(group_robot_msg_type::VOICE.to_string()),
            media_id: Some(media_id.to_string()),
            ..Default::default()
        };
        svc.post_without_token(webhook_url, &message.to_json())
            .await?;
        Ok(())
    }

    async fn send_template_card_message(
        &self,
        webhook_url: &str,
        wx_cp_group_robot_message: &WxCpGroupRobotMessage,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendTemplateCardMessage`：直接发送调用方消息体
        svc.post_without_token(webhook_url, &wx_cp_group_robot_message.to_json())
            .await?;
        Ok(())
    }
}
