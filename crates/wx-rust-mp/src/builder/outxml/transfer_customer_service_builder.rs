//! 客服转接消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.TransferCustomerServiceBuilder`。

use crate::bean::message::WxMpXmlOutTransferKefuMessage;
use crate::builder::outxml::BaseBuilder;
use crate::builder::outxml::base_builder::set_common;

/// 客服转接消息构建器。
#[derive(Debug, Clone, Default)]
pub struct TransferCustomerServiceBuilder {
    base: BaseBuilder,
    kf_account: Option<String>,
}

impl TransferCustomerServiceBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 指定会话客服的帐号。
    pub fn kf_account(mut self, kf_account: impl Into<String>) -> Self {
        self.kf_account = Some(kf_account.into());
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

    /// 构建客服转接消息。
    pub fn build(self) -> WxMpXmlOutTransferKefuMessage {
        let mut m = WxMpXmlOutTransferKefuMessage::new();
        set_common(&mut m.base, self.base);
        m.kf_account = self.kf_account;
        m
    }
}
