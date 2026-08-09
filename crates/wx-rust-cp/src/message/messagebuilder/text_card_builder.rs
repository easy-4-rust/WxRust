//! 文本卡片消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.TextCardBuilder`
//! （msgType 固定为 `textcard`）。

use crate::bean::message::WxCpMessage;
use crate::message::messagebuilder::BaseBuilder;

/// 文本卡片消息 builder。
#[derive(Debug, Clone, Default)]
pub struct TextCardBuilder {
    base: BaseBuilder,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    btn_txt: Option<String>,
}

impl TextCardBuilder {
    /// 构建空 builder（msgType 固定为 textcard）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("textcard".to_string()),
                ..Default::default()
            },
            title: None,
            description: None,
            url: None,
            btn_txt: None,
        }
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

    /// 设置点击后跳转的链接。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// 设置按钮文字（对应 Java `btnTxt`）。
    pub fn btn_txt(mut self, btn_txt: impl Into<String>) -> Self {
        self.btn_txt = Some(btn_txt.into());
        self
    }

    /// 设置企业应用的 id。
    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.base = self.base.agent_id(agent_id);
        self
    }

    /// 设置接收消息的成员。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.base = self.base.to_user(to_user);
        self
    }

    /// 设置接收消息的部门。
    pub fn to_party(mut self, to_party: impl Into<String>) -> Self {
        self.base = self.base.to_party(to_party);
        self
    }

    /// 设置接收消息的标签。
    pub fn to_tag(mut self, to_tag: impl Into<String>) -> Self {
        self.base = self.base.to_tag(to_tag);
        self
    }

    /// 构建消息。
    pub fn build(self) -> WxCpMessage {
        let mut m = self.base.build_base();
        m.title = self.title;
        m.description = self.description;
        m.url = self.url;
        m.btn_txt = self.btn_txt;
        m
    }
}
