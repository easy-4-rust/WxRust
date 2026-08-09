//! 小程序消息处理器。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaMessageHandler`：
//! 处理小程序推送消息的处理器接口。

use std::sync::Arc;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxMaService;
use crate::message::{RouteContext, WxMaMessage, WxMaOutMessage};

/// 小程序消息处理器。
///
/// Java 接口签名 `WxMaOutMessage handle(WxMaMessage, Map, WxMaService,
/// WxSessionManager) throws WxErrorException`；Rust 以 `Result` 表达同一错误路径，
/// 返回类型为可选输出消息（Java `WxMaOutMessage` 抽象类 → Rust trait 对象，
/// 可为 XML 或 JSON 格式消息）。
pub trait WxMaMessageHandler: Send + Sync {
    /// 处理消息，返回回复消息（可为空）。
    ///
    /// # 参数
    /// - `wx_message`：微信推送的消息
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `wx_ma_service`：小程序服务
    /// - `session_manager`：会话管理器
    fn handle(
        &self,
        wx_message: &WxMaMessage,
        context: &mut RouteContext,
        wx_ma_service: Option<&dyn WxMaService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<Arc<dyn WxMaOutMessage + Send + Sync>>, WxErrorException>;
}
