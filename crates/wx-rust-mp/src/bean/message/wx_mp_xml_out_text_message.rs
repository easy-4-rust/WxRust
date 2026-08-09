//! 文本被动回复消息。
//!
//! 对应 Java `WxMpXmlOutTextMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, text_to_xml};

/// 文本消息（`MsgType = text`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutTextMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 回复的消息内容。
    pub content: Option<String>,
}

impl WxMpXmlOutTextMessage {
    /// 构造文本消息（msgType 固定为 text）。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("text".to_string()),
                ..Default::default()
            },
            content: None,
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        text_to_xml(self)
    }
}
