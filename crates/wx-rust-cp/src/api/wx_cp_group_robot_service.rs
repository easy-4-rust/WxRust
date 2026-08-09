//! 群机器人消息推送服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpGroupRobotService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{NewArticle, WxCpGroupRobotMessage};

/// 群机器人消息推送服务。
#[async_trait]
pub trait WxCpGroupRobotService: Send + Sync {
    /// 发送 text 类型的消息（对应 Java
    /// `WxCpGroupRobotService.sendText(String, List<String>, List<String>)`）。
    async fn send_text(
        &self,
        content: &str,
        mentioned_list: &[&str],
        mobile_list: &[&str],
    ) -> Result<(), WxErrorException>;

    /// 发送 markdown 类型的消息（对应 Java
    /// `WxCpGroupRobotService.sendMarkdown(String)`）。
    async fn send_markdown(&self, content: &str) -> Result<(), WxErrorException>;

    /// 发送 image 类型的消息（对应 Java
    /// `WxCpGroupRobotService.sendImage(String, String)`）。
    async fn send_image(&self, base64: &str, md5: &str) -> Result<(), WxErrorException>;

    /// 发送 news 类型的消息（对应 Java
    /// `WxCpGroupRobotService.sendNews(List<NewArticle>)`）。
    async fn send_news(&self, article_list: &[NewArticle]) -> Result<(), WxErrorException>;

    /// 发送 text 类型的消息（指定 webhook，对应 Java
    /// `WxCpGroupRobotService.sendText(String, String, List<String>,
    /// List<String>)`）。
    async fn send_text_with_webhook_url(
        &self,
        webhook_url: &str,
        content: &str,
        mentioned_list: &[&str],
        mobile_list: &[&str],
    ) -> Result<(), WxErrorException>;

    /// 发送 markdown 类型的消息（指定 webhook，对应 Java
    /// `WxCpGroupRobotService.sendMarkdown(String, String)`）。
    async fn send_markdown_with_webhook_url(
        &self,
        webhook_url: &str,
        content: &str,
    ) -> Result<(), WxErrorException>;

    /// 发送 markdown_v2 类型的消息（对应 Java
    /// `WxCpGroupRobotService.sendMarkdownV2(String)`）。
    async fn send_markdown_v2(&self, content: &str) -> Result<(), WxErrorException>;

    /// 发送 markdown_v2 类型的消息（指定 webhook，对应 Java
    /// `WxCpGroupRobotService.sendMarkdownV2(String, String)`）。
    async fn send_markdown_v2_with_webhook_url(
        &self,
        webhook_url: &str,
        content: &str,
    ) -> Result<(), WxErrorException>;

    /// 发送 image 类型的消息（指定 webhook，对应 Java
    /// `WxCpGroupRobotService.sendImage(String, String, String)`）。
    async fn send_image_with_webhook_url(
        &self,
        webhook_url: &str,
        base64: &str,
        md5: &str,
    ) -> Result<(), WxErrorException>;

    /// 发送 news 类型的消息（指定 webhook，对应 Java
    /// `WxCpGroupRobotService.sendNews(String, List<NewArticle>)`）。
    async fn send_news_with_webhook_url(
        &self,
        webhook_url: &str,
        article_list: &[NewArticle],
    ) -> Result<(), WxErrorException>;

    /// 发送文件类型的消息（对应 Java
    /// `WxCpGroupRobotService.sendFile(String, String)`）。
    async fn send_file(&self, webhook_url: &str, media_id: &str) -> Result<(), WxErrorException>;

    /// 发送语音文件消息（对应 Java
    /// `WxCpGroupRobotService.sendVoice(String, String)`）。
    async fn send_voice(&self, webhook_url: &str, media_id: &str) -> Result<(), WxErrorException>;

    /// 发送模板卡片消息（对应 Java
    /// `WxCpGroupRobotService.sendTemplateCardMessage(String,
    /// WxCpGroupRobotMessage)`）。
    async fn send_template_card_message(
        &self,
        webhook_url: &str,
        wx_cp_group_robot_message: &WxCpGroupRobotMessage,
    ) -> Result<(), WxErrorException>;
}
