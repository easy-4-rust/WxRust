//! 语音被动回复消息。
//!
//! 对应 Java `WxMpXmlOutVoiceMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, voice_to_xml};

/// 语音消息（`MsgType = voice`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutVoiceMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 通过素材管理接口上传多媒体文件得到的 media_id。
    pub media_id: Option<String>,
}

impl WxMpXmlOutVoiceMessage {
    /// 构造语音消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("voice".to_string()),
                ..Default::default()
            },
            media_id: None,
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        voice_to_xml(self)
    }
}
