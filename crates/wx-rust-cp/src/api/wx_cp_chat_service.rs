//! 群聊服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpChatService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpAppChatMessage, WxCpChat};

/// 群聊服务。
#[async_trait]
pub trait WxCpChatService: Send + Sync {
    /// 创建群聊会话（对应 Java
    /// `WxCpChatService.create(String, String, List<String>, String)`；
    /// 返回创建的群聊会话 chatId；`chatId` 不填则系统随机生成）。
    async fn create(
        &self,
        name: &str,
        owner: &str,
        users: &[&str],
        chat_id: Option<&str>,
    ) -> Result<String, WxErrorException>;

    /// 修改群聊会话（对应 Java
    /// `WxCpChatService.update(String, String, String, List<String>,
    /// List<String>)`）。
    async fn update(
        &self,
        chat_id: &str,
        name: Option<&str>,
        owner: Option<&str>,
        users_to_add: &[&str],
        users_to_delete: &[&str],
    ) -> Result<(), WxErrorException>;

    /// 获取群聊会话（对应 Java `WxCpChatService.get(String)`，
    /// 响应 `chat_info` 子对象解析）。
    async fn get(&self, chat_id: &str) -> Result<WxCpChat, WxErrorException>;

    /// 群聊会话消息推送（对应 Java
    /// `WxCpChatService.sendMsg(WxCpAppChatMessage)`）。
    async fn send_msg(&self, message: &WxCpAppChatMessage) -> Result<(), WxErrorException>;
}
