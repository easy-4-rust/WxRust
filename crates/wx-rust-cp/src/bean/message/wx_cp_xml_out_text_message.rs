//! 文本被动回复消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutTextMessage`。
//! 线格式 golden：`<xml><ToUserName>…<FromUserName>…<CreateTime>…
//! <MsgType><![CDATA[text]]></MsgType><Content><![CDATA[…]]></Content></xml>`。

use super::wx_cp_xml_out_message::{
    WxCpXmlOutMessage, encrypt_xml, push_cdata_field, to_xml_with_body,
};
use crate::config::WxCpConfigStorage;

/// 文本消息（`MsgType = text`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutTextMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxCpXmlOutMessage,
    /// 回复的消息内容。
    pub content: Option<String>,
}

impl WxCpXmlOutTextMessage {
    /// 构造文本消息（msgType 固定为 text）。
    pub fn new() -> Self {
        Self {
            base: WxCpXmlOutMessage {
                msg_type: Some("text".to_string()),
                ..Default::default()
            },
            content: None,
        }
    }

    /// 转换成 xml 格式（对应 Java `toXml()`）。
    pub fn to_xml(&self) -> String {
        let mut body = String::new();
        push_cdata_field(&mut body, "Content", self.content.as_deref());
        to_xml_with_body(&self.base, &body)
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxCpConfigStorage) -> Result<String, String> {
        encrypt_xml(&self.to_xml(), config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bean::message::WxCpXmlOutMessage;

    /// Java `WxCpXmlOutTextMessageTest.test` 线格式 golden（去除空白后比较）。
    #[test]
    fn to_xml_golden() {
        let mut m = WxCpXmlOutTextMessage::new();
        m.content = Some("content".to_string());
        m.base.create_time = Some(1122);
        m.base.from_user_name = Some("from".to_string());
        m.base.to_user_name = Some("to".to_string());
        let expected = concat!(
            "<xml>",
            "<ToUserName><![CDATA[to]]></ToUserName>",
            "<FromUserName><![CDATA[from]]></FromUserName>",
            "<CreateTime>1122</CreateTime>",
            "<MsgType><![CDATA[text]]></MsgType>",
            "<Content><![CDATA[content]]></Content>",
            "</xml>"
        );
        assert_eq!(m.to_xml(), expected);
    }

    /// Java `WxCpXmlOutTextMessageTest.testBuild`：builder 链式构建。
    #[test]
    fn build_golden() {
        let m = WxCpXmlOutMessage::text()
            .content("content")
            .from_user("from")
            .to_user("to")
            .build();
        let expected = concat!(
            "<xml>",
            "<ToUserName><![CDATA[to]]></ToUserName>",
            "<FromUserName><![CDATA[from]]></FromUserName>",
            "<CreateTime>",
            "</CreateTime>",
            "<MsgType><![CDATA[text]]></MsgType>",
            "<Content><![CDATA[content]]></Content>",
            "</xml>"
        );
        // CreateTime 动态生成，仅校验其余部分
        let xml = m.to_xml();
        assert!(xml.starts_with("<xml><ToUserName><![CDATA[to]]></ToUserName><FromUserName><![CDATA[from]]></FromUserName><CreateTime>"));
        assert!(xml.ends_with("</CreateTime><MsgType><![CDATA[text]]></MsgType><Content><![CDATA[content]]></Content></xml>"));
        let _ = expected;
    }
}
