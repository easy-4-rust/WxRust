//! 处理视频号推送消息的处理器。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.rule.WxChannelMessageHandler`：
//! `Object handle(T message, String content, String appId, Map context,
//! WxSessionManager) throws WxErrorException`。
//!
//! Java 返回 `Object`（本子系统无输出消息类型，实践中为 `"success"` 或 null）；
//! Rust 以 `Option<String>` 表达（ADAPTED）。错误以 `Result` 上抛，由路由器按
//! 异常处理器语义处理。

use std::sync::Arc;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::message::RouteContext;

/// 处理视频号推送消息的处理器（对应 Java `WxChannelMessageHandler<T>`）。
pub trait WxChannelMessageHandler<T>: Send + Sync {
    /// 处理消息。
    ///
    /// # 参数
    /// - `message`：重新反序列化后的类型化消息
    /// - `content`：消息原始内容
    /// - `app_id`：appId
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `session_manager`：会话管理器
    ///
    /// # 返回
    /// 输出消息（`"success"` 或 null；对应 Java 返回 `Object`）
    fn handle(
        &self,
        message: &T,
        content: &str,
        app_id: &str,
        context: &mut RouteContext,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<String>, WxErrorException>;
}

/// 闭包处理器适配器（Rust 适配，无对应 Java 类）。
///
/// 对应 Java 中 `(message, content, appId, context, sessionManager) -> {...}`
/// 的 lambda 处理器。
pub struct WxChannelMessageHandlerFn<T> {
    f: Arc<
        dyn Fn(
                &T,
                &str,
                &str,
                &mut RouteContext,
                &dyn WxSessionManager,
            ) -> Result<Option<String>, WxErrorException>
            + Send
            + Sync,
    >,
}

impl<T> WxChannelMessageHandlerFn<T> {
    /// 由闭包构建处理器。
    pub fn new(
        f: impl Fn(
            &T,
            &str,
            &str,
            &mut RouteContext,
            &dyn WxSessionManager,
        ) -> Result<Option<String>, WxErrorException>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self { f: Arc::new(f) }
    }
}

impl<T> WxChannelMessageHandler<T> for WxChannelMessageHandlerFn<T> {
    fn handle(
        &self,
        message: &T,
        content: &str,
        app_id: &str,
        context: &mut RouteContext,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<String>, WxErrorException> {
        (self.f)(message, content, app_id, context, session_manager)
    }
}
