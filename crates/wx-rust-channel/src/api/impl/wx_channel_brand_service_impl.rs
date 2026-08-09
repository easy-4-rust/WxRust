//! WxChannelBrandServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelBrandServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_brand_service::WxChannelBrandService;
use crate::bean::audit::AuditApplyResponse;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::brand::{
    Brand, BrandApplyListResponse, BrandInfoResponse, BrandListResponse, BrandParam,
};
use crate::enums::url_brand as url;

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

/// 品牌服务实现。
pub struct WxChannelBrandServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelBrandServiceImpl {
    /// 构建品牌服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelBrandService for WxChannelBrandServiceImpl {
    /// 对应 Java `WxChannelBrandServiceImpl.listAllBrand`：
    /// `StreamPageParam`（空值跳过）后 POST `ALL_BRAND_URL`。
    async fn list_all_brand(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<BrandListResponse, WxErrorException> {
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
        ]);
        let response = svc.post(url::ALL_BRAND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBrandServiceImpl.addBrandApply`：
    /// 序列化 `BrandParam`（包裹 `Brand`）后 POST `ADD_BRAND_URL`。
    async fn add_brand_apply(&self, brand: Brand) -> Result<AuditApplyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = BrandParam { brand };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_BRAND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBrandServiceImpl.updateBrandApply`：
    /// 序列化 `BrandParam` 后 POST `UPDATE_BRAND_URL`。
    async fn update_brand_apply(
        &self,
        brand: Brand,
    ) -> Result<AuditApplyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = BrandParam { brand };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_BRAND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBrandServiceImpl.cancelBrandApply`：
    /// `{"brand_id":"..","audit_id":".."}` 后 POST `CANCEL_BRAND_AUDIT_URL`。
    async fn cancel_brand_apply(
        &self,
        brand_id: String,
        audit_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            ("brand_id", serde_json::Value::String(brand_id)),
            ("audit_id", serde_json::Value::String(audit_id)),
        ]);
        let response = svc.post(url::CANCEL_BRAND_AUDIT_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBrandServiceImpl.deleteBrandApply`：
    /// `{"brand_id":".."}` 后 POST `DELETE_BRAND_URL`。
    async fn delete_brand_apply(
        &self,
        brand_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("brand_id", serde_json::Value::String(brand_id))]);
        let response = svc.post(url::DELETE_BRAND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBrandServiceImpl.getBrandApply`：
    /// `{"brand_id":".."}` 后 POST `GET_BRAND_URL`。
    async fn get_brand_apply(
        &self,
        brand_id: String,
    ) -> Result<BrandInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("brand_id", serde_json::Value::String(brand_id))]);
        let response = svc.post(url::GET_BRAND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBrandServiceImpl.listBrandApply`：
    /// `BrandSearchParam`（空值跳过）后 POST `LIST_BRAND_URL`。
    async fn list_brand_apply(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<BrandApplyListResponse, WxErrorException> {
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
        let response = svc.post(url::LIST_BRAND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBrandServiceImpl.listValidBrandApply`：
    /// `StreamPageParam`（空值跳过）后 POST `LIST_BRAND_VALID_URL`。
    async fn list_valid_brand_apply(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<BrandApplyListResponse, WxErrorException> {
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
        ]);
        let response = svc.post(url::LIST_BRAND_VALID_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
