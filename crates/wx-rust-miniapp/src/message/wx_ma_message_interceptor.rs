//! 小程序消息拦截器。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaMessageInterceptor`：
//! 微信消息拦截器，可以用来做验证。

use wx_rust_common::session::WxSessionManager;

use crate::api::WxMaService;
use crate::message::{RouteContext, WxMaMessage};

/// 小程序消息拦截器。
///
/// Java 接口签名 `boolean intercept(WxMaMessage, Map, WxMaService,
/// WxSessionManager) throws WxErrorException`；拦截结果 `true` 代表放行，
/// `false` 代表不通过（不调用 handler）。
pub trait WxMaMessageInterceptor: Send + Sync {
    /// 拦截判断：返回 `false` 时中断该规则的后续处理。
    ///
    /// # 参数
    /// - `wx_message`：微信推送的消息
    /// - `context`：上下文（handler/interceptor 之间传递信息用）
    /// - `wx_ma_service`：小程序服务
    /// - `session_manager`：会话管理器
    fn intercept(
        &self,
        wx_message: &WxMaMessage,
        context: &mut RouteContext,
        wx_ma_service: Option<&dyn WxMaService>,
        session_manager: &dyn WxSessionManager,
    ) -> bool;
}
