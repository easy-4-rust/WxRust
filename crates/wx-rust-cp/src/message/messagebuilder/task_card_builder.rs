//! 任务卡片消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.TaskCardBuilder`
//! （msgType 固定为 `taskcard`；safe 置 null，对应 Java `m.setSafe(null)`）。

use crate::bean::message::WxCpMessage;
use crate::bean::taskcard::TaskCardButton;
use crate::message::messagebuilder::BaseBuilder;

/// 任务卡片消息 builder。
#[derive(Debug, Clone, Default)]
pub struct TaskCardBuilder {
    base: BaseBuilder,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    task_id: Option<String>,
    buttons: Vec<TaskCardButton>,
}

impl TaskCardBuilder {
    /// 构建空 builder（msgType 固定为 taskcard）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("taskcard".to_string()),
                ..Default::default()
            },
            title: None,
            description: None,
            url: None,
            task_id: None,
            buttons: Vec::new(),
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

    /// 设置任务 id（对应 Java `taskId`）。
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = Some(task_id.into());
        self
    }

    /// 设置按钮列表。
    pub fn buttons(mut self, buttons: Vec<TaskCardButton>) -> Self {
        self.buttons = buttons;
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

    /// 构建消息（safe 置 null）。
    pub fn build(self) -> WxCpMessage {
        let mut m = self.base.build_base();
        m.safe = None;
        m.title = self.title;
        m.description = self.description;
        m.url = self.url;
        m.task_id = self.task_id;
        m.task_buttons = self.buttons;
        m
    }
}
