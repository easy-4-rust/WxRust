//! 文件消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.FileBuilder`
//! （msgType 固定为 `file`）。

use crate::bean::message::WxCpMessage;
use crate::message::messagebuilder::BaseBuilder;

/// 文件消息 builder。
#[derive(Debug, Clone, Default)]
pub struct FileBuilder {
    base: BaseBuilder,
    media_id: Option<String>,
}

impl FileBuilder {
    /// 构建空 builder（msgType 固定为 file）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("file".to_string()),
                ..Default::default()
            },
            media_id: None,
        }
    }

    /// 设置媒体文件 id。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
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
        m.media_id = self.media_id;
        m
    }
}
