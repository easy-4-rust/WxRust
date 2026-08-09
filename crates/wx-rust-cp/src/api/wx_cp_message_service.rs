//! 消息推送服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpMessageService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpLinkedCorpMessage, WxCpLinkedCorpMessageSendResult, WxCpMessage, WxCpMessageSendResult,
    WxCpMessageSendStatistics, WxCpSchoolContactMessage, WxCpSchoolContactMessageSendResult,
};

/// 消息推送服务。
#[async_trait]
pub trait WxCpMessageService: Send + Sync {
    /// 发送消息（对应 Java `WxCpMessageService.send(WxCpMessage)`）。
    async fn send(&self, message: &WxCpMessage) -> Result<WxCpMessageSendResult, WxErrorException>;

    /// 查询应用消息发送统计（对应 Java
    /// `WxCpMessageService.getStatistics(int)`；`timeType`：0-当天，1-昨天）。
    async fn get_statistics(
        &self,
        time_type: i32,
    ) -> Result<WxCpMessageSendStatistics, WxErrorException>;

    /// 发送互联企业消息（对应 Java
    /// `WxCpMessageService.sendLinkedCorpMessage(WxCpLinkedCorpMessage)`）。
    async fn send_linked_corp_message(
        &self,
        message: &WxCpLinkedCorpMessage,
    ) -> Result<WxCpLinkedCorpMessageSendResult, WxErrorException>;

    /// 发送「学校通知」（对应 Java
    /// `WxCpMessageService.sendSchoolContactMessage(WxCpSchoolContactMessage)`）。
    async fn send_school_contact_message(
        &self,
        message: &WxCpSchoolContactMessage,
    ) -> Result<WxCpSchoolContactMessageSendResult, WxErrorException>;

    /// 撤回应用消息（对应 Java `WxCpMessageService.recall(String)`）。
    async fn recall(&self, msg_id: &str) -> Result<(), WxErrorException>;
}
