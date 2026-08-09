//! 群发消息服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpMassMessageServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpMassMessageService, WxMpService};
use crate::bean::{
    WxMpMassGetResult, WxMpMassNews, WxMpMassOpenIdsMessage, WxMpMassPreviewMessage,
    WxMpMassSendResult, WxMpMassSpeedGetResult, WxMpMassTagMessage, WxMpMassUploadResult,
    WxMpMassVideo,
};
use crate::enums::wx_mp_api_url::mass_message;

/// 群发消息服务实现。
pub struct WxMpMassMessageServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpMassMessageServiceImpl {
    /// 构建群发消息服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 群发图文消息 JSON（对应 Java `WxMpMassNewsGsonAdapter`：articles 数组）。
    fn news_json(news: &WxMpMassNews) -> String {
        let articles: Vec<serde_json::Value> = news
            .articles
            .iter()
            .map(|a| {
                let mut article = serde_json::Map::new();
                article.insert("thumb_media_id".into(), serde_json::json!(a.thumb_media_id));
                article.insert("thumb_url".into(), serde_json::json!(a.thumb_url));
                article.insert("title".into(), serde_json::json!(a.title));
                article.insert("content".into(), serde_json::json!(a.content));
                if !a.author.is_empty() {
                    article.insert("author".into(), serde_json::json!(a.author));
                }
                if !a.content_source_url.is_empty() {
                    article.insert(
                        "content_source_url".into(),
                        serde_json::json!(a.content_source_url),
                    );
                }
                if !a.digest.is_empty() {
                    article.insert("digest".into(), serde_json::json!(a.digest));
                }
                article.insert(
                    "show_cover_pic".into(),
                    serde_json::json!(if a.show_cover_pic { "1" } else { "0" }),
                );
                if !a.url.is_empty() {
                    article.insert("url".into(), serde_json::json!(a.url));
                }
                if a.need_open_comment {
                    article.insert("need_open_comment".into(), serde_json::json!(1));
                }
                if a.only_fans_can_comment {
                    article.insert("only_fans_can_comment".into(), serde_json::json!(1));
                }
                serde_json::Value::Object(article)
            })
            .collect();
        serde_json::json!({ "articles": articles }).to_string()
    }

    /// 按消息类型插入内容分支（对应 Java adapter 的 msgtype 分支逻辑）。
    fn insert_branch(
        map: &mut serde_json::Map<String, serde_json::Value>,
        msg_type: &str,
        media_id: &str,
        content: &str,
        media_ids: &[String],
    ) {
        match msg_type {
            "text" => {
                map.insert("text".into(), serde_json::json!({ "content": content }));
            }
            "mpnews" | "voice" | "mpvideo" => {
                map.insert(msg_type.into(), serde_json::json!({ "media_id": media_id }));
            }
            "image" => {
                if !media_ids.is_empty() {
                    map.insert(
                        "images".into(),
                        serde_json::json!({ "media_ids": media_ids }),
                    );
                } else {
                    map.insert("image".into(), serde_json::json!({ "media_id": media_id }));
                }
            }
            _ => {}
        }
    }

    /// openid 列表群发 JSON（对应 Java `WxMpMassOpenIdsMessageGsonAdapter`）。
    fn open_ids_message_json(message: &WxMpMassOpenIdsMessage) -> String {
        let mut map = serde_json::Map::new();
        map.insert("touser".into(), serde_json::json!(message.to_users));
        Self::insert_branch(
            &mut map,
            &message.msg_type,
            &message.media_id,
            &message.content,
            &message.media_ids,
        );
        map.insert("msgtype".into(), serde_json::json!(message.msg_type));
        if !message.client_msg_id.is_empty() {
            map.insert(
                "clientmsgid".into(),
                serde_json::json!(message.client_msg_id),
            );
        }
        serde_json::Value::Object(map).to_string()
    }

