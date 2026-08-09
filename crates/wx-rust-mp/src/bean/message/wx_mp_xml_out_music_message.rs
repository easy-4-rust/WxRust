//! 音乐被动回复消息。
//!
//! 对应 Java `WxMpXmlOutMusicMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, music_to_xml};

/// 音乐消息内容。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutMusic {
    /// 音乐标题。
    pub title: Option<String>,
    /// 音乐描述。
    pub description: Option<String>,
    /// 缩略图的媒体 id。
    pub thumb_media_id: Option<String>,
    /// 音乐链接。
    pub music_url: Option<String>,
    /// 高质量音乐链接（WIFI 环境优先使用）。
    pub hq_music_url: Option<String>,
}

/// 音乐消息（`MsgType = music`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutMusicMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 音乐内容。
    pub music: Option<WxMpXmlOutMusic>,
}

impl WxMpXmlOutMusicMessage {
    /// 构造音乐消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("music".to_string()),
                ..Default::default()
            },
            music: None,
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        music_to_xml(self)
    }
}
