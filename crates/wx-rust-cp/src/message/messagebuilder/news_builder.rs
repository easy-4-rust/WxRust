//! 图文消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.NewsBuilder`
//! （msgType 固定为 `news`）。

use crate::bean::article::NewArticle;
use crate::bean::message::WxCpMessage;
use crate::message::messagebuilder::BaseBuilder;

/// 图文消息 builder。
#[derive(Debug, Clone, Default)]
pub struct NewsBuilder {
    base: BaseBuilder,
    articles: Vec<NewArticle>,
}

impl NewsBuilder {
    /// 构建空 builder（msgType 固定为 news）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("news".to_string()),
                ..Default::default()
            },
            articles: Vec::new(),
        }
    }

    /// 添加图文（对应 Java `addArticle(NewArticle...)`）。
    pub fn add_article(mut self, article: NewArticle) -> Self {
        self.articles.push(article);
        self
    }

    /// 批量设置图文列表（对应 Java `articles(List<NewArticle>)`）。
    pub fn articles(mut self, articles: Vec<NewArticle>) -> Self {
        self.articles = articles;
        self
    }

    /// 设置企业应用的 id。
    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.base = self.base.agent_id(agent_id);
        self
    }

    /// 设置接收消息的成员。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.base = self.base.to_user(to_user);
        self
    }

    /// 设置接收消息的部门。
    pub fn to_party(mut self, to_party: impl Into<String>) -> Self {
        self.base = self.base.to_party(to_party);
        self
    }

    /// 设置接收消息的标签。
    pub fn to_tag(mut self, to_tag: impl Into<String>) -> Self {
        self.base = self.base.to_tag(to_tag);
        self
    }

    /// 构建消息。
    pub fn build(self) -> WxCpMessage {
        let mut m = self.base.build_base();
        m.articles = self.articles;
        m
    }
}
