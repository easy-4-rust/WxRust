//! WxMpFreePublishService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpFreePublishServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpFreePublishService, WxMpService};

use crate::bean::freepublish::{WxMpFreePublishInfo, WxMpFreePublishList, WxMpFreePublishStatus};
use crate::enums::wx_mp_api_url::free_publish;

/// WxMpFreePublish服务实现。
pub struct WxMpFreePublishServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpFreePublishServiceImpl {
    /// 构建 WxMpFreePublish服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 从响应 JSON 提取指定字段（对应 Java `GsonParser.parse(json).get(key)`）。
    fn extract_str(json: &str, key: &str) -> Result<String, WxErrorException> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get(key)
            .map(|v| {
                v.as_str()
                    .map(String::from)
                    .unwrap_or_else(|| v.to_string())
            })
            .ok_or_else(|| WxErrorException::from_code(-99, format!("{key} 缺失")))
    }

    /// 判断响应 errcode 是否为 "0"（对应 Java `ERRCODE_SUCCESS`）。
    fn err_code_is_zero(json: &str) -> Result<bool, WxErrorException> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(value
            .get("errcode")
            .map(|v| v.to_string() == "0")
            .unwrap_or(false))
    }
}

#[async_trait]
impl WxMpFreePublishService for WxMpFreePublishServiceImpl {
    async fn submit(&self, media_id: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"media_id": media_id});
        let response = svc
            .post(&free_publish::submit(config.as_ref()), &body.to_string())
            .await?;
        Self::extract_str(&response, "publish_id")
    }

    async fn get_push_status(
        &self,
        publish_id: &str,
    ) -> Result<WxMpFreePublishStatus, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"publish_id": publish_id});
        let response = svc
            .post(
                &free_publish::get_push_status(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpFreePublishStatus::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_push(&self, article_id: &str, index: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"article_id": article_id, "index": index});
        let response = svc
            .post(&free_publish::del_push(config.as_ref()), &body.to_string())
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn get_article_from_id(
        &self,
        article_id: &str,
    ) -> Result<WxMpFreePublishInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"article_id": article_id});
        let response = svc
            .post(
                &free_publish::get_article(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpFreePublishInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_publication_records(
        &self,
        offset: i32,
        count: i32,
    ) -> Result<WxMpFreePublishList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"offset": offset, "count": count, "no_content": 0});
        let response = svc
            .post(&free_publish::batch_get(config.as_ref()), &body.to_string())
            .await?;
        WxMpFreePublishList::from_json(&response).map_err(WxErrorException::Serde)
    }
}
