//! 被动回复消息基类。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.WxMpXmlOutMessage`。Java 为
//! 抽象类 + 静态 Builder 工厂；Rust 以普通结构 + 关联函数返回各 Builder。
//! `to_xml` 按 XStream 线格式输出：String 字段包 CDATA、数值裸值、null 省略、
//! 父类字段在前子类字段在后。

use crate::bean::message::{
    WxMpXmlOutImageMessage, WxMpXmlOutMusicMessage, WxMpXmlOutNewsMessage, WxMpXmlOutTextMessage,
    WxMpXmlOutTransferBizAiIvrMessage, WxMpXmlOutTransferKefuMessage, WxMpXmlOutVideoMessage,
    WxMpXmlOutVoiceMessage,
};
use crate::builder::outxml::{
    ImageBuilder, MusicBuilder, NewsBuilder, TextBuilder, TransferBizAiIvrBuilder,
    TransferCustomerServiceBuilder, VideoBuilder, VoiceBuilder,
};
use crate::config::WxMpConfigStorage;
use crate::util::crypto::WxMpCryptUtil;

/// 被动回复消息基类。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutMessage {
    /// 接收方帐号（收到的 OpenID）。
    pub to_user_name: Option<String>,
    /// 开发者微信号。
    pub from_user_name: Option<String>,
    /// 消息创建时间。
    pub create_time: Option<i64>,
    /// 消息类型。
    pub msg_type: Option<String>,
    /// 加密内容。
    pub encrypt: Option<String>,
    /// 消息签名。
    pub msg_signature: Option<String>,
    /// 时间戳。
    pub time_stamp: Option<String>,
    /// 随机串。
    pub nonce: Option<String>,
}

impl WxMpXmlOutMessage {
    /// 获得文本消息 builder。
    pub fn text() -> TextBuilder {
        TextBuilder::new()
    }

    /// 获得图片消息 builder。
    pub fn image() -> ImageBuilder {
        ImageBuilder::new()
    }

    /// 获得语音消息 builder。
    pub fn voice() -> VoiceBuilder {
        VoiceBuilder::new()
    }

    /// 获得视频消息 builder。
    pub fn video() -> VideoBuilder {
        VideoBuilder::new()
    }

    /// 获得音乐消息 builder。
    pub fn music() -> MusicBuilder {
        MusicBuilder::new()
    }

    /// 获得图文消息 builder。
    pub fn news() -> NewsBuilder {
        NewsBuilder::new()
    }

    /// 获得客服消息 builder。
    pub fn transfer_customer_service() -> TransferCustomerServiceBuilder {
        TransferCustomerServiceBuilder::new()
    }

    /// 获得转接 AI 回复消息 builder。
    pub fn transfer_biz_ai_ivr() -> TransferBizAiIvrBuilder {
        TransferBizAiIvrBuilder::new()
    }

    /// 转换成 xml 格式（对应 Java `toXml()`）。
    ///
    /// XStream 线格式：`<xml>` 根，字段按父类声明序 + 子类字段序，
    /// String 包 `<![CDATA[...]]>`，数值裸值，null 字段省略。
    pub fn to_xml(&self, subclass_body: &str) -> String {
        let mut s = String::from("<xml>");
        push_cdata_field(&mut s, "ToUserName", self.to_user_name.as_deref());
        push_cdata_field(&mut s, "FromUserName", self.from_user_name.as_deref());
        if let Some(t) = self.create_time {
            s.push_str(&format!("<CreateTime>{t}</CreateTime>"));
        }
        push_cdata_field(&mut s, "MsgType", self.msg_type.as_deref());
        s.push_str(subclass_body);
        push_cdata_field(&mut s, "Encrypt", self.encrypt.as_deref());
        push_cdata_field(&mut s, "MsgSignature", self.msg_signature.as_deref());
        push_cdata_field(&mut s, "TimeStamp", self.time_stamp.as_deref());
        push_cdata_field(&mut s, "Nonce", self.nonce.as_deref());
        s.push_str("</xml>");
        s
    }

    /// 转换成加密的结果（对应 Java `toEncrypted`）。
    pub fn to_encrypted(&self, config: &dyn WxMpConfigStorage) -> Result<Self, String> {
        let plain_xml = self.to_xml("");
        let crypt_util = WxMpCryptUtil::new(config)?;
        let ctx = crypt_util.encrypt_context(&plain_xml)?;
        Ok(WxMpXmlOutMessage {
            nonce: Some(ctx.nonce.clone()),
            encrypt: Some(ctx.encrypted_xml.clone()),
            time_stamp: Some(ctx.timestamp.clone()),
            msg_signature: Some(ctx.signature.clone()),
            ..Default::default()
        })
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxMpConfigStorage) -> Result<String, String> {
        let plain_xml = self.to_xml("");
        let crypt_util = WxMpCryptUtil::new(config)?;
        crypt_util.encrypt(&plain_xml)
    }
}

