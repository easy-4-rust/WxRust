//! 企业微信第三方应用消息推送服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpMessageService`：
//! 第三方应用使用授权企业的 access_token 代表授权企业发送应用消息。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::message::{
    WxCpLinkedCorpMessage, WxCpLinkedCorpMessageSendResult, WxCpMessage, WxCpMessageSendResult,
    WxCpMessageSendStatistics, WxCpSchoolContactMessage, WxCpSchoolContactMessageSendResult,
};

/// 企业微信第三方应用消息推送服务。
#[async_trait]
pub trait WxCpTpMessageService: Send + Sync {
    /// 发送应用消息（代授权企业发送，对应 Java `send(WxCpMessage, String)`）。
    async fn send(
        &self,
        message: &WxCpMessage,
        corp_id: &str,
    ) -> Result<WxCpMessageSendResult, WxErrorException>;

    /// 查询应用消息发送统计（对应 Java `getStatistics(int, String)`：
    /// timeType 0 当天，1 昨天）。
    async fn get_statistics(
        &self,
        time_type: i32,
        corp_id: &str,
    ) -> Result<WxCpMessageSendStatistics, WxErrorException>;

    /// 互联企业发送应用消息（对应 Java
    /// `sendLinkedCorpMessage(WxCpLinkedCorpMessage, String)`）。
    async fn send_linked_corp_message(
        &self,
        message: &WxCpLinkedCorpMessage,
        corp_id: &str,
    ) -> Result<WxCpLinkedCorpMessageSendResult, WxErrorException>;

    /// 发送「学校通知」（对应 Java
    /// `sendSchoolContactMessage(WxCpSchoolContactMessage, String)`）。
    async fn send_school_contact_message(
        &self,
        message: &WxCpSchoolContactMessage,
        corp_id: &str,
    ) -> Result<WxCpSchoolContactMessageSendResult, WxErrorException>;

    /// 撤回应用消息（对应 Java `recall(String, String)`）。
    async fn recall(&self, msg_id: &str, corp_id: &str) -> Result<(), WxErrorException>;
}
