//! 视频号小店消息子系统。
//!
//! 对应 Java `me.chanjar.weixin.channel.message` 包（Wave 2 H2c 迁移）：
//! 消息基类 [`WxChannelMessage`]、消息路由器 [`WxChannelMessageRouter`]、
//! 路由规则 [`WxChannelMessageRouterRule`] 及 rule 子包的
//! handler/interceptor/matcher 接口。消息数据类在 [`crate::bean::message`]。
//!
//! 设计说明（与 miniapp 的 `WxMaMessageRouter` 同一模式）：
//! - Java 规则列表 `List<WxChannelMessageRouterRule<? extends WxChannelMessage>>`
//!   以类型擦除 trait（[`WxChannelMessageRouterRuleErased`]）承载异构泛型规则；
//! - 异步规则对应 Java 线程池 `executorService.submit`，Rust 以
//!   `tokio::spawn` 表达（入口 `route` 为 async，ADAPTED）；
//! - 重复消息检查器复用 common 的 `WxMessageDuplicateChecker`
//!   （默认 `WxMessageInMemoryDuplicateCheckerSingleton`）。

pub mod rule;
pub mod wx_channel_message;
pub mod wx_channel_message_router;
pub mod wx_channel_message_router_rule;

use std::collections::HashMap;

pub use wx_channel_message::WxChannelMessage;
pub use wx_channel_message_router::WxChannelMessageRouter;
pub use wx_channel_message_router_rule::{
    WxChannelMessageLike, WxChannelMessageRouterRule, WxChannelMessageRouterRuleErased,
};

/// 路由上下文类型（对应 Java `Map<String, Object>`，handler/interceptor 之间
/// 传递信息用）。
pub type RouteContext = HashMap<String, Box<dyn std::any::Any + Send>>;