/// 输出 String 字段为 CDATA 元素（null 省略）。
pub(crate) fn push_cdata_field(s: &mut String, name: &str, value: Option<&str>) {
    if let Some(v) = value {
        s.push_str(&format!("<{name}><![CDATA[{v}]]></{name}>"));
    }
}

/// 将文本消息序列化为 xml（对齐 Java `WxMpXmlOutTextMessage.toXml()`）。
pub fn text_to_xml(m: &WxMpXmlOutTextMessage) -> String {
    m.base.to_xml(&{
        let mut body = String::new();
        push_cdata_field(&mut body, "Content", m.content.as_deref());
        body
    })
}

/// 将图片消息序列化为 xml。
pub fn image_to_xml(m: &WxMpXmlOutImageMessage) -> String {
    let mut body = String::new();
    if let Some(media_id) = m.media_id.as_deref() {
        body.push_str(&format!(
            "<Image><MediaId><![CDATA[{media_id}]]></MediaId></Image>"
        ));
    }
    m.base.to_xml(&body)
}

/// 将语音消息序列化为 xml。
pub fn voice_to_xml(m: &WxMpXmlOutVoiceMessage) -> String {
    let mut body = String::new();
    if let Some(media_id) = m.media_id.as_deref() {
        body.push_str(&format!(
            "<Voice><MediaId><![CDATA[{media_id}]]></MediaId></Voice>"
        ));
    }
    m.base.to_xml(&body)
}

/// 将视频消息序列化为 xml。
pub fn video_to_xml(m: &WxMpXmlOutVideoMessage) -> String {
    let mut body = String::new();
    if let Some(v) = m.video.as_ref() {
        body.push_str("<Video>");
        push_cdata_field(&mut body, "MediaId", v.media_id.as_deref());
        push_cdata_field(&mut body, "Title", v.title.as_deref());
        push_cdata_field(&mut body, "Description", v.description.as_deref());
        body.push_str("</Video>");
    }
    m.base.to_xml(&body)
}

/// 将音乐消息序列化为 xml。
pub fn music_to_xml(m: &WxMpXmlOutMusicMessage) -> String {
    let mut body = String::new();
    if let Some(mu) = m.music.as_ref() {
        body.push_str("<Music>");
        push_cdata_field(&mut body, "Title", mu.title.as_deref());
        push_cdata_field(&mut body, "Description", mu.description.as_deref());
        push_cdata_field(&mut body, "ThumbMediaId", mu.thumb_media_id.as_deref());
        push_cdata_field(&mut body, "MusicUrl", mu.music_url.as_deref());
        push_cdata_field(&mut body, "HQMusicUrl", mu.hq_music_url.as_deref());
        body.push_str("</Music>");
    }
    m.base.to_xml(&body)
}

/// 将图文消息序列化为 xml（XStream 声明序：Articles 在前，ArticleCount 在后）。
pub fn news_to_xml(m: &WxMpXmlOutNewsMessage) -> String {
    let mut body = String::new();
    body.push_str("<Articles>");
    for item in &m.articles {
        body.push_str("<item>");
        push_cdata_field(&mut body, "Title", item.title.as_deref());
        push_cdata_field(&mut body, "Description", item.description.as_deref());
        push_cdata_field(&mut body, "PicUrl", item.pic_url.as_deref());
        push_cdata_field(&mut body, "Url", item.url.as_deref());
        body.push_str("</item>");
    }
    body.push_str("</Articles>");
    if !m.articles.is_empty() {
        body.push_str(&format!(
            "<ArticleCount>{}</ArticleCount>",
            m.articles.len()
        ));
    }
    m.base.to_xml(&body)
}

/// 将客服转接消息序列化为 xml。
pub fn transfer_kefu_to_xml(m: &WxMpXmlOutTransferKefuMessage) -> String {
    let mut body = String::new();
    if let Some(kf_account) = m.kf_account.as_deref() {
        body.push_str(&format!(
            "<TransInfo><KfAccount><![CDATA[{kf_account}]]></KfAccount></TransInfo>"
        ));
    }
    m.base.to_xml(&body)
}

/// 将 AI 转接消息序列化为 xml（无额外字段）。
pub fn transfer_biz_ai_ivr_to_xml(m: &WxMpXmlOutTransferBizAiIvrMessage) -> String {
    m.base.to_xml("")
}
