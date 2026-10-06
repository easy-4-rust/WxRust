//! WxStoreProductAssistantServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreProductAssistantServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_product_assistant_service::WxStoreProductAssistantService;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::product::assistant::{
    BeginTimingSaleParam, CancelTimingSaleParam, CategoryPreCheckParam, CategoryPreCheckResponse,
    ExternalProductMappingNewParam, ExternalProductMappingNewResponse, ExternalProductMappingParam,
    ExternalProductMappingResponse, ProductBrandRecommendParam, ProductBrandRecommendResponse,
};
use crate::enums::url_product_assistant as url;

/// 商品辅助功能服务实现。
pub struct WxStoreProductAssistantServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreProductAssistantServiceImpl {
    /// 构建商品辅助功能服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreProductAssistantService for WxStoreProductAssistantServiceImpl {
    async fn category_pre_check(
        &self,
        param: CategoryPreCheckParam,
    ) -> Result<CategoryPreCheckResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::CATEGORY_PRE_CHECK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_product_brand_recommend(
        &self,
        param: ProductBrandRecommendParam,
    ) -> Result<ProductBrandRecommendResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::PRODUCT_BRAND_RECOMMEND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn external_product_mapping(
        &self,
        param: ExternalProductMappingParam,
    ) -> Result<ExternalProductMappingResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::EXTERNAL_PRODUCT_MAPPING_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn external_product_mapping_new(
        &self,
        param: ExternalProductMappingNewParam,
    ) -> Result<ExternalProductMappingNewResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::EXTERNAL_PRODUCT_MAPPING_NEW_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn begin_timing_sale(
        &self,
        param: BeginTimingSaleParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::BEGIN_TIMING_SALE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn cancel_timing_sale(
        &self,
        param: CancelTimingSaleParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::CANCEL_TIMING_SALE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
