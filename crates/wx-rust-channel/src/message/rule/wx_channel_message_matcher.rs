//! 消息匹配器，用在消息路由的时候。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.rule.WxChannelMessageMatcher`。
//! Java 方法名 `match`（Rust 关键字，命名为 `match_message`，与 mp/miniapp
//! 的 `WxMpMessageMatcher`/`WxMaMessageMatcher` 一致）。

use crate::message::WxChannelMessage;

/// 消息匹配器（对应 Java `WxChannelMessageMatcher`）。
pub trait WxChannelMessageMatcher: Send + Sync {
    /// 消息是否匹配某种模式。
    fn match_message(&self, message: &WxChannelMessage) -> bool;
}
