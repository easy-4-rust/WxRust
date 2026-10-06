//! 微信消息拦截器，可以用来做验证。
//!
//! 对应 Java `com.binarywang.wxjava.store.message.rule.WxStoreMessageInterceptor`：
//! `boolean intercept(WxStoreMessage, String, Map, WxStoreService,
//! WxSessionManager) throws WxErrorException`。
//!
//! Java 拦截器入参带 `WxStoreService`；Rust 以 `Option<&dyn WxStoreService>`
//! 表达（测试/无服务场景为 `None`）。错误以 `Result` 上抛，由路由器按异常
//! 处理器语义处理。

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxStoreService;
use crate::message::{RouteContext, WxStoreMessage};

/// 微信消息拦截器（对应 Java `WxStoreMessageInterceptor`）。
pub trait WxStoreMessageInterceptor: Send + Sync {
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
        message: &WxStoreMessage,
        content: &str,
        context: &mut RouteContext,
        service: Option<&dyn WxStoreService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<bool, WxErrorException>;
}
