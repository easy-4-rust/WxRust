//! 企业微信第三方应用（tp）消息拦截器。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.message.WxCpTpMessageInterceptor`：
//! 服务商消息拦截器，可以用来做验证。

use wx_rust_common::session::WxSessionManager;

use crate::bean::message::WxCpTpXmlMessage;
use crate::message::RouteContext;
use crate::tp::service::WxCpTpService;

/// 企业微信第三方应用消息拦截器。
///
/// Java 接口签名 `boolean intercept(WxCpTpXmlMessage, Map, WxCpTpService,
/// WxSessionManager) throws WxErrorException`；拦截结果 `true` 代表放行，
/// `false` 代表不通过（不调用 handler，对应 Java 提前返回 null）。
pub trait WxCpTpMessageInterceptor: Send + Sync {
    /// 拦截判断：返回 `false` 时中断该规则的后续处理。
    ///
    /// # 参数
    /// - `wx_message`：服务商推送的消息
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `wx_cp_tp_service`：第三方应用服务
    /// - `session_manager`：会话管理器
    fn intercept(
        &self,
        wx_message: &WxCpTpXmlMessage,
        context: &mut RouteContext,
        wx_cp_tp_service: Option<&dyn WxCpTpService>,
        session_manager: &dyn WxSessionManager,
    ) -> bool;
}
