//! 微信消息匹配器。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMessageMatcher`。

use crate::bean::message::WxMpXmlMessage;

/// 微信消息匹配器（自定义复杂匹配规则）。
pub trait WxMpMessageMatcher: Send + Sync {
    /// 判断消息是否匹配。
    fn match_message(&self, message: &WxMpXmlMessage) -> bool;
}
