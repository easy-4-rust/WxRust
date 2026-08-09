//! 订阅消息事件。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.WxMpSubscribeMsgEvent`。

/// 订阅消息弹窗事件（`subscribe_msg_popup_event`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxMpSubscribeMsgPopupEvent {
    /// 弹窗事件列表。
    pub list: Vec<PopupEvent>,
}

/// 订阅消息变更事件（`subscribe_msg_change_event`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxMpSubscribeMsgChangeEvent {
    /// 变更事件列表。
    pub list: Vec<ChangeEvent>,
}

/// 订阅消息发送事件（`subscribe_msg_sent_event`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxMpSubscribeMsgSentEvent {
    /// 发送事件列表。
    pub list: Vec<SentEvent>,
}

/// 弹窗事件项。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PopupEvent {
    /// 模板 id。
    pub template_id: Option<String>,
    /// 订阅状态。
    pub subscribe_status_string: Option<String>,
    /// 弹窗场景。
    pub popup_scene: Option<String>,
}

impl PopupEvent {
    /// 模板 id。
    pub fn get_template_id(&self) -> Option<&str> {
        self.template_id.as_deref()
    }

    /// 订阅状态（accept/reject）。
    pub fn get_subscribe_status_string(&self) -> Option<&str> {
        self.subscribe_status_string.as_deref()
    }

    /// 弹窗场景。
    pub fn get_popup_scene(&self) -> Option<&str> {
        self.popup_scene.as_deref()
    }
}

/// 变更事件项。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeEvent {
    /// 模板 id。
    pub template_id: Option<String>,
    /// 订阅状态。
    pub subscribe_status_string: Option<String>,
}

impl ChangeEvent {
    /// 模板 id。
    pub fn get_template_id(&self) -> Option<&str> {
        self.template_id.as_deref()
    }

    /// 订阅状态。
    pub fn get_subscribe_status_string(&self) -> Option<&str> {
        self.subscribe_status_string.as_deref()
    }
}

/// 发送事件项。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SentEvent {
    /// 模板 id。
    pub template_id: Option<String>,
    /// 消息 id。
    pub msg_id: Option<String>,
    /// 错误码。
    pub error_code: Option<String>,
    /// 错误状态。
    pub error_status: Option<String>,
}

impl SentEvent {
    /// 模板 id。
    pub fn get_template_id(&self) -> Option<&str> {
        self.template_id.as_deref()
    }

    /// 消息 id。
    pub fn get_msg_id(&self) -> Option<&str> {
        self.msg_id.as_deref()
    }

    /// 错误码。
    pub fn get_error_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }

    /// 错误状态。
    pub fn get_error_status(&self) -> Option<&str> {
        self.error_status.as_deref()
    }
}

/// 订阅消息事件容器（Java 以静态内部类组织，Rust 平铺为独立类型）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxMpSubscribeMsgEvent {
    /// 弹窗事件。
    pub popup_event: Option<WxMpSubscribeMsgPopupEvent>,
    /// 变更事件。
    pub change_event: Option<WxMpSubscribeMsgChangeEvent>,
    /// 发送事件。
    pub sent_event: Option<WxMpSubscribeMsgSentEvent>,
}
