//! 企业微信消息构建与路由。
//!
//! 对应 Java `me.chanjar.weixin.cp.message` 包。Wave 1 提供消息构建器
//! （`messagebuilder` 对应 Java `bean.messagebuilder` 子包、`outxmlbuilder`
//! 对应 Java `bean.outxmlbuilder` 子包）；Wave 2 实现消息路由
//! （`WxCpMessageRouter`/`WxCpMessageRouterRule`/`WxCpMessageHandler`/
//! `WxCpMessageInterceptor`/`WxCpMessageMatcher`），路由入口为
//! `WxCpXmlMessage`（对应 Java `route(WxCpXmlMessage)`）。

pub mod messagebuilder;
pub mod outxmlbuilder;
pub mod wx_cp_message_handler;
pub mod wx_cp_message_interceptor;
pub mod wx_cp_message_matcher;
pub mod wx_cp_message_router;
pub mod wx_cp_message_router_rule;

pub use wx_cp_message_handler::WxCpMessageHandler;
pub use wx_cp_message_interceptor::WxCpMessageInterceptor;
pub use wx_cp_message_matcher::WxCpMessageMatcher;
pub use wx_cp_message_router::RouteContext;
pub use wx_cp_message_router::RuleBuilder;
pub use wx_cp_message_router::WxCpMessageRouter;
pub use wx_cp_message_router_rule::WxCpMessageRouterRule;
