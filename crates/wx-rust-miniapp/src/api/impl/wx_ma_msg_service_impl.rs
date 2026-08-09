//! 消息服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaMsgServiceImpl`。
//! 各方法委托门面 `WxMaService`（门面已实现同一 URL/payload/响应解析，
//! 与 Java 委托 `service.post(...)` 同一语义）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMaMsgService, WxMaService};
use crate::bean::{WxMaSubscribeMessage, WxMaUniformMessage, WxMaUpdatableMsg};
use crate::message::WxMaKefuMessage;

/// 消息服务实现。
pub struct WxMaMsgServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaMsgServiceImpl {
    /// 构建消息服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaMsgService for WxMaMsgServiceImpl {
    async fn send_kefu_msg(&self, message: &WxMaKefuMessage) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `sendKefuMsg`：POST `/cgi-bin/message/custom/send`（`Msg.KEFU_MESSAGE_SEND_URL`），
        // 响应非 null 即返回 true
        svc.send_kefu_msg(message).await
    }

    async fn send_subscribe_msg(
        &self,
        subscribe_message: &WxMaSubscribeMessage,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `sendSubscribeMsg`：POST `/cgi-bin/message/subscribe/send`
        // （`Msg.SUBSCRIBE_MSG_SEND_URL`），显式 errcode 校验已被执行引擎覆盖（同一语义）
        svc.send_subscribe_msg(subscribe_message).await
    }

    async fn send_uniform_msg(
        &self,
        uniform_message: &WxMaUniformMessage,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `sendUniformMsg`：POST `/cgi-bin/message/wxopen/template/uniform_send`
        // （`Msg.UNIFORM_MSG_SEND_URL`）
        svc.send_uniform_msg(uniform_message).await
    }

    async fn create_updatable_message_activity_id(
        &self,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `createUpdatableMessageActivityId`：GET
        // `/cgi-bin/message/wxopen/activityid/create`（`Msg.ACTIVITY_ID_CREATE_URL`），
        // 返回 `GsonParser.parse` 完整响应 JSON 对象
        svc.create_updatable_message_activity_id().await
    }

    async fn set_updatable_msg(&self, msg: &WxMaUpdatableMsg) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `setUpdatableMsg`：POST `/cgi-bin/message/wxopen/updatablemsg/send`
        // （`Msg.UPDATABLE_MSG_SEND_URL`）
        svc.set_updatable_msg(msg).await
    }
}
