//! 视频消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.VideoBuilder`
//! （msgType 固定为 `video`）。

use crate::bean::message::WxCpMessage;
use crate::message::messagebuilder::BaseBuilder;

/// 视频消息 builder。
#[derive(Debug, Clone, Default)]
pub struct VideoBuilder {
    base: BaseBuilder,
    media_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    thumb_media_id: Option<String>,
}

impl VideoBuilder {
    /// 构建空 builder（msgType 固定为 video）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("video".to_string()),
                ..Default::default()
            },
            media_id: None,
            title: None,
            description: None,
            thumb_media_id: None,
        }
    }

    /// 设置媒体文件 id。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
        self
    }

    /// 设置标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置缩略图 media_id（对应 Java `thumbMediaId`）。
    pub fn thumb_media_id(mut self, thumb_media_id: impl Into<String>) -> Self {
        self.thumb_media_id = Some(thumb_media_id.into());
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
        m.title = self.title;
        m.description = self.description;
        m.thumb_media_id = self.thumb_media_id;
        m
    }
}