    /// 标签群发 JSON（对应 Java `WxMpMassTagMessageGsonAdapter`：filter + send_ignore_reprint）。
    fn tag_message_json(message: &WxMpMassTagMessage) -> String {
        let mut map = serde_json::Map::new();
        let filter = if message.is_send_all || message.tag_id == 0 {
            serde_json::json!({ "is_to_all": true })
        } else {
            serde_json::json!({ "is_to_all": false, "tag_id": message.tag_id })
        };
        map.insert("filter".into(), filter);
        Self::insert_branch(
            &mut map,
            &message.msg_type,
            &message.media_id,
            &message.content,
            &message.media_ids,
        );
        map.insert("msgtype".into(), serde_json::json!(message.msg_type));
        map.insert(
            "send_ignore_reprint".into(),
            serde_json::json!(if message.send_ignore_reprint { 1 } else { 0 }),
        );
        if !message.client_msg_id.is_empty() {
            map.insert(
                "clientmsgid".into(),
                serde_json::json!(message.client_msg_id),
            );
        }
        serde_json::Value::Object(map).to_string()
    }

    /// 预览消息 JSON（对应 Java `WxMpMassPreviewMessageGsonAdapter`：towxname/touser）。
    fn preview_message_json(message: &WxMpMassPreviewMessage) -> String {
        let mut map = serde_json::Map::new();
        map.insert(
            "towxname".into(),
            serde_json::json!(message.to_wx_user_name),
        );
        map.insert(
            "touser".into(),
            serde_json::json!(message.to_wx_user_openid),
        );
        Self::insert_branch(
            &mut map,
            &message.msg_type,
            &message.media_id,
            &message.content,
            &[],
        );
        map.insert("msgtype".into(), serde_json::json!(message.msg_type));
        serde_json::Value::Object(map).to_string()
    }
}

#[async_trait]
impl WxMpMassMessageService for WxMpMassMessageServiceImpl {
    async fn mass_news_upload(
        &self,
        news: &WxMpMassNews,
    ) -> Result<WxMpMassUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::news_json(news);
        let response = svc
            .post(&mass_message::upload_news(config.as_ref()), &body)
            .await?;
        WxMpMassUploadResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn mass_video_upload(
        &self,
        video: &WxMpMassVideo,
    ) -> Result<WxMpMassUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"media_id": video.media_id, "description": video.description, "title": video.title});
        let response = svc
            .post(
                &mass_message::upload_video(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpMassUploadResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn mass_group_message_send(
        &self,
        message: &WxMpMassTagMessage,
    ) -> Result<WxMpMassSendResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::tag_message_json(message);
        let response = svc
            .post(&mass_message::send_all(config.as_ref()), &body)
            .await?;
        WxMpMassSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn mass_open_ids_message_send(
        &self,
        message: &WxMpMassOpenIdsMessage,
    ) -> Result<WxMpMassSendResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::open_ids_message_json(message);
        let response = svc
            .post(&mass_message::send(config.as_ref()), &body)
            .await?;
        WxMpMassSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn mass_message_preview(
        &self,
        preview: &WxMpMassPreviewMessage,
    ) -> Result<WxMpMassSendResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::preview_message_json(preview);
        let response = svc
            .post(&mass_message::preview(config.as_ref()), &body)
            .await?;
        WxMpMassSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete(&self, msg_id: i64, article_index: i32) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"msg_id": msg_id, "article_idx": article_index});
        svc.post(&mass_message::delete(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    async fn message_mass_speed_get(&self) -> Result<WxMpMassSpeedGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .post(&mass_message::speed_get(config.as_ref()), "{}")
            .await?;
        WxMpMassSpeedGetResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn message_mass_speed_set(&self, speed: i32) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"speed": speed});
        svc.post(&mass_message::speed_set(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    async fn message_mass_get(&self, msg_id: i64) -> Result<WxMpMassGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"msg_id": msg_id});
        let response = svc
            .post(&mass_message::get(config.as_ref()), &body.to_string())
            .await?;
        WxMpMassGetResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}
