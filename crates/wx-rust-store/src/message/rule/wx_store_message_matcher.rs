//! 消息匹配器，用在消息路由的时候。
//!
//! 对应 Java `com.binarywang.wxjava.store.message.rule.WxStoreMessageMatcher`。
//! Java 方法名 `match`（Rust 关键字，命名为 `match_message`，与 mp/miniapp
//! 的 `WxMpMessageMatcher`/`WxMaMessageMatcher` 一致）。

use crate::message::WxStoreMessage;

/// 消息匹配器（对应 Java `WxStoreMessageMatcher`）。
pub trait WxStoreMessageMatcher: Send + Sync {
    /// 消息是否匹配某种模式。
    fn match_message(&self, message: &WxStoreMessage) -> bool;
}
