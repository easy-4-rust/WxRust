//! 文本被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder.TextBuilder`。

use crate::bean::message::WxCpXmlOutTextMessage;
use crate::message::outxmlbuilder::BaseBuilder;

/// 文本消息 builder。
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
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.base = self.base.to_user(to_user);
        self
    }

    /// 设置开发者微信号。
    pub fn from_user(mut self, from_user: impl Into<String>) -> Self {
        self.base = self.base.from_user(from_user);
        self
    }

    /// 构建消息（msgType 固定为 text）。
    pub fn build(self) -> WxCpXmlOutTextMessage {
        let mut m = WxCpXmlOutTextMessage::new();
        self.base.set_common(&mut m.base);
        m.content = self.content;
        m
    }
}
