//! 转接 AI 回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.TransferBizAiIvrBuilder`。

use crate::bean::message::WxMpXmlOutTransferBizAiIvrMessage;
use crate::builder::outxml::BaseBuilder;
use crate::builder::outxml::base_builder::set_common;

/// 转接 AI 回复消息构建器。
#[derive(Debug, Clone, Default)]
pub struct TransferBizAiIvrBuilder {
    base: BaseBuilder,
}

impl TransferBizAiIvrBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收方帐号。
    pub fn to_user(self, to_user: impl Into<String>) -> Self {
        Self {
            base: self.base.to_user(to_user),
        }
    }

    /// 设置开发者微信号。
    pub fn from_user(self, from_user: impl Into<String>) -> Self {
        Self {
            base: self.base.from_user(from_user),
        }
    }

    /// 构建转接 AI 回复消息。
    pub fn build(self) -> WxMpXmlOutTransferBizAiIvrMessage {
        let mut m = WxMpXmlOutTransferBizAiIvrMessage::new();
        set_common(&mut m.base, self.base);
        m
    }
}
