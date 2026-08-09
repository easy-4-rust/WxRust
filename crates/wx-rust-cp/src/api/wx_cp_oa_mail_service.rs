//! 企业微信企业邮箱服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaMailService`：应用可通过
//! 该接口发送普通邮件/日程邮件/会议邮件（支持附件能力），统一走
//! `POST /cgi-bin/exmail/app/compose_send`（文档：
//! https://developer.work.weixin.qq.com/document/path/95486）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpBaseResp, WxCpMailCommonSendRequest, WxCpMailMeetingSendRequest,
    WxCpMailScheduleSendRequest,
};

/// 企业微信企业邮箱服务。
#[async_trait]
pub trait WxCpOaMailService: Send + Sync {
    /// 发送普通邮件（对应 Java `mailCommonSend(WxCpMailCommonSendRequest)`）。
    async fn mail_common_send(
        &self,
        request: &WxCpMailCommonSendRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 发送日程邮件（对应 Java `mailScheduleSend(WxCpMailScheduleSendRequest)`）。
    async fn mail_schedule_send(
        &self,
        request: &WxCpMailScheduleSendRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 发送会议邮件（对应 Java `mailMeetingSend(WxCpMailMeetingSendRequest)`）。
    async fn mail_meeting_send(
        &self,
        request: &WxCpMailMeetingSendRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;
}
