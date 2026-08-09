//! 硬件信息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.HardWare`。

/// 硬件平台相关信息。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HardWare {
    /// 硬件视图消息。
    pub message_view: Option<String>,
    /// 硬件动作消息。
    pub message_action: Option<String>,
}

impl HardWare {
    /// 硬件视图消息。
    pub fn get_message_view(&self) -> Option<&str> {
        self.message_view.as_deref()
    }

    /// 硬件动作消息。
    pub fn get_message_action(&self) -> Option<&str> {
        self.message_action.as_deref()
    }
}
