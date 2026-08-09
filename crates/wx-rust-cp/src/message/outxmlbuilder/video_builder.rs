//! 视频被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder.VideoBuilder`。

use crate::bean::message::WxCpXmlOutVideoMessage;
use crate::message::outxmlbuilder::BaseBuilder;

/// 视频消息 builder。
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

    /// 设置接收方帐号。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.base = self.base.to_user(to_user);
        self
    }

    /// 设置开发者微信号。
    pub fn from_user(mut self, from_user: impl Into<String>) -> Self {
        self.base = self.base.from_user(from_user);
        self
    }

    /// 构建消息（msgType 固定为 video）。
    pub fn build(self) -> WxCpXmlOutVideoMessage {
        let mut m = WxCpXmlOutVideoMessage::new();
        self.base.set_common(&mut m.base);
        m.video.media_id = self.media_id;
        m.video.title = self.title;
        m.video.description = self.description;
        m
    }
}
