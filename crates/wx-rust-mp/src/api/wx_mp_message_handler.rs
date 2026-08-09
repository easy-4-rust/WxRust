//! 微信消息处理器。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMessageHandler`。

use std::collections::HashMap;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxMpService;
use crate::bean::message::{WxMpXmlMessage, WxMpXmlOutMessage};

/// 微信消息处理器。
///
/// Java 抛 `WxErrorException`；Rust 以 `Result` 表达同一错误路径。
pub trait WxMpMessageHandler: Send + Sync {
    /// 处理消息，返回回复消息（可为空）。
    ///
    /// # 参数
    /// - `wx_message`：微信推送的消息
    /// - `context`：上下文（规则间共享）
    /// - `wx_mp_service`：公众号服务
    /// - `session_manager`：会话管理器
    fn handle(
        &self,
        wx_message: &WxMpXmlMessage,
        context: &mut HashMap<String, Box<dyn std::any::Any + Send>>,
        wx_mp_service: Option<&dyn WxMpService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxMpXmlOutMessage>, WxErrorException>;
}
