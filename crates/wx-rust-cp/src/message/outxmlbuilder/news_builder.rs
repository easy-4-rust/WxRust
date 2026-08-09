//! 图文被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder.NewsBuilder`。

use crate::bean::message::WxCpXmlOutNewsMessage;
use crate::bean::message::wx_cp_xml_out_news_message::Item;
use crate::message::outxmlbuilder::BaseBuilder;

/// 图文消息 builder。
#[derive(Debug, Clone, Default)]
pub struct NewsBuilder {
    base: BaseBuilder,
    articles: Vec<Item>,
}

impl NewsBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加图文（对应 Java `addArticle(Item...)`）。
    pub fn add_article(mut self, item: Item) -> Self {
        self.articles.push(item);
        self
    }

    /// 批量设置图文列表。
    pub fn articles(mut self, articles: Vec<Item>) -> Self {
        self.articles = articles;
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

    /// 构建消息（msgType 固定为 news）。
    pub fn build(self) -> WxCpXmlOutNewsMessage {
        let mut m = WxCpXmlOutNewsMessage::new();
        self.base.set_common(&mut m.base);
        for item in self.articles {
            m.add_article(item);
        }
        m
    }
}
