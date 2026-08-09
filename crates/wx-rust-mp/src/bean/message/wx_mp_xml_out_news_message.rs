//! 图文被动回复消息。
//!
//! 对应 Java `WxMpXmlOutNewsMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, news_to_xml};

/// 图文消息条目。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutNewsMessageItem {
    /// 图文消息标题。
    pub title: Option<String>,
    /// 图文消息描述。
    pub description: Option<String>,
    /// 图片链接。
    pub pic_url: Option<String>,
    /// 点击图文消息跳转链接。
    pub url: Option<String>,
}

/// 图文消息（`MsgType = news`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutNewsMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 图文消息列表。
    pub articles: Vec<WxMpXmlOutNewsMessageItem>,
}

impl WxMpXmlOutNewsMessage {
    /// 构造图文消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("news".to_string()),
                ..Default::default()
            },
            articles: Vec::new(),
        }
    }

    /// 添加图文条目（对应 Java `addArticle`）。
    pub fn add_article(&mut self, item: WxMpXmlOutNewsMessageItem) {
        self.articles.push(item);
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        news_to_xml(self)
    }
}
