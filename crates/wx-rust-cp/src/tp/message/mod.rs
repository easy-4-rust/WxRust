//! 企业微信第三方应用消息路由。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.message` 包：服务商推送消息的
//! 路由族——`WxCpTpMessageRouter`/`WxCpTpMessageRouterRule`/
//! `WxCpTpMessageHandler`/`WxCpTpMessageInterceptor`/
//! `WxCpTpMessageMatcher`（路由入口为 `WxCpTpXmlMessage`，与
//! `WxCpMessageRouter` 相比多了 infoType/changeType 维度）。

pub mod wx_cp_tp_message_handler;
pub mod wx_cp_tp_message_interceptor;
pub mod wx_cp_tp_message_matcher;
pub mod wx_cp_tp_message_router;
pub mod wx_cp_tp_message_router_rule;

pub use wx_cp_tp_message_handler::WxCpTpMessageHandler;
pub use wx_cp_tp_message_interceptor::WxCpTpMessageInterceptor;
pub use wx_cp_tp_message_matcher::WxCpTpMessageMatcher;
pub use wx_cp_tp_message_router::TpRuleBuilder;
pub use wx_cp_tp_message_router::WxCpTpMessageRouter;
pub use wx_cp_tp_message_router_rule::WxCpTpMessageRouterRule;
