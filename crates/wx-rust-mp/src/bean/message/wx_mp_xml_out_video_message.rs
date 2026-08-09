//! 视频被动回复消息。
//!
//! 对应 Java `WxMpXmlOutVideoMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, video_to_xml};

/// 视频消息内容。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutVideo {
    /// 通过素材管理接口上传多媒体文件得到的 media_id。
    pub media_id: Option<String>,
    /// 视频消息的标题。
    pub title: Option<String>,
    /// 视频消息的描述。
    pub description: Option<String>,
}

/// 视频消息（`MsgType = video`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutVideoMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 视频内容。
    pub video: Option<WxMpXmlOutVideo>,
}

impl WxMpXmlOutVideoMessage {
    /// 构造视频消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("video".to_string()),
                ..Default::default()
            },
            video: None,
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        video_to_xml(self)
    }
}
