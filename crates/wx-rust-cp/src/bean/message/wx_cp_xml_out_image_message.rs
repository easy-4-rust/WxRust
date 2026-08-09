//! 图片被动回复消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutImageMessage`。
//! 线格式 golden：`<Image><MediaId><![CDATA[…]]></MediaId></Image>`。

use super::wx_cp_xml_out_message::{WxCpXmlOutMessage, encrypt_xml, to_xml_with_body};
use crate::config::WxCpConfigStorage;

/// 图片消息（`MsgType = image`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutImageMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxCpXmlOutMessage,
    /// 媒体文件 id。
    pub media_id: Option<String>,
}

impl WxCpXmlOutImageMessage {
    /// 构造图片消息（msgType 固定为 image）。
    pub fn new() -> Self {
        Self {
            base: WxCpXmlOutMessage {
                msg_type: Some("image".to_string()),
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
                "<Image><MediaId><![CDATA[{media_id}]]></MediaId></Image>"
            ));
        }
        to_xml_with_body(&self.base, &body)
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxCpConfigStorage) -> Result<String, String> {
        encrypt_xml(&self.to_xml(), config)
    }
}
