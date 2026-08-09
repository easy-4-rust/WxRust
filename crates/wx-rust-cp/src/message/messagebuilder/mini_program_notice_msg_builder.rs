//! 小程序通知消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.MiniProgramNoticeMsgBuilder`
//! （msgType 固定为 `miniprogram_notice`）。

use std::collections::HashMap;

use crate::bean::message::WxCpMessage;
use crate::message::messagebuilder::BaseBuilder;

/// 小程序通知消息 builder。
#[derive(Debug, Clone, Default)]
pub struct MiniProgramNoticeMsgBuilder {
    base: BaseBuilder,
    app_id: Option<String>,
    page: Option<String>,
    title: Option<String>,
    description: Option<String>,
    content_items: HashMap<String, String>,
    emphasis_first_item: Option<bool>,
}

impl MiniProgramNoticeMsgBuilder {
    /// 构建空 builder（msgType 固定为 miniprogram_notice）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("miniprogram_notice".to_string()),
                ..Default::default()
            },
            app_id: None,
            page: None,
            title: None,
            description: None,
            content_items: HashMap::new(),
            emphasis_first_item: None,
        }
    }

    /// 设置小程序 appid。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// 设置小程序页面。
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
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

    /// 设置消息内容键值对。
    pub fn content_items(mut self, content_items: HashMap<String, String>) -> Self {
        self.content_items = content_items;
        self
    }

    /// 设置是否放大第一个 content_item。
    pub fn emphasis_first_item(mut self, emphasis_first_item: bool) -> Self {
        self.emphasis_first_item = Some(emphasis_first_item);
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
        m.content_items = self.content_items;
        m.app_id = self.app_id;
        m.description = self.description;
        m.title = self.title;
        m.emphasis_first_item = self.emphasis_first_item;
        m.page = self.page;
        m
    }
}
