//! 企业微信消息匹配器。
//!
//! 对应 Java `me.chanjar.weixin.cp.message.WxCpMessageMatcher`：
//! 消息匹配器，用在消息路由的时候（自定义复杂匹配规则）。

use crate::bean::message::WxCpXmlMessage;

/// 企业微信消息匹配器（自定义复杂匹配规则）。
///
/// Java 方法名 `match`（Rust 关键字，命名为 `match_message`，与
/// wx-rust-mp 的 `WxMpMessageMatcher::match_message` 一致）。
pub trait WxCpMessageMatcher: Send + Sync {
    /// 判断消息是否匹配某种模式。
    fn match_message(&self, message: &WxCpXmlMessage) -> bool;
}
