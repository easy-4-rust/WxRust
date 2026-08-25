//! WxChannelLimitedDiscountServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelLimitedDiscountServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_limited_discount_service::WxChannelLimitedDiscountService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::limit::{
    LimitTaskAddResponse, LimitTaskListResponse, LimitTaskParam, LimitTaskUpdateParam,
    LimitTaskUpdateResponse,
};
use crate::enums::url_limited_discount as url;

/// 限时抢购服务实现。
pub struct WxChannelLimitedDiscountServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelLimitedDiscountServiceImpl {
    /// 构建限时抢购服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

#[async_trait]
impl WxChannelLimitedDiscountService for WxChannelLimitedDiscountServiceImpl {
    async fn add_limit_task(
        &self,
        param: LimitTaskParam,
    ) -> Result<LimitTaskAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn list_limit_task(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<LimitTaskListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "next_key",
                if next_key.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(next_key)
                },
            ),
            (
                "status",
                status
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::LIST_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn stop_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"task_id": task_id}).to_string();
        let response = svc.post(url::STOP_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn delete_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"task_id": task_id}).to_string();
        let response = svc.post(url::DELETE_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn update_limit_task(
        &self,
        param: LimitTaskUpdateParam,
    ) -> Result<LimitTaskUpdateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
