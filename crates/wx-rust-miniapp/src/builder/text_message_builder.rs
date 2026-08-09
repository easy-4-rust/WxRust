//! 文本消息 builder。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.builder.TextMessageBuilder`。

use crate::builder::base_builder::BaseBuilder;
use crate::message::{KfText, WxMaKefuMessage};

/// 文本消息构建器。
#[derive(Debug, Clone, Default)]
pub struct TextMessageBuilder {
    base: BaseBuilder,
    content: Option<String>,
}

impl TextMessageBuilder {
    /// 构建空 builder（消息类型 `text`）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("text".to_string()),
                ..Default::default()
            },
            content: None,
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

    /// 设置文本内容（对应 Java `content`）。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 构建文本客服消息（对应 Java `build()`）。
    pub fn build(self) -> WxMaKefuMessage {
        let mut m = self.base.build();
        m.text = Some(KfText {
            content: self.content,
        });
        m
    }
}
