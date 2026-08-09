//! WxMpDraftService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpDraftServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpDraftService, WxMpService};

use crate::bean::draft::{WxMpAddDraft, WxMpDraftInfo, WxMpDraftList, WxMpUpdateDraft};
use crate::enums::wx_mp_api_url::draft;

/// WxMpDraft服务实现。
pub struct WxMpDraftServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpDraftServiceImpl {
    /// 构建 WxMpDraft服务。
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
impl WxMpDraftService for WxMpDraftServiceImpl {
    async fn add_draft(&self, add_draft: &WxMpAddDraft) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(add_draft).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&draft::add(config.as_ref()), &body).await?;
        Self::extract_str(&response, "media_id")
    }

    async fn update_draft(&self, update_draft: &WxMpUpdateDraft) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::to_string(update_draft)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&draft::update(config.as_ref()), &body).await?;
        Self::err_code_is_zero(&response)
    }

    async fn get_draft(&self, media_id: &str) -> Result<WxMpDraftInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"media_id": media_id});
        let response = svc
            .post(&draft::get(config.as_ref()), &body.to_string())
            .await?;
        WxMpDraftInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn del_draft(&self, media_id: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"media_id": media_id});
        let response = svc
            .post(&draft::delete(config.as_ref()), &body.to_string())
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn list_draft(&self, offset: i32, count: i32) -> Result<WxMpDraftList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"offset": offset, "count": count, "no_content": 0});
        let response = svc
            .post(&draft::list(config.as_ref()), &body.to_string())
            .await?;
        WxMpDraftList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn count_draft(&self) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc.get(&draft::count(config.as_ref()), "").await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("total_count")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "total_count 缺失"))
    }
}
