//! 视频消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.VideoBuilder`。

use crate::bean::message::{WxMpXmlOutVideo, WxMpXmlOutVideoMessage};
use crate::builder::outxml::BaseBuilder;
use crate::builder::outxml::base_builder::set_common;

/// 视频消息构建器。
#[derive(Debug, Clone, Default)]
pub struct VideoBuilder {
    base: BaseBuilder,
    media_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
}

impl VideoBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置素材 media_id。
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

    /// 构建视频消息。
    pub fn build(self) -> WxMpXmlOutVideoMessage {
        let mut m = WxMpXmlOutVideoMessage::new();
        set_common(&mut m.base, self.base);
        m.video = Some(WxMpXmlOutVideo {
            media_id: self.media_id,
            title: self.title,
            description: self.description,
        });
        m
    }
}
