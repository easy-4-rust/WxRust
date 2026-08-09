//! 视频被动回复消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutVideoMessage`。
//! 线格式：`<Video><MediaId>…<Title>…<Description>…</Video>`。

use super::wx_cp_xml_out_message::{
    WxCpXmlOutMessage, encrypt_xml, push_cdata_field, to_xml_with_body,
};
use crate::config::WxCpConfigStorage;

/// 视频消息（`MsgType = video`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutVideoMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxCpXmlOutMessage,
    /// 视频信息。
    pub video: Video,
}

impl WxCpXmlOutVideoMessage {
    /// 构造视频消息（msgType 固定为 video）。
    pub fn new() -> Self {
        Self {
            base: WxCpXmlOutMessage {
                msg_type: Some("video".to_string()),
                ..Default::default()
            },
            video: Video::default(),
        }
    }

    /// 设置媒体文件 id（对应 Java `setMediaId`）。
    pub fn set_media_id(&mut self, media_id: impl Into<String>) {
        self.video.media_id = Some(media_id.into());
    }

    /// 设置标题（对应 Java `setTitle`）。
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.video.title = Some(title.into());
    }

    /// 设置描述（对应 Java `setDescription`）。
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.video.description = Some(description.into());
    }

    /// 转换成 xml 格式（对应 Java `toXml()`）。
    pub fn to_xml(&self) -> String {
        let mut body = String::new();
        body.push_str("<Video>");
        push_cdata_field(&mut body, "MediaId", self.video.media_id.as_deref());
        push_cdata_field(&mut body, "Title", self.video.title.as_deref());
        push_cdata_field(&mut body, "Description", self.video.description.as_deref());
        body.push_str("</Video>");
        to_xml_with_body(&self.base, &body)
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxCpConfigStorage) -> Result<String, String> {
        encrypt_xml(&self.to_xml(), config)
    }
}

/// 视频信息（对应 Java `WxCpXmlOutVideoMessage.Video`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Video {
    /// 媒体文件 id。
    pub media_id: Option<String>,
    /// 标题。
    pub title: Option<String>,
    /// 描述。
    pub description: Option<String>,
}
