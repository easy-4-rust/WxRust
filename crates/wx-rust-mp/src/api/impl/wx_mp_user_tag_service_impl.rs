//! WxMpUserTagService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpUserTagServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpUserTagService};

use crate::bean::tag::{WxTagListUser, WxUserTag};
use crate::enums::wx_mp_api_url::tags;

/// WxMpUserTag服务实现。
pub struct WxMpUserTagServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpUserTagServiceImpl {
    /// 构建 WxMpUserTag服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpUserTagService for WxMpUserTagServiceImpl {
    async fn tag_create(&self, name: &str) -> Result<WxUserTag, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tag": {"name": name}});
        let response = svc
            .post(&tags::create(config.as_ref()), &body.to_string())
            .await?;
        WxUserTag::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn tag_get(&self) -> Result<Vec<WxUserTag>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc.get(&tags::get(config.as_ref()), "").await?;
        WxUserTag::list_from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn tag_update(&self, tag_id: i64, name: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tag": {"id": tag_id, "name": name}});
        svc.post(&tags::update(config.as_ref()), &body.to_string())
            .await?;
        // Java 语义：响应 errcode==0 返回 true（执行器已校验 errcode）
        Ok(true)
    }

    async fn tag_delete(&self, tag_id: i64) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tag": {"id": tag_id}});
        svc.post(&tags::delete(config.as_ref()), &body.to_string())
            .await?;
        Ok(true)
    }

    async fn tag_list_user(
        &self,
        tag_id: i64,
        next_openid: &str,
    ) -> Result<WxTagListUser, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tagid": tag_id, "next_openid": next_openid.trim()});
        let response = svc
            .post(&tags::tag_user_get(config.as_ref()), &body.to_string())
            .await?;
        WxTagListUser::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn user_tag_list(&self, openid: &str) -> Result<Vec<i64>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"openid": openid});
        let response = svc
            .post(&tags::get_id_list(config.as_ref()), &body.to_string())
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("tagid_list")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_i64()).collect())
            .ok_or_else(|| WxErrorException::from_code(-99, "tagid_list 字段缺失"))
    }

    async fn batch_tagging(&self, tag_id: i64, openids: &[&str]) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tagid": tag_id, "openid_list": openids});
        svc.post(&tags::batch_tagging(config.as_ref()), &body.to_string())
            .await?;
        Ok(true)
    }

    async fn batch_untagging(
        &self,
        tag_id: i64,
        openids: &[&str],
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tagid": tag_id, "openid_list": openids});
        svc.post(&tags::batch_untagging(config.as_ref()), &body.to_string())
            .await?;
        Ok(true)
    }
}
