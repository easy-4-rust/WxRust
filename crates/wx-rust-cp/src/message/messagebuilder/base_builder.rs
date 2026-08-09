//! 消息 builder 基类。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.BaseBuilder`：持有
//! 公共字段（msgType/agentId/toUser/toParty/toTag/safe），`build()` 时
//! safe 缺省取 "0"（`KefuMsgSafe.NO`）。

use crate::bean::message::WxCpMessage;

/// 消息 builder 基类。
#[derive(Debug, Clone, Default)]
pub struct BaseBuilder {
    /// 消息类型（对应 Java `msgType`）。
    pub(crate) msg_type: Option<String>,
    /// 企业应用的 id。
    pub(crate) agent_id: Option<i32>,
    /// 接收消息的成员。
    pub(crate) to_user: Option<String>,
    /// 接收消息的部门。
    pub(crate) to_party: Option<String>,
    /// 接收消息的标签。
    pub(crate) to_tag: Option<String>,
    /// 是否保密消息。
    pub(crate) safe: Option<String>,
}

impl BaseBuilder {
    /// 设置企业应用的 id（对应 Java `agentId(Integer)`）。
    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.agent_id = Some(agent_id);
        self
    }

    /// 设置接收消息的成员（对应 Java `toUser(String)`）。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.to_user = Some(to_user.into());
        self
    }

    /// 设置接收消息的部门（对应 Java `toParty(String)`）。
    pub fn to_party(mut self, to_party: impl Into<String>) -> Self {
        self.to_party = Some(to_party.into());
        self
    }

    /// 设置接收消息的标签（对应 Java `toTag(String)`）。
    pub fn to_tag(mut self, to_tag: impl Into<String>) -> Self {
        self.to_tag = Some(to_tag.into());
        self
    }

    /// 设置是否保密消息（对应 Java `safe(String)`）。
    pub fn safe(mut self, safe: impl Into<String>) -> Self {
        self.safe = Some(safe.into());
        self
    }

    /// 构建消息公共字段（对应 Java `BaseBuilder.build()`；safe 缺省 "0"）。
    pub(crate) fn build_base(&self) -> WxCpMessage {
        let safe = self
            .safe
            .clone()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "0".to_string());
        WxCpMessage {
            agent_id: self.agent_id,
            msg_type: self.msg_type.clone(),
            to_user: self.to_user.clone(),
            to_party: self.to_party.clone(),
            to_tag: self.to_tag.clone(),
            safe: Some(safe),
            ..Default::default()
        }
    }
}
