//! 企业微信智能机器人服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpIntelligentRobotService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpIntelligentRobot, WxCpIntelligentRobotChatRequest, WxCpIntelligentRobotChatResponse,
    WxCpIntelligentRobotCreateRequest, WxCpIntelligentRobotCreateResponse,
    WxCpIntelligentRobotMessage, WxCpIntelligentRobotSendMessageRequest,
    WxCpIntelligentRobotSendMessageResponse, WxCpIntelligentRobotUpdateRequest,
};

/// 企业微信智能机器人服务。
#[async_trait]
pub trait WxCpIntelligentRobotService: Send + Sync {
    /// 创建智能机器人（对应 Java
    /// `WxCpIntelligentRobotService.createRobot(WxCpIntelligentRobotCreateRequest)`）。
    async fn create_robot(
        &self,
        request: &WxCpIntelligentRobotCreateRequest,
    ) -> Result<WxCpIntelligentRobotCreateResponse, WxErrorException>;

    /// 删除智能机器人（对应 Java
    /// `WxCpIntelligentRobotService.deleteRobot(String)`）。
    async fn delete_robot(&self, robot_id: &str) -> Result<(), WxErrorException>;

    /// 更新智能机器人（对应 Java
    /// `WxCpIntelligentRobotService.updateRobot(WxCpIntelligentRobotUpdateRequest)`）。
    async fn update_robot(
        &self,
        request: &WxCpIntelligentRobotUpdateRequest,
    ) -> Result<(), WxErrorException>;

    /// 查询智能机器人（对应 Java
    /// `WxCpIntelligentRobotService.getRobot(String)`）。
    async fn get_robot(&self, robot_id: &str) -> Result<WxCpIntelligentRobot, WxErrorException>;

    /// 智能机器人会话（对应 Java
    /// `WxCpIntelligentRobotService.chat(WxCpIntelligentRobotChatRequest)`）。
    async fn chat(
        &self,
        request: &WxCpIntelligentRobotChatRequest,
    ) -> Result<WxCpIntelligentRobotChatResponse, WxErrorException>;

    /// 重置智能机器人会话（对应 Java
    /// `WxCpIntelligentRobotService.resetSession(String, String, String)`）。
    async fn reset_session(
        &self,
        robot_id: &str,
        userid: &str,
        session_id: &str,
    ) -> Result<(), WxErrorException>;

    /// 智能机器人主动发送消息（对应 Java
    /// `WxCpIntelligentRobotService.sendMessage(WxCpIntelligentRobotSendMessageRequest)`）。
    async fn send_message(
        &self,
        request: &WxCpIntelligentRobotSendMessageRequest,
    ) -> Result<WxCpIntelligentRobotSendMessageResponse, WxErrorException>;

    /// 解析智能机器人 API 模式回调消息（对应 Java
    /// `WxCpIntelligentRobotService.parseCallbackMessage(String)`）。
    async fn parse_callback_message(
        &self,
        callback_message_json: &str,
    ) -> Result<WxCpIntelligentRobotMessage, WxErrorException>;
}
