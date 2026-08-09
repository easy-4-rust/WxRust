//! 图片被动回复消息。
//!
//! 对应 Java `WxMpXmlOutImageMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, image_to_xml};

/// 图片消息（`MsgType = image`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutImageMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 通过素材管理接口上传多媒体文件得到的 media_id。
    pub media_id: Option<String>,
}

impl WxMpXmlOutImageMessage {
    /// 构造图片消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("image".to_string()),
                ..Default::default()
            },
            media_id: None,
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        image_to_xml(self)
    }
}
