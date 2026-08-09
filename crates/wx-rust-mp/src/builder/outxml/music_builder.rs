//! 音乐消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.MusicBuilder`。

use crate::bean::message::{WxMpXmlOutMusic, WxMpXmlOutMusicMessage};
use crate::builder::outxml::BaseBuilder;
use crate::builder::outxml::base_builder::set_common;

/// 音乐消息构建器。
#[derive(Debug, Clone, Default)]
pub struct MusicBuilder {
    base: BaseBuilder,
    title: Option<String>,
    description: Option<String>,
    thumb_media_id: Option<String>,
    music_url: Option<String>,
    hq_music_url: Option<String>,
}

impl MusicBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置音乐标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置音乐描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置缩略图 media_id。
    pub fn thumb_media_id(mut self, thumb_media_id: impl Into<String>) -> Self {
        self.thumb_media_id = Some(thumb_media_id.into());
        self
    }

    /// 设置音乐链接。
    pub fn music_url(mut self, music_url: impl Into<String>) -> Self {
        self.music_url = Some(music_url.into());
        self
    }

    /// 设置高质量音乐链接。
    pub fn hq_music_url(mut self, hq_music_url: impl Into<String>) -> Self {
        self.hq_music_url = Some(hq_music_url.into());
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

    /// 构建音乐消息。
    pub fn build(self) -> WxMpXmlOutMusicMessage {
        let mut m = WxMpXmlOutMusicMessage::new();
        set_common(&mut m.base, self.base);
        m.music = Some(WxMpXmlOutMusic {
            title: self.title,
            description: self.description,
            thumb_media_id: self.thumb_media_id,
            music_url: self.music_url,
            hq_music_url: self.hq_music_url,
        });
        m
    }
}
