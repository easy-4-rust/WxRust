//! 微信消息拦截器。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMessageInterceptor`。

use std::collections::HashMap;

use wx_rust_common::session::WxSessionManager;

use crate::api::WxMpService;
use crate::bean::message::WxMpXmlMessage;

/// 微信消息拦截器。
pub trait WxMpMessageInterceptor: Send + Sync {
    /// 拦截判断：返回 `false` 时中断该规则的后续处理。
    ///
    /// # 参数
    /// - `wx_message`：微信推送的消息
    /// - `context`：上下文
    /// - `wx_mp_service`：公众号服务
    /// - `session_manager`：会话管理器
    fn intercept(
        &self,
        wx_message: &WxMpXmlMessage,
        context: &mut HashMap<String, Box<dyn std::any::Any + Send>>,
        wx_mp_service: Option<&dyn WxMpService>,
        session_manager: &dyn WxSessionManager,
    ) -> bool;
}
