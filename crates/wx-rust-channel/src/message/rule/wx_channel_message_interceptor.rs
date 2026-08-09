//! 微信消息拦截器，可以用来做验证。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.rule.WxChannelMessageInterceptor`：
//! `boolean intercept(WxChannelMessage, String, Map, WxChannelService,
//! WxSessionManager) throws WxErrorException`。
//!
//! Java 拦截器入参带 `WxChannelService`；Rust 以 `Option<&dyn WxChannelService>`
//! 表达（测试/无服务场景为 `None`）。错误以 `Result` 上抛，由路由器按异常
//! 处理器语义处理。

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxChannelService;
use crate::message::{RouteContext, WxChannelMessage};

/// 微信消息拦截器（对应 Java `WxChannelMessageInterceptor`）。
pub trait WxChannelMessageInterceptor: Send + Sync {
    /// 拦截判断：返回 `Ok(true)` 放行，`Ok(false)` 中断该规则的后续处理。
    ///
    /// # 参数
    /// - `message`：原始消息（未重新反序列化）
    /// - `content`：消息原始内容
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `service`：服务实例（可为空）
    /// - `session_manager`：会话管理器
    fn intercept(
        &self,
        message: &WxChannelMessage,
        content: &str,
        context: &mut RouteContext,
        service: Option<&dyn WxChannelService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<bool, WxErrorException>;
}
