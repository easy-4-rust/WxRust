//! 企业微信第三方应用（tp）消息处理器。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.message.WxCpTpMessageHandler`：
//! 处理服务商推送消息的处理器接口（消息类型为 `WxCpTpXmlMessage`，
//! 服务为 `WxCpTpService`）。

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::bean::message::{WxCpTpXmlMessage, WxCpXmlOutMessage};
use crate::message::RouteContext;
use crate::tp::service::WxCpTpService;

/// 企业微信第三方应用消息处理器。
///
/// Java 接口签名 `WxCpXmlOutMessage handle(WxCpTpXmlMessage, Map,
/// WxCpTpService, WxSessionManager) throws WxErrorException`；Rust 以
/// `Result` 表达同一错误路径，返回可选输出消息（`WxCpXmlOutMessage`）。
pub trait WxCpTpMessageHandler: Send + Sync {
    /// 处理消息，返回回复消息（可为空）。
    ///
    /// # 参数
    /// - `wx_message`：服务商推送的消息
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `wx_cp_tp_service`：第三方应用服务
    /// - `session_manager`：会话管理器
    fn handle(
        &self,
        wx_message: &WxCpTpXmlMessage,
        context: &mut RouteContext,
        wx_cp_tp_service: Option<&dyn WxCpTpService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxCpXmlOutMessage>, WxErrorException>;
}
