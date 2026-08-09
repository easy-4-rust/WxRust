//! 图文消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.NewsBuilder`。

use crate::bean::message::{WxMpXmlOutNewsMessage, WxMpXmlOutNewsMessageItem};
use crate::builder::outxml::BaseBuilder;
use crate::builder::outxml::base_builder::set_common;

/// 图文消息构建器。
#[derive(Debug, Clone, Default)]
pub struct NewsBuilder {
    base: BaseBuilder,
    articles: Vec<WxMpXmlOutNewsMessageItem>,
}

impl NewsBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加图文条目。
    pub fn add_article(mut self, item: WxMpXmlOutNewsMessageItem) -> Self {
        self.articles.push(item);
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

    /// 构建图文消息。
    pub fn build(self) -> WxMpXmlOutNewsMessage {
        let mut m = WxMpXmlOutNewsMessage::new();
        set_common(&mut m.base, self.base);
        m.articles = self.articles;
        m
    }
}
