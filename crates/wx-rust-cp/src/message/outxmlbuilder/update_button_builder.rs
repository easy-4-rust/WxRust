//! 更新按钮被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder.UpdateButtonBuilder`。

use crate::bean::message::WxCpXmlOutUpdateBtnMessage;
use crate::message::outxmlbuilder::BaseBuilder;

/// 更新按钮消息 builder。
#[derive(Debug, Clone, Default)]
pub struct UpdateButtonBuilder {
    base: BaseBuilder,
    replace_name: Option<String>,
}

impl UpdateButtonBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置替换按钮文案（对应 Java `replaceName`）。
    pub fn replace_name(mut self, replace_name: impl Into<String>) -> Self {
        self.replace_name = Some(replace_name.into());
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

    /// 构建消息（msgType 固定为 update_button）。
    pub fn build(self) -> WxCpXmlOutUpdateBtnMessage {
        let mut m = WxCpXmlOutUpdateBtnMessage::new();
        self.base.set_common(&mut m.base);
        m.replace_name = self.replace_name;
        m
    }
}
