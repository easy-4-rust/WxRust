//! 事件被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder.EventBuilder`：
//! 通讯录变更事件回包。

use crate::bean::message::WxCpXmlOutEventMessage;
use crate::message::outxmlbuilder::BaseBuilder;

/// 事件消息 builder。
#[derive(Debug, Clone, Default)]
pub struct EventBuilder {
    base: BaseBuilder,
    event: Option<String>,
    chat_id: Option<String>,
    change_type: Option<String>,
    update_detail: Option<String>,
    join_scene: Option<String>,
    quit_scene: Option<String>,
    mem_change_cnt: Option<String>,
    tag_type: Option<String>,
    strategy_id: Option<String>,
    user_id: Option<String>,
    external_user_id: Option<String>,
    state: Option<String>,
    welcome_code: Option<String>,
    source: Option<String>,
    fail_reason: Option<String>,
    id: Option<String>,
}

impl EventBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置事件类型。
    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    /// 设置群 ID。
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = Some(chat_id.into());
        self
    }

    /// 设置变更类型（对应 Java `changeType`）。
    pub fn change_type(mut self, change_type: impl Into<String>) -> Self {
        self.change_type = Some(change_type.into());
        self
    }

    /// 设置变更详情。
    pub fn update_detail(mut self, update_detail: impl Into<String>) -> Self {
        self.update_detail = Some(update_detail.into());
        self
    }

    /// 设置加入场景。
    pub fn join_scene(mut self, join_scene: impl Into<String>) -> Self {
        self.join_scene = Some(join_scene.into());
        self
    }

    /// 设置退出场景。
    pub fn quit_scene(mut self, quit_scene: impl Into<String>) -> Self {
        self.quit_scene = Some(quit_scene.into());
        self
    }

    /// 设置成员变更数量。
    pub fn mem_change_cnt(mut self, mem_change_cnt: impl Into<String>) -> Self {
        self.mem_change_cnt = Some(mem_change_cnt.into());
        self
    }

    /// 设置标签类型。
    pub fn tag_type(mut self, tag_type: impl Into<String>) -> Self {
        self.tag_type = Some(tag_type.into());
        self
    }

    /// 设置客户群策略 id。
    pub fn strategy_id(mut self, strategy_id: impl Into<String>) -> Self {
        self.strategy_id = Some(strategy_id.into());
        self
    }

    /// 设置变更信息的成员 UserID（对应 Java `userID`）。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 设置变更信息的外部联系人 userid（对应 Java `externalUserID`）。
    pub fn external_user_id(mut self, external_user_id: impl Into<String>) -> Self {
        self.external_user_id = Some(external_user_id.into());
        self
    }

    /// 设置「联系我」方式配置的 state 参数。
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// 设置来源。
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// 设置欢迎语 code。
    pub fn welcome_code(mut self, welcome_code: impl Into<String>) -> Self {
        self.welcome_code = Some(welcome_code.into());
        self
    }

    /// 设置客户接替失败的原因。
    pub fn fail_reason(mut self, fail_reason: impl Into<String>) -> Self {
        self.fail_reason = Some(fail_reason.into());
        self
    }

    /// 设置部门 Id（或标签 id）。
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
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

    /// 构建消息（msgType 固定为 event）。
    pub fn build(self) -> WxCpXmlOutEventMessage {
        let mut m = WxCpXmlOutEventMessage::new();
        self.base.set_common(&mut m.base);
        m.event = self.event;
        m.chat_id = self.chat_id;
        m.change_type = self.change_type;
        m.update_detail = self.update_detail;
        m.join_scene = self.join_scene;
        m.quit_scene = self.quit_scene;
        m.mem_change_cnt = self.mem_change_cnt;
        m.tag_type = self.tag_type;
        m.strategy_id = self.strategy_id;
        m.user_id = self.user_id;
        m.external_user_id = self.external_user_id;
        m.state = self.state;
        m.welcome_code = self.welcome_code;
        m.source = self.source;
        m.fail_reason = self.fail_reason;
        m.id = self.id;
        m
    }
}
