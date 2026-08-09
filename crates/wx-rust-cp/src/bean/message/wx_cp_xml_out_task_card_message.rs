//! 任务卡片被动回复消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutTaskCardMessage`
//! （`MsgType = update_taskcard`）：`<TaskCard><![CDATA[…]]></TaskCard>`
//! 携带替换按钮文案（`XStreamReplaceNameConverter` 语义）。

use super::wx_cp_xml_out_message::{WxCpXmlOutMessage, encrypt_xml, to_xml_with_body};
use crate::config::WxCpConfigStorage;

/// 任务卡片消息（`MsgType = update_taskcard`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutTaskCardMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxCpXmlOutMessage,
    /// 替换按钮文案（对应 Java `replaceName`）。
    pub replace_name: Option<String>,
}

impl WxCpXmlOutTaskCardMessage {
    /// 构造任务卡片消息（msgType 固定为 update_taskcard）。
    pub fn new() -> Self {
        Self {
            base: WxCpXmlOutMessage {
                msg_type: Some("update_taskcard".to_string()),
                ..Default::default()
            },
            replace_name: None,
        }
    }

    /// 转换成 xml 格式（对应 Java `toXml()`）。
    pub fn to_xml(&self) -> String {
        let mut body = String::new();
        if let Some(replace_name) = self.replace_name.as_deref() {
            body.push_str(&format!("<TaskCard><![CDATA[{replace_name}]]></TaskCard>"));
        }
        to_xml_with_body(&self.base, &body)
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxCpConfigStorage) -> Result<String, String> {
        encrypt_xml(&self.to_xml(), config)
    }
}
