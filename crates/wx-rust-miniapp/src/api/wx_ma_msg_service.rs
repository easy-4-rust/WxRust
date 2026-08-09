//! 消息发送接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaMsgService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxMaSubscribeMessage, WxMaUniformMessage, WxMaUpdatableMsg};
use crate::message::WxMaKefuMessage;

/// 消息发送接口。
#[async_trait]
pub trait WxMaMsgService: Send + Sync {
    /// 发送客服消息（对应 Java `sendKefuMsg(WxMaKefuMessage)`）。
    ///
    /// 发送成功即返回 `true`（Java `responseContent != null`）。
    async fn send_kefu_msg(&self, message: &WxMaKefuMessage) -> Result<bool, WxErrorException>;

    /// 发送订阅消息（对应 Java `sendSubscribeMsg(WxMaSubscribeMessage)`）。
    async fn send_subscribe_msg(
        &self,
        subscribe_message: &WxMaSubscribeMessage,
    ) -> Result<(), WxErrorException>;

    /// 下发小程序和公众号统一的服务消息（对应 Java
    /// `sendUniformMsg(WxMaUniformMessage)`）。
    async fn send_uniform_msg(
        &self,
        uniform_message: &WxMaUniformMessage,
    ) -> Result<(), WxErrorException>;

    /// 创建被分享动态消息的 activity_id（对应 Java
    /// `createUpdatableMessageActivityId()`，返回完整响应 JSON 对象）。
    async fn create_updatable_message_activity_id(
        &self,
    ) -> Result<serde_json::Value, WxErrorException>;

    /// 修改被分享的动态消息（对应 Java `setUpdatableMsg(WxMaUpdatableMsg)`）。
    async fn set_updatable_msg(&self, msg: &WxMaUpdatableMsg) -> Result<(), WxErrorException>;
}
