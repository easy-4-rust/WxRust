//! 图片被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder.ImageBuilder`。

use crate::bean::message::WxCpXmlOutImageMessage;
use crate::message::outxmlbuilder::BaseBuilder;

/// 图片消息 builder。
#[derive(Debug, Clone, Default)]
pub struct ImageBuilder {
    base: BaseBuilder,
    media_id: Option<String>,
}

impl ImageBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置媒体文件 id。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.media_id = Some(media_id.into());
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

    /// 构建消息（msgType 固定为 image）。
    pub fn build(self) -> WxCpXmlOutImageMessage {
        let mut m = WxCpXmlOutImageMessage::new();
        self.base.set_common(&mut m.base);
        m.media_id = self.media_id;
        m
    }
}
