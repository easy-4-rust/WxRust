//! 语音消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.VoiceBuilder`。

use crate::bean::message::WxMpXmlOutVoiceMessage;
use crate::builder::outxml::BaseBuilder;
use crate::builder::outxml::base_builder::set_common;

/// 语音消息构建器。
#[derive(Debug, Clone, Default)]
pub struct VoiceBuilder {
    base: BaseBuilder,
    media_id: Option<String>,
}

impl VoiceBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置素材 media_id。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
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

    /// 构建语音消息。
    pub fn build(self) -> WxMpXmlOutVoiceMessage {
        let mut m = WxMpXmlOutVoiceMessage::new();
        set_common(&mut m.base, self.base);
        m.media_id = self.media_id;
        m
    }
}
