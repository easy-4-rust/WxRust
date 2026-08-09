//! 家校沟通消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpSchoolContactMessage`。
//! Java 以 `toJson()` 手工组装 JsonObject：顶层键序 `recv_scope`(有值) →
//! `to_parent_userid`(数组) → `to_student_userid` → `to_party` → `toall`(1/0)
//! → `msgtype` → `agentid`(有值) → `enable_id_trans` → `enable_duplicate_check`
//! → `duplicate_check_interval` → 消息体子对象（text/image/file/voice/video/
//! news/mpnews/miniprogram）。

use crate::bean::article::{MpnewsArticle, NewArticle};
use crate::bean::message::wx_cp_message::opt_json;

/// 家校沟通消息。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpSchoolContactMessage {
    /// 接收范围（对应 Java `recv_scope`）。
    pub recv_scope: Option<i32>,
    /// 接收消息的家长 userid 列表。
    pub to_parent_user_id: Vec<String>,
    /// 接收消息的学生 userid 列表。
    pub to_student_user_id: Vec<String>,
    /// 接收消息的部门 id 列表。
    pub to_party: Vec<String>,
    /// 是否发送给所有人（对应 Java `toall`）。
    pub to_all: Option<bool>,
    /// 消息类型（text/image/file/voice/video/news/mpnews/miniprogram）。
    pub msg_type: Option<String>,
    /// 企业应用的 id。
    pub agent_id: Option<i32>,
    /// 消息内容。
    pub content: Option<String>,
    /// 是否开启 id 转译。
    pub enable_id_trans: Option<bool>,
    /// 是否开启重复消息检查。
    pub enable_duplicate_check: Option<bool>,
    /// 重复消息检查的时间间隔。
    pub duplicate_check_interval: Option<i32>,
    /// 媒体文件 id。
    pub media_id: Option<String>,
    /// 标题。
    pub title: Option<String>,
    /// 描述。
    pub description: Option<String>,
    /// 图文消息缩略图的 media_id。
    pub thumb_media_id: Option<String>,
    /// 小程序 appid。
    pub app_id: Option<String>,
    /// 点击消息卡片后的小程序页面。
    pub page_path: Option<String>,
    /// 图文消息（news）。
    pub articles: Vec<NewArticle>,
    /// 图文消息（mpnews）。
    pub mp_news_articles: Vec<MpnewsArticle>,
}

impl WxCpSchoolContactMessage {
    /// 序列化为 JSON（对应 Java `toJson()`）。
    pub fn to_json(&self) -> String {
        let mut message = serde_json::Map::new();
        if let Some(recv_scope) = self.recv_scope {
            message.insert("recv_scope".to_string(), serde_json::json!(recv_scope));
        }
        if !self.to_parent_user_id.is_empty() {
            message.insert(
                "to_parent_userid".to_string(),
                serde_json::Value::Array(
                    self.to_parent_user_id
                        .iter()
                        .cloned()
                        .map(serde_json::Value::from)
                        .collect(),
                ),
            );
        }
        if !self.to_student_user_id.is_empty() {
            message.insert(
                "to_student_userid".to_string(),
                serde_json::Value::Array(
                    self.to_student_user_id
                        .iter()
                        .cloned()
                        .map(serde_json::Value::from)
                        .collect(),
                ),
            );
        }
        if !self.to_party.is_empty() {
            message.insert(
                "to_party".to_string(),
                serde_json::Value::Array(
                    self.to_party
                        .iter()
                        .cloned()
                        .map(serde_json::Value::from)
                        .collect(),
                ),
            );
        }
        if let Some(to_all) = self.to_all {
            message.insert(
                "toall".to_string(),
                serde_json::json!(if to_all { 1 } else { 0 }),
            );
        }
        message.insert("msgtype".to_string(), opt_json(&self.msg_type));
        if let Some(agent_id) = self.agent_id {
            message.insert("agentid".to_string(), serde_json::json!(agent_id));
        }
        if self.enable_id_trans == Some(true) {
            message.insert("enable_id_trans".to_string(), serde_json::json!(1));
        }
        if self.enable_duplicate_check == Some(true) {
            message.insert("enable_duplicate_check".to_string(), serde_json::json!(1));
        }
        if let Some(interval) = self.duplicate_check_interval {
            message.insert(
                "duplicate_check_interval".to_string(),
                serde_json::json!(interval),
            );
        }
        self.handle_msg_type(&mut message);
        serde_json::Value::Object(message).to_string()
    }

    /// 消息体分派（对应 Java `handleMsgType`）。
    fn handle_msg_type(&self, message: &mut serde_json::Map<String, serde_json::Value>) {
        match self.msg_type.as_deref() {
            Some("text") => {
                let mut text = serde_json::Map::new();
                text.insert("content".to_string(), opt_json(&self.content));
                message.insert("text".to_string(), serde_json::Value::Object(text));
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
                message.insert("mpnews".to_string(), serde_json::Value::Object(news));
            }
            Some("miniprogram") => {
                let mut miniprogram = serde_json::Map::new();
                miniprogram.insert("appid".to_string(), opt_json(&self.app_id));
                miniprogram.insert("pagepath".to_string(), opt_json(&self.page_path));
                miniprogram.insert("title".to_string(), opt_json(&self.title));
                miniprogram.insert("thumb_media_id".to_string(), opt_json(&self.thumb_media_id));
                message.insert(
                    "miniprogram".to_string(),
                    serde_json::Value::Object(miniprogram),
                );
            }
            _ => {
                // 未知类型不做任何处理（对应 Java default 分支）
            }
        }
    }
}
