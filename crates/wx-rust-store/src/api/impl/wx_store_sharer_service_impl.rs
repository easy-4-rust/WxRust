//! WxStoreSharerServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreSharerServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_sharer_service::WxStoreSharerService;
use crate::bean::sharer::{
    SharerBindResponse, SharerInfoResponse, SharerOrderParam, SharerOrderResponse,
    SharerSearchResponse, SharerUnbindParam, SharerUnbindResponse,
};
use crate::enums::url_sharer as url;

/// 构建 JSON 对象（跳过空值，对应 Java Jackson `JsonInclude.Include.NON_NULL`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 分享员服务实现。
pub struct WxStoreSharerServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreSharerServiceImpl {
    /// 构建分享员服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreSharerService for WxStoreSharerServiceImpl {
    /// 对应 Java `WxStoreSharerServiceImpl.bindSharer`：
    /// `{"username":".."}`（`GsonHelper.buildJsonObject`）后 POST
    /// `BIND_SHARER_URL`。
    async fn bind_sharer(&self, username: String) -> Result<SharerBindResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("username", serde_json::Value::String(username))]);
        let response = svc.post(url::BIND_SHARER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreSharerServiceImpl.searchSharer`：
    /// `SharerSearchParam`（空值跳过）后 POST `SEARCH_SHARER_URL`。
    async fn search_sharer(
        &self,
        openid: String,
        username: String,
    ) -> Result<SharerSearchResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "openid",
                if openid.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(openid)
                },
            ),
            (
                "username",
                if username.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(username)
                },
            ),
        ]);
        let response = svc.post(url::SEARCH_SHARER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreSharerServiceImpl.listSharer`：
    /// `SharerListParam`（空值跳过）后 POST `LIST_SHARER_URL`。
    async fn list_sharer(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        sharer_type: Option<i32>,
    ) -> Result<SharerInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "page",
                page.map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "sharer_type",
                sharer_type
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::LIST_SHARER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreSharerServiceImpl.listSharerOrder`：
    /// 序列化 `SharerOrderParam` 后 POST `LIST_SHARER_ORDER_URL`。
    async fn list_sharer_order(
        &self,
        param: SharerOrderParam,
    ) -> Result<SharerOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::LIST_SHARER_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreSharerServiceImpl.unbindSharer`：
    /// 序列化 `SharerUnbindParam`（key 为 `openid_list`）后 POST
    /// `UNBIND_SHARER_URL`。
    async fn unbind_sharer(
        &self,
        open_ids: Vec<String>,
    ) -> Result<SharerUnbindResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = SharerUnbindParam {
            open_ids: Some(open_ids),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UNBIND_SHARER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
