//! 消息路由规则子包。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.rule` 包：
//! [`WxChannelMessageHandler`]（处理器）、[`WxChannelMessageInterceptor`]
//! （拦截器）、[`WxChannelMessageMatcher`]（匹配器）、[`HandlerConsumer`]
//! （处理器消费者函数别名）。

pub mod handler_consumer;
pub mod wx_channel_message_handler;
pub mod wx_channel_message_interceptor;
pub mod wx_channel_message_matcher;

pub use handler_consumer::HandlerConsumer;
pub use wx_channel_message_handler::{WxChannelMessageHandler, WxChannelMessageHandlerFn};
pub use wx_channel_message_interceptor::WxChannelMessageInterceptor;
pub use wx_channel_message_matcher::WxChannelMessageMatcher;
