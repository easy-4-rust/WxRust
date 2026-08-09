//! 语音被动回复消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutVoiceMessage`。
//! 线格式：`<Voice><MediaId><![CDATA[…]]></MediaId></Voice>`。

use super::wx_cp_xml_out_message::{WxCpXmlOutMessage, encrypt_xml, to_xml_with_body};
use crate::config::WxCpConfigStorage;

/// 语音消息（`MsgType = voice`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutVoiceMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxCpXmlOutMessage,
    /// 媒体文件 id。
    pub media_id: Option<String>,
}

impl WxCpXmlOutVoiceMessage {
    /// 构造语音消息（msgType 固定为 voice）。
    pub fn new() -> Self {
        Self {
            base: WxCpXmlOutMessage {
                msg_type: Some("voice".to_string()),
                ..Default::default()
            },
            media_id: None,
        }
    }

    /// 转换成 xml 格式（对应 Java `toXml()`）。
    pub fn to_xml(&self) -> String {
        let mut body = String::new();
        if let Some(media_id) = self.media_id.as_deref() {
            body.push_str(&format!(
                "<Voice><MediaId><![CDATA[{media_id}]]></MediaId></Voice>"
            ));
        }
        to_xml_with_body(&self.base, &body)
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxCpConfigStorage) -> Result<String, String> {
        encrypt_xml(&self.to_xml(), config)
    }
}
