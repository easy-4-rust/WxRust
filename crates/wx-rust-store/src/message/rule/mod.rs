//! 消息路由规则子包。
//!
//! 对应 Java `com.binarywang.wxjava.store.message.rule` 包：
//! [`WxStoreMessageHandler`]（处理器）、[`WxStoreMessageInterceptor`]
//! （拦截器）、[`WxStoreMessageMatcher`]（匹配器）、[`HandlerConsumer`]
//! （处理器消费者函数别名）。

pub mod handler_consumer;
pub mod wx_store_message_handler;
pub mod wx_store_message_interceptor;
pub mod wx_store_message_matcher;

pub use handler_consumer::HandlerConsumer;

pub use wx_store_message_interceptor::WxStoreMessageInterceptor;
pub use wx_store_message_matcher::WxStoreMessageMatcher;

pub use wx_store_message_handler::{WxStoreMessageHandler, WxStoreMessageHandlerFn};
