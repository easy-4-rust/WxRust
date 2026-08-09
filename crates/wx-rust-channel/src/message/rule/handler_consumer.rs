//! 处理器消费者函数别名。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.rule.HandlerConsumer`
//! （函数式接口 `void accept(T, U, V, W, X)`）：消息服务默认规则把
//! `(message, content, appId, context, sessionManager)` 消费函数包装为
//! handler（对应 Java `BaseWxChannelMessageServiceImpl.addRule`）。

use std::sync::Arc;

use wx_rust_common::session::WxSessionManager;

use crate::message::RouteContext;

/// 处理器消费者（对应 Java `HandlerConsumer<T, String, String, Map, WxSessionManager>`）。
pub type HandlerConsumer<T> =
    Arc<dyn Fn(&T, &str, &str, &mut RouteContext, &dyn WxSessionManager) + Send + Sync>;
