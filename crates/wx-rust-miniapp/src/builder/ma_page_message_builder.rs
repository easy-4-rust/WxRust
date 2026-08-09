//! 小程序卡片消息 builder。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.builder.MaPageMessageBuilder`。

use crate::builder::base_builder::BaseBuilder;
use crate::message::{KfMaPage, WxMaKefuMessage};

/// 小程序卡片消息构建器。
#[derive(Debug, Clone, Default)]
pub struct MaPageMessageBuilder {
    base: BaseBuilder,
    title: Option<String>,
    page_path: Option<String>,
    thumb_media_id: Option<String>,
}

impl MaPageMessageBuilder {
    /// 构建空 builder（消息类型 `miniprogrampage`）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("miniprogrampage".to_string()),
                ..Default::default()
            },
            title: None,
            page_path: None,
            thumb_media_id: None,
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

    /// 设置小程序页面路径（对应 Java `pagePath`）。
    pub fn page_path(mut self, page_path: impl Into<String>) -> Self {
        self.page_path = Some(page_path.into());
        self
    }

    /// 设置缩略图 media_id（对应 Java `thumbMediaId`）。
    pub fn thumb_media_id(mut self, thumb_media_id: impl Into<String>) -> Self {
        self.thumb_media_id = Some(thumb_media_id.into());
        self
    }

    /// 构建小程序卡片客服消息（对应 Java `build()`）。
    pub fn build(self) -> WxMaKefuMessage {
        let mut m = self.base.build();
        m.ma_page = Some(KfMaPage {
            title: self.title,
            page_path: self.page_path,
            thumb_media_id: self.thumb_media_id,
        });
        m
    }
}
