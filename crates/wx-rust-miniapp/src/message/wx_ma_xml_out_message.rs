//! 输出给微信服务器的 XML 格式消息。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaXmlOutMessage`。XStream
//! 线格式：`<xml>` 根，`ToUserName`/`FromUserName`/`MsgType` 为 CDATA、
//! `CreateTime` 裸值、null 字段省略。

use crate::config::WxMaConfig;
use crate::message::WxMaOutMessage;
use crate::message::wx_ma_message::push_cdata;
use crate::util::crypto::WxMaCryptUtils;

/// 微信小程序输出给微信服务器的 XML 格式消息（对应 Java `WxMaXmlOutMessage`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxMaXmlOutMessage {
    /// 接收方帐号（收到的 OpenID）。
    pub to_user_name: Option<String>,
    /// 开发者微信号。
    pub from_user_name: Option<String>,
    /// 消息创建时间。
    pub create_time: Option<i64>,
    /// 消息类型。
    pub msg_type: Option<String>,
}

impl WxMaXmlOutMessage {
    /// 转换成 xml 格式（对应 Java `toXml()`，XStream 声明序）。
    pub fn to_xml(&self) -> String {
        let mut s = String::from("<xml>");
        push_cdata(&mut s, "ToUserName", self.to_user_name.as_deref());
        push_cdata(&mut s, "FromUserName", self.from_user_name.as_deref());
        if let Some(t) = self.create_time {
            s.push_str(&format!("<CreateTime>{t}</CreateTime>"));
        }
        push_cdata(&mut s, "MsgType", self.msg_type.as_deref());
        s.push_str("</xml>");
        s
    }
}

impl WxMaOutMessage for WxMaXmlOutMessage {
    /// 转换成 XML 格式。
    fn to_xml(&self) -> String {
        WxMaXmlOutMessage::to_xml(self)
    }

    /// 转换成 JSON 格式（对于 XML 消息类型，返回 XML 格式，与 Java 一致）。
    fn to_json(&self) -> String {
        self.to_xml()
    }

    /// 转换成加密的 xml 格式。
    fn to_encrypted_xml(&self, config: &dyn WxMaConfig) -> Result<String, String> {
        let plain_xml = self.to_xml();
        let crypt_util = WxMaCryptUtils::new(config)?;
        crypt_util.encrypt(&plain_xml)
    }

    /// 转换成加密的 JSON 格式（对于 XML 消息类型，返回加密的 XML 格式，与 Java 一致）。
    fn to_encrypted_json(&self, config: &dyn WxMaConfig) -> Result<String, String> {
        self.to_encrypted_xml(config)
    }
}
