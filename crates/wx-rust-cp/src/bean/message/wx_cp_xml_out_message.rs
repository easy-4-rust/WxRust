//! 被动回复消息基类。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutMessage`。Java 为
//! 抽象类 + 静态 Builder 工厂；Rust 以普通结构 + 关联函数返回各 Builder。
//! `to_xml` 按 XStream 线格式输出：String 字段包 CDATA、数值裸值、null 省略、
//! 父类字段在前子类字段在后（Java 测试 golden：`<xml><ToUserName>…
//! <FromUserName>…<CreateTime>…<MsgType>…` + 子类字段 + `</xml>`）。

use crate::config::WxCpConfigStorage;
use crate::message::outxmlbuilder::{
    EventBuilder, ImageBuilder, NewsBuilder, TaskCardBuilder, TextBuilder, UpdateButtonBuilder,
    VideoBuilder, VoiceBuilder,
};
use crate::util::crypto::WxCpCryptUtils;

/// 被动回复消息基类。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutMessage {
    /// 接收方帐号（对应 Java `ToUserName`）。
    pub to_user_name: Option<String>,
    /// 开发者微信号（对应 Java `FromUserName`）。
    pub from_user_name: Option<String>,
    /// 消息创建时间。
    pub create_time: Option<i64>,
    /// 消息类型。
    pub msg_type: Option<String>,
}

impl WxCpXmlOutMessage {
    /// 获得文本消息 builder（对应 Java `TEXT()`）。
    pub fn text() -> TextBuilder {
        TextBuilder::new()
    }

    /// 获得图片消息 builder（对应 Java `IMAGE()`）。
    pub fn image() -> ImageBuilder {
        ImageBuilder::new()
    }

    /// 获得语音消息 builder（对应 Java `VOICE()`）。
    pub fn voice() -> VoiceBuilder {
        VoiceBuilder::new()
    }

    /// 获得视频消息 builder（对应 Java `VIDEO()`）。
    pub fn video() -> VideoBuilder {
        VideoBuilder::new()
    }

    /// 获得图文消息 builder（对应 Java `NEWS()`）。
    pub fn news() -> NewsBuilder {
        NewsBuilder::new()
    }

    /// 获得任务卡片消息 builder（对应 Java `TASK_CARD()`）。
    pub fn task_card() -> TaskCardBuilder {
        TaskCardBuilder::new()
    }

    /// 获得更新按钮消息 builder（对应 Java `UPDATE_BUTTON()`）。
    pub fn update_button() -> UpdateButtonBuilder {
        UpdateButtonBuilder::new()
    }

    /// 获得事件消息 builder（对应 Java `EVENT()`）。
    pub fn event() -> EventBuilder {
        EventBuilder::new()
    }
}

/// 输出 String 字段为 CDATA 元素（null 省略）。
pub(crate) fn push_cdata_field(s: &mut String, name: &str, value: Option<&str>) {
    if let Some(v) = value {
        s.push_str(&format!("<{name}><![CDATA[{v}]]></{name}>"));
    }
}

/// 将公共字段序列化为 xml 头（`<xml>` 根 + 公共字段 + 子类字段体 + `</xml>`，
/// 对应 Java `toXml()` 的 XStream 线格式）。
pub(crate) fn to_xml_with_body(base: &WxCpXmlOutMessage, subclass_body: &str) -> String {
    let mut s = String::from("<xml>");
    push_cdata_field(&mut s, "ToUserName", base.to_user_name.as_deref());
    push_cdata_field(&mut s, "FromUserName", base.from_user_name.as_deref());
    if let Some(t) = base.create_time {
        s.push_str(&format!("<CreateTime>{t}</CreateTime>"));
    }
    push_cdata_field(&mut s, "MsgType", base.msg_type.as_deref());
    s.push_str(subclass_body);
    s.push_str("</xml>");
    s
}

/// 加密被动回复消息（对应 Java `toEncryptedXml(WxCpConfigStorage)`：
/// 先序列化明文 xml，再经 `WxCpCryptUtil.encrypt` 加密打包）。
pub(crate) fn encrypt_xml(
    plain_xml: &str,
    config: &dyn WxCpConfigStorage,
) -> Result<String, String> {
    let crypt_util = WxCpCryptUtils::new(config)?;
    crypt_util.encrypt(plain_xml)
}
