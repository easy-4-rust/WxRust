//! 企业互联消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpLinkedCorpMessage`。Java
//! 以 `toJson()` 手工组装 JsonObject：顶层键序 `touser`(数组) → `toparty` →
//! `totag` → `toall`(1/0) → `msgtype` → `agentid`(有值) → 消息体子对象 →
//! `safe`(1/0，仅设置时输出)。

use crate::bean::article::{MpnewsArticle, NewArticle};
use crate::bean::message::wx_cp_message::opt_json;

/// 企业互联消息。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpLinkedCorpMessage {
    /// 是否发送给所有人（对应 Java `toall`）。
    pub is_to_all: Option<bool>,
    /// 接收消息的成员 userid 列表。
    pub to_users: Vec<String>,
    /// 接收消息的部门 id 列表。
    pub to_parties: Vec<String>,
    /// 接收消息的标签 id 列表。
    pub to_tags: Vec<String>,
    /// 企业应用的 id。
    pub agent_id: Option<i32>,
    /// 消息类型（text/markdown/textcard/image/file/video/news/mpnews/
    /// miniprogram_notice）。
    pub msg_type: Option<String>,
    /// 消息内容。
    pub content: Option<String>,
    /// 媒体文件 id。
    pub media_id: Option<String>,
    /// 图文消息缩略图的 media_id。
    pub thumb_media_id: Option<String>,
    /// 标题。
    pub title: Option<String>,
    /// 描述。
    pub description: Option<String>,
    /// 是否保密消息（仅设置时输出 `safe` 1/0）。
    pub is_safe: Option<bool>,
    /// 点击后跳转的链接。
    pub url: Option<String>,
    /// 按钮文字。
    pub btn_txt: Option<String>,
    /// 图文消息（news）。
    pub articles: Vec<NewArticle>,
    /// 图文消息（mpnews）。
    pub mp_news_articles: Vec<MpnewsArticle>,
    /// 小程序 appid。
    pub app_id: Option<String>,
    /// 点击消息卡片后的小程序页面。
    pub page: Option<String>,
    /// 是否放大第一个 content_item。
    pub emphasis_first_item: Option<bool>,
    /// 消息内容键值对。
    pub content_items: std::collections::HashMap<String, String>,
}

impl WxCpLinkedCorpMessage {
    /// 序列化为 JSON（对应 Java `toJson()`）。
    pub fn to_json(&self) -> String {
        let mut message = serde_json::Map::new();
        if !self.to_users.is_empty() {
            message.insert(
                "touser".to_string(),
                serde_json::Value::Array(
                    self.to_users
                        .iter()
                        .cloned()
                        .map(serde_json::Value::from)
                        .collect(),
                ),
            );
        }
        if !self.to_parties.is_empty() {
            message.insert(
                "toparty".to_string(),
                serde_json::Value::Array(
                    self.to_parties
                        .iter()
                        .cloned()
                        .map(serde_json::Value::from)
                        .collect(),
                ),
            );
        }
        if !self.to_tags.is_empty() {
            message.insert(
                "totag".to_string(),
                serde_json::Value::Array(
                    self.to_tags
                        .iter()
                        .cloned()
                        .map(serde_json::Value::from)
                        .collect(),
                ),
            );
        }
        if let Some(is_to_all) = self.is_to_all {
            message.insert(
                "toall".to_string(),
                serde_json::json!(if is_to_all { 1 } else { 0 }),
            );
        }
        message.insert("msgtype".to_string(), opt_json(&self.msg_type));
        if let Some(agent_id) = self.agent_id {
            message.insert("agentid".to_string(), serde_json::json!(agent_id));
        }
        self.handle_msg_type(&mut message);
        if let Some(is_safe) = self.is_safe {
            message.insert(
                "safe".to_string(),
                serde_json::json!(if is_safe { 1 } else { 0 }),
            );
        }
        serde_json::Value::Object(message).to_string()
    }

