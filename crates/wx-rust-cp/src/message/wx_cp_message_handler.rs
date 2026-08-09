//! 企业微信消息处理器。
//!
//! 对应 Java `me.chanjar.weixin.cp.message.WxCpMessageHandler`：
//! 处理微信推送消息的处理器接口。

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxCpService;
use crate::bean::message::{WxCpXmlMessage, WxCpXmlOutMessage};
use crate::message::RouteContext;

/// 企业微信消息处理器。
///
/// Java 接口签名 `WxCpXmlOutMessage handle(WxCpXmlMessage, Map,
/// WxCpService, WxSessionManager) throws WxErrorException`；Rust 以
/// `Result` 表达同一错误路径，返回可选输出消息（`WxCpXmlOutMessage`）。
pub trait WxCpMessageHandler: Send + Sync {
    /// 处理消息，返回回复消息（可为空）。
    ///
    /// # 参数
    /// - `wx_message`：微信推送的消息
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `wx_cp_service`：企业微信服务
    /// - `session_manager`：会话管理器
    fn handle(
        &self,
        wx_message: &WxCpXmlMessage,
        context: &mut RouteContext,
        wx_cp_service: Option<&dyn WxCpService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxCpXmlOutMessage>, WxErrorException>;
}
