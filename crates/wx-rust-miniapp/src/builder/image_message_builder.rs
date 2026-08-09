//! 图片消息 builder。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.builder.ImageMessageBuilder`。

use crate::builder::base_builder::BaseBuilder;
use crate::message::{KfImage, WxMaKefuMessage};

/// 图片消息构建器。
#[derive(Debug, Clone, Default)]
pub struct ImageMessageBuilder {
    base: BaseBuilder,
    media_id: Option<String>,
}

impl ImageMessageBuilder {
    /// 构建空 builder（消息类型 `image`）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("image".to_string()),
                ..Default::default()
            },
            media_id: None,
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

    /// 设置素材 media_id（对应 Java `mediaId`）。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
        self
    }

    /// 构建图片客服消息（对应 Java `build()`）。
    pub fn build(self) -> WxMaKefuMessage {
        let mut m = self.base.build();
        m.image = Some(KfImage {
            media_id: self.media_id,
        });
        m
    }
}
