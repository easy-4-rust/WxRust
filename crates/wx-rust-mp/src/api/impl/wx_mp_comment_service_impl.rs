//! WxMpCommentService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpCommentServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpCommentService, WxMpService};

use crate::bean::comment::WxMpCommentListVo;
use crate::enums::wx_mp_api_url::comment;

/// WxMpComment服务实现。
pub struct WxMpCommentServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpCommentServiceImpl {
    /// 构建 WxMpComment服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 构建评论操作公共参数（对应 Java `buildJson`）。
    fn build_json(msg_data_id: &str, index: Option<i32>, user_comment_id: i64) -> String {
        let mut body = serde_json::Map::new();
        body.insert("msg_data_id".into(), serde_json::json!(msg_data_id));
        body.insert("user_comment_id".into(), serde_json::json!(user_comment_id));
        if let Some(i) = index {
            body.insert("index".into(), serde_json::json!(i));
        }
        serde_json::Value::Object(body).to_string()
    }
}

#[async_trait]
impl WxMpCommentService for WxMpCommentServiceImpl {
    async fn open(&self, msg_data_id: &str, index: Option<i32>) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert("msg_data_id".into(), serde_json::json!(msg_data_id));
        if let Some(i) = index {
            body.insert("index".into(), serde_json::json!(i));
        }
        svc.post(
            &comment::open(config.as_ref()),
            &serde_json::Value::Object(body).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn close(&self, msg_data_id: &str, index: Option<i32>) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert("msg_data_id".into(), serde_json::json!(msg_data_id));
        if let Some(i) = index {
            body.insert("index".into(), serde_json::json!(i));
        }
        svc.post(
            &comment::close(config.as_ref()),
            &serde_json::Value::Object(body).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn list(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        begin: i32,
        count: i32,
        r#type: i32,
    ) -> Result<WxMpCommentListVo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert("msg_data_id".into(), serde_json::json!(msg_data_id));
        body.insert("begin".into(), serde_json::json!(begin));
        body.insert("count".into(), serde_json::json!(count));
        body.insert("type".into(), serde_json::json!(r#type));
        if let Some(i) = index {
            body.insert("index".into(), serde_json::json!(i));
        }
        let response = svc
            .post(
                &comment::list(config.as_ref()),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        WxMpCommentListVo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn mark_elect(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::mark_elect(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn unmark_elect(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::unmark_elect(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn delete(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::delete(config.as_ref()), &body).await?;
        Ok(())
    }

    async fn reply_add(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
        content: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body: serde_json::Value =
            serde_json::from_str(&Self::build_json(msg_data_id, index, user_comment_id))
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        body.as_object_mut()
            .ok_or_else(|| WxErrorException::from_code(-99, "内部错误"))?
            .insert("content".into(), serde_json::json!(content));
        svc.post(&comment::reply_add(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    async fn reply_delete(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::reply_delete(config.as_ref()), &body)
            .await?;
        Ok(())
    }
}
