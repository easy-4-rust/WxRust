//! 企业微信消息拦截器。
//!
//! 对应 Java `me.chanjar.weixin.cp.message.WxCpMessageInterceptor`：
//! 微信消息拦截器，可以用来做验证。

use wx_rust_common::session::WxSessionManager;

use crate::api::WxCpService;
use crate::bean::message::WxCpXmlMessage;
use crate::message::RouteContext;

/// 企业微信消息拦截器。
///
/// Java 接口签名 `boolean intercept(WxCpXmlMessage, Map, WxCpService,
/// WxSessionManager) throws WxErrorException`；拦截结果 `true` 代表放行，
/// `false` 代表不通过（不调用 handler，对应 Java 提前返回 null）。
pub trait WxCpMessageInterceptor: Send + Sync {
    /// 拦截判断：返回 `false` 时中断该规则的后续处理。
    ///
    /// # 参数
    /// - `wx_message`：微信推送的消息
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `wx_cp_service`：企业微信服务
    /// - `session_manager`：会话管理器
    fn intercept(
        &self,
        wx_message: &WxCpXmlMessage,
        context: &mut RouteContext,
        wx_cp_service: Option<&dyn WxCpService>,
        session_manager: &dyn WxSessionManager,
    ) -> bool;
}
