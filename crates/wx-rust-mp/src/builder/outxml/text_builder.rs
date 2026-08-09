//! 文本消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.TextBuilder`。

use crate::bean::message::WxMpXmlOutTextMessage;
use crate::builder::outxml::BaseBuilder;
use crate::builder::outxml::base_builder::set_common;

/// 文本消息构建器。
#[derive(Debug, Clone, Default)]
pub struct TextBuilder {
    base: BaseBuilder,
    content: Option<String>,
}

impl TextBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置消息内容。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 设置接收方帐号。
    pub fn to_user(self, to_user: impl Into<String>) -> Self {
        Self {
            base: self.base.to_user(to_user),
            ..self
        }
    }

    /// 设置开发者微信号。
    pub fn from_user(self, from_user: impl Into<String>) -> Self {
        Self {
            base: self.base.from_user(from_user),
            ..self
        }
    }

    /// 构建文本消息。
    pub fn build(self) -> WxMpXmlOutTextMessage {
        let mut m = WxMpXmlOutTextMessage::new();
        set_common(&mut m.base, self.base);
        m.content = self.content;
        m
    }
}
