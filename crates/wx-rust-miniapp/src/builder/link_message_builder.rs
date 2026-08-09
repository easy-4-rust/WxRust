//! 图文链接消息 builder。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.builder.LinkMessageBuilder`。

use crate::builder::base_builder::BaseBuilder;
use crate::message::{KfLink, WxMaKefuMessage};

/// 图文链接消息构建器。
#[derive(Debug, Clone, Default)]
pub struct LinkMessageBuilder {
    base: BaseBuilder,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    thumb_url: Option<String>,
}

impl LinkMessageBuilder {
    /// 构建空 builder（消息类型 `link`）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("link".to_string()),
                ..Default::default()
            },
            title: None,
            description: None,
            url: None,
            thumb_url: None,
        }
    }

    /// 设置接收者 openid。
    pub fn to_user(self, to_user: impl Into<String>) -> Self {
        Self {
            base: self.base.to_user(to_user),
            ..self
        }
    }

    /// 设置 AI 会话上下文消息 id。
    pub fn ai_msg_context_msg_id(self, msg_id: impl Into<String>) -> Self {
        Self {
            base: self.base.ai_msg_context_msg_id(msg_id),
            ..self
        }
    }

    /// 设置标题（对应 Java `title`）。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置描述（对应 Java `description`）。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置跳转链接（对应 Java `url`）。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// 设置缩略图链接（对应 Java `thumbUrl`）。
    pub fn thumb_url(mut self, thumb_url: impl Into<String>) -> Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    /// 构建图文链接客服消息（对应 Java `build()`）。
    pub fn build(self) -> WxMaKefuMessage {
        let mut m = self.base.build();
        m.link = Some(KfLink {
            title: self.title,
            description: self.description,
            url: self.url,
            thumb_url: self.thumb_url,
        });
        m
    }
}
