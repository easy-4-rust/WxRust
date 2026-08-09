//! 群聊消息（企业微信群聊会话消息）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpAppChatMessage`。Java 以
//! `toJson()` 手工组装 JsonObject：顶层键序 `msgtype` → `chatid` → `safe`
//! （仅 true 时输出 1）→ 消息体子对象。

use crate::bean::article::{MpnewsArticle, NewArticle};
use crate::bean::message::wx_cp_message::opt_json;

/// 群聊消息。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpAppChatMessage {
    /// 消息类型（text/markdown/textcard/image/file/voice/video/news/mpnews）。
    pub msg_type: Option<String>,
    /// 消息内容。
    pub content: Option<String>,
    /// 群聊 id。
    pub chat_id: Option<String>,
    /// 媒体文件 id。
    pub media_id: Option<String>,
    /// 标题。
    pub title: Option<String>,
    /// 描述。
    pub description: Option<String>,
    /// 是否保密消息（仅 true 时输出 `"safe":1`）。
    pub safe: Option<bool>,
    /// 点击后跳转的链接。
    pub url: Option<String>,
    /// 按钮文字。
    pub btn_txt: Option<String>,
    /// 图文消息（news）。
    pub articles: Vec<NewArticle>,
    /// 图文消息（mpnews）。
    pub mpnews_articles: Vec<MpnewsArticle>,
}

impl WxCpAppChatMessage {
    /// 序列化为 JSON（对应 Java `toJson()`）。
    pub fn to_json(&self) -> String {
        let mut message = serde_json::Map::new();
        message.insert("msgtype".to_string(), opt_json(&self.msg_type));
        message.insert("chatid".to_string(), opt_json(&self.chat_id));
        if self.safe == Some(true) {
            message.insert("safe".to_string(), serde_json::json!(1));
        }
        self.handle_msg_type(&mut message);
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
            Some("image") | Some("file") | Some("voice") => {
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
                        .mpnews_articles
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
            _ => {
                // 未知类型不做任何处理（对应 Java default 分支）
            }
        }
    }
}
