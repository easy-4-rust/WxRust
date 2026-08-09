//! 图文被动回复消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlOutNewsMessage`。
//! 线格式 golden（Java 测试）：`<Articles><item>…</item>…</Articles>
//! <ArticleCount>n</ArticleCount>`，`ArticleCount` 为 articles 数量。

use super::wx_cp_xml_out_message::{
    WxCpXmlOutMessage, encrypt_xml, push_cdata_field, to_xml_with_body,
};
use crate::config::WxCpConfigStorage;

/// 图文消息（`MsgType = news`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlOutNewsMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxCpXmlOutMessage,
    /// 图文列表（对应 Java `articles`）。
    pub articles: Vec<Item>,
}

impl WxCpXmlOutNewsMessage {
    /// 构造图文消息（msgType 固定为 news）。
    pub fn new() -> Self {
        Self {
            base: WxCpXmlOutMessage {
                msg_type: Some("news".to_string()),
                ..Default::default()
            },
            articles: Vec::new(),
        }
    }

    /// 添加图文（对应 Java `addArticle(Item)`，同步 articleCount）。
    pub fn add_article(&mut self, item: Item) {
        self.articles.push(item);
    }

    /// 转换成 xml 格式（对应 Java `toXml()`）。
    pub fn to_xml(&self) -> String {
        let mut body = String::new();
        body.push_str("<Articles>");
        for item in &self.articles {
            body.push_str("<item>");
            push_cdata_field(&mut body, "Title", item.title.as_deref());
            push_cdata_field(&mut body, "Description", item.description.as_deref());
            push_cdata_field(&mut body, "PicUrl", item.pic_url.as_deref());
            push_cdata_field(&mut body, "Url", item.url.as_deref());
            body.push_str("</item>");
        }
        body.push_str("</Articles>");
        body.push_str(&format!(
            "<ArticleCount>{}</ArticleCount>",
            self.articles.len()
        ));
        to_xml_with_body(&self.base, &body)
    }

    /// 转换成加密的 xml 格式（对应 Java `toEncryptedXml`）。
    pub fn to_encrypted_xml(&self, config: &dyn WxCpConfigStorage) -> Result<String, String> {
        encrypt_xml(&self.to_xml(), config)
    }
}

/// 图文项（对应 Java `WxCpXmlOutNewsMessage.Item`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Item {
    /// 标题。
    pub title: Option<String>,
    /// 描述。
    pub description: Option<String>,
    /// 图片链接。
    pub pic_url: Option<String>,
    /// 点击跳转链接。
    pub url: Option<String>,
}