    /// 消息体分派（对应 Java `handleMsgType`）。
    fn handle_msg_type(&self, message: &mut serde_json::Map<String, serde_json::Value>) {
        match self.msg_type.as_deref() {
            Some("text") | Some("markdown") => {
                let mut text = serde_json::Map::new();
                text.insert("content".to_string(), opt_json(&self.content));
                message.insert(
                    self.msg_type.as_deref().unwrap_or_default().to_string(),
                    serde_json::Value::Object(text),
                );
            }
            Some("textcard") => {
                let mut text = serde_json::Map::new();
                text.insert("title".to_string(), opt_json(&self.title));
                text.insert("description".to_string(), opt_json(&self.description));
                text.insert("url".to_string(), opt_json(&self.url));
                text.insert("btntxt".to_string(), opt_json(&self.btn_txt));
                message.insert("textcard".to_string(), serde_json::Value::Object(text));
            }
            Some("image") | Some("file") => {
                let mut media = serde_json::Map::new();
                media.insert("media_id".to_string(), opt_json(&self.media_id));
                message.insert(
                    self.msg_type.as_deref().unwrap_or_default().to_string(),
                    serde_json::Value::Object(media),
                );
            }
            Some("video") => {
                let mut video = serde_json::Map::new();
                video.insert("media_id".to_string(), opt_json(&self.media_id));
                video.insert("title".to_string(), opt_json(&self.title));
                video.insert("description".to_string(), opt_json(&self.description));
                message.insert("video".to_string(), serde_json::Value::Object(video));
            }
            Some("news") => {
                let mut news = serde_json::Map::new();
                let articles = self
                    .articles
                    .iter()
                    .map(|article| {
                        serde_json::json!({
                            "title": article.title,
                            "description": article.description,
                            "url": article.url,
                            "picurl": article.pic_url,
                            "btntxt": article.btn_text,
                        })
                    })
                    .collect::<Vec<_>>();
                news.insert("articles".to_string(), serde_json::Value::Array(articles));
                message.insert("news".to_string(), serde_json::Value::Object(news));
            }
            Some("mpnews") => {
                let mut news = serde_json::Map::new();
                if let Some(media_id) = self.media_id.as_deref() {
                    news.insert("media_id".to_string(), serde_json::json!(media_id));
                } else {
                    let articles = self
                        .mp_news_articles
                        .iter()
                        .map(|article| {
                            serde_json::json!({
                                "title": article.title,
                                "thumb_media_id": article.thumb_media_id,
                                "author": article.author,
                                "content_source_url": article.content_source_url,
                                "content": article.content,
                                "digest": article.digest,
                            })
                        })
                        .collect::<Vec<_>>();
                    news.insert("articles".to_string(), serde_json::Value::Array(articles));
                }
                message.insert("mpnews".to_string(), serde_json::Value::Object(news));
            }
            Some("miniprogram_notice") => {
                let mut notice = serde_json::Map::new();
                notice.insert("appid".to_string(), opt_json(&self.app_id));
                notice.insert("page".to_string(), opt_json(&self.page));
                notice.insert("description".to_string(), opt_json(&self.description));
                notice.insert("title".to_string(), opt_json(&self.title));
                notice.insert(
                    "emphasis_first_item".to_string(),
                    self.emphasis_first_item
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(serde_json::Value::Null),
                );
                let content = self
                    .content_items
                    .iter()
                    .map(|(k, v)| serde_json::json!({ "key": k, "value": v }))
                    .collect::<Vec<_>>();
                notice.insert(
                    "content_item".to_string(),
                    serde_json::Value::Array(content),
                );
                message.insert(
                    "miniprogram_notice".to_string(),
                    serde_json::Value::Object(notice),
                );
            }
            _ => {
                // 未知类型不做任何处理（对应 Java default 分支）
            }
        }
    }
}
