//! 小程序消息匹配器。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaMessageMatcher`：
//! 消息匹配器，用在消息路由的时候。

use crate::message::WxMaMessage;

/// 小程序消息匹配器（自定义复杂匹配规则）。
///
/// Java 方法名 `match`（Rust 关键字，命名为 `match_message`，与
/// wx-rust-mp 的 `WxMpMessageMatcher::match_message` 一致）。
pub trait WxMaMessageMatcher: Send + Sync {
    /// 判断消息是否匹配某种模式。
    fn match_message(&self, message: &WxMaMessage) -> bool;
}
