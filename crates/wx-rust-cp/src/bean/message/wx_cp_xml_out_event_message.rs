//! 事件被动回复消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutEventMessage`
//! （`MsgType = event`）：通讯录变更事件回包（ChangeType/UserID 等）。

use super::wx_cp_xml_out_message::{
    WxCpXmlOutMessage, encrypt_xml, push_cdata_field, to_xml_with_body,
};
use crate::config::WxCpConfigStorage;

/// 事件消息（`MsgType = event`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutEventMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxCpXmlOutMessage,
    /// 事件类型（对应 Java `Event`）。
    pub event: Option<String>,
    /// 群 ID。
    pub chat_id: Option<String>,
    /// 变更类型（对应 Java `ChangeType`）。
    pub change_type: Option<String>,
    /// 变更详情。
    pub update_detail: Option<String>,
    /// 加入场景。
    pub join_scene: Option<String>,
    /// 退出场景。
    pub quit_scene: Option<String>,
    /// 成员变更数量。
    pub mem_change_cnt: Option<String>,
    /// 标签类型。
    pub tag_type: Option<String>,
    /// 客户群策略 id。
    pub strategy_id: Option<String>,
    /// 变更信息的成员 UserID。
    pub user_id: Option<String>,
    /// 变更信息的外部联系人 userid。
    pub external_user_id: Option<String>,
    /// 「联系我」方式配置的 state 参数。
    pub state: Option<String>,
    /// 欢迎语 code。
    pub welcome_code: Option<String>,
    /// 来源。
    pub source: Option<String>,
    /// 客户接替失败的原因。
    pub fail_reason: Option<String>,
    /// 部门 Id（或标签 id）。
    pub id: Option<String>,
}

impl WxCpXmlOutEventMessage {
    /// 构造事件消息（msgType 固定为 event）。
    pub fn new() -> Self {
        Self {
            base: WxCpXmlOutMessage {
                msg_type: Some("event".to_string()),
                ..Default::default()
            },
            event: None,
            chat_id: None,
            change_type: None,
            update_detail: None,
            join_scene: None,
            quit_scene: None,
            mem_change_cnt: None,
            tag_type: None,
            strategy_id: None,
            user_id: None,
            external_user_id: None,
            state: None,
            welcome_code: None,
            source: None,
            fail_reason: None,
            id: None,
        }
    }

    /// 转换成 xml 格式（对应 Java `toXml()`）。
    pub fn to_xml(&self) -> String {
        let mut body = String::new();
        push_cdata_field(&mut body, "Event", self.event.as_deref());
        push_cdata_field(&mut body, "ChatId", self.chat_id.as_deref());
        push_cdata_field(&mut body, "ChangeType", self.change_type.as_deref());
        push_cdata_field(&mut body, "UpdateDetail", self.update_detail.as_deref());
        push_cdata_field(&mut body, "JoinScene", self.join_scene.as_deref());
        push_cdata_field(&mut body, "QuitScene", self.quit_scene.as_deref());
        push_cdata_field(&mut body, "MemChangeCnt", self.mem_change_cnt.as_deref());
        push_cdata_field(&mut body, "TagType", self.tag_type.as_deref());
        push_cdata_field(&mut body, "StrategyId", self.strategy_id.as_deref());
        push_cdata_field(&mut body, "UserID", self.user_id.as_deref());
        push_cdata_field(
            &mut body,
            "ExternalUserID",
            self.external_user_id.as_deref(),
        );
        push_cdata_field(&mut body, "State", self.state.as_deref());
        push_cdata_field(&mut body, "WelcomeCode", self.welcome_code.as_deref());
        push_cdata_field(&mut body, "Source", self.source.as_deref());
        push_cdata_field(&mut body, "FailReason", self.fail_reason.as_deref());
        push_cdata_field(&mut body, "Id", self.id.as_deref());
        to_xml_with_body(&self.base, &body)
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxCpConfigStorage) -> Result<String, String> {
        encrypt_xml(&self.to_xml(), config)
    }
}
