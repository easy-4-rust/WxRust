//! WxChannelSupplierServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelSupplierServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_supplier_service::WxChannelSupplierService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::supplier::{
    DistributeTypeResponse, DropshipAssignRequest, DropshipDetailResponse, DropshipListRequest,
    DropshipListResponse, DropshipResponse, DropshipSearchRequest, ProductDistributeRequest,
    ProductListResponse, SupplierInfoResponse, SupplierListResponse,
};
use crate::enums::url_supplier as url;

/// 代发管理服务实现。
pub struct WxChannelSupplierServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelSupplierServiceImpl {
    /// 构建代发管理服务。
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
impl WxChannelSupplierService for WxChannelSupplierServiceImpl {
    async fn get_supplier_list_default(&self) -> Result<SupplierListResponse, WxErrorException> {
        self.get_supplier_list(None, String::new()).await
    }

    async fn get_supplier_list(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<SupplierListResponse, WxErrorException> {
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
        let response = svc.post(url::GET_SUPPLIER_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_distribute(&self) -> Result<DistributeTypeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.post(url::GET_DISTRIBUTE_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_manually_distribute(&self) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.post(url::SET_MANUALLY_DISTRIBUTE_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_all_distribute(
        &self,
        supplier_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"supplier_id": supplier_id}).to_string();
        let response = svc.post(url::SET_ALL_DISTRIBUTION_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_product_distribute(
        &self,
        req: ProductDistributeRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SET_PRODUCT_DISTRIBUTE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_product_default_distribute(
        &self,
        product_id: String,
    ) -> Result<SupplierInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"product_id": product_id}).to_string();
        let response = svc
            .post(url::GET_PRODUCT_DEFAULT_DISTRIBUTE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_product_list(
        &self,
        supplier_id: String,
    ) -> Result<ProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"supplier_id": supplier_id}).to_string();
        let response = svc.post(url::GET_PRODUCT_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn assign_order(
        &self,
        req: DropshipAssignRequest,
    ) -> Result<DropshipResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ASSIGN_DROPSHIP_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn cancel_dropship(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"order_id": order_id}).to_string();
        let response = svc.post(url::CANCEL_DROPSHIP_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_dropship(
        &self,
        order_id: String,
    ) -> Result<DropshipDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"order_id": order_id}).to_string();
        let response = svc.post(url::GET_DROPSHIP_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn list_dropship(
        &self,
        req: DropshipListRequest,
    ) -> Result<DropshipListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_DROPSHIP_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn search_dropship(
        &self,
        req: DropshipSearchRequest,
    ) -> Result<DropshipListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SEARCH_DROPSHIP_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
