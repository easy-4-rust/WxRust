//! Builder 基类。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.builder.BaseBuilder`：持有消息类型、
//! 接收者 openid 与 AI 会话上下文 id，`build()` 产出 `WxMaKefuMessage` 基体。

use crate::message::{AiMsgContext, WxMaKefuMessage};

/// 客服消息 builder 基类。
#[derive(Debug, Clone, Default)]
pub struct BaseBuilder {
    /// 消息类型。
    pub(crate) msg_type: Option<String>,
    /// 接收者 openid。
    pub(crate) to_user: Option<String>,
    /// AI 会话上下文消息 id。
    pub(crate) ai_msg_context_msg_id: Option<String>,
}

impl BaseBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收者 openid（对应 Java `toUser`）。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.to_user = Some(to_user.into());
        self
    }

    /// 设置 AI 会话上下文消息 id（对应 Java `aiMsgContextMsgId`）。
    pub fn ai_msg_context_msg_id(mut self, msg_id: impl Into<String>) -> Self {
        self.ai_msg_context_msg_id = Some(msg_id.into());
        self
    }

    /// 构建消息基体（对应 Java `build()`）。
    pub fn build(&self) -> WxMaKefuMessage {
        let mut m = WxMaKefuMessage {
            msg_type: self.msg_type.clone(),
            to_user: self.to_user.clone(),
            ..Default::default()
        };
        if let Some(msg_id) = &self.ai_msg_context_msg_id {
            m.ai_msg_context = Some(AiMsgContext {
                msg_id: Some(msg_id.clone()),
            });
        }
        m
    }
}
