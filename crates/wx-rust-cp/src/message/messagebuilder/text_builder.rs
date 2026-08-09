//! 文本消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.TextBuilder`
//! （msgType 固定为 `text`）。

use crate::bean::message::WxCpMessage;
use crate::message::messagebuilder::BaseBuilder;

/// 文本消息 builder。
#[derive(Debug, Clone, Default)]
pub struct TextBuilder {
    base: BaseBuilder,
    content: Option<String>,
}

impl TextBuilder {
    /// 构建空 builder（msgType 固定为 text）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("text".to_string()),
                ..Default::default()
            },
            content: None,
        }
    }

    /// 设置消息内容（对应 Java `content(String)`）。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
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

    /// 设置是否保密消息。
    pub fn safe(mut self, safe: impl Into<String>) -> Self {
        self.base = self.base.safe(safe);
        self
    }

    /// 构建消息（对应 Java `build()`）。
    pub fn build(self) -> WxCpMessage {
        let mut m = self.base.build_base();
        m.content = self.content;
        m
    }
}
