//! WxStoreEwaybillServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreEwaybillServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_ewaybill_service::WxStoreEwaybillService;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::ewaybill::{
    AccountInfoResponse, AddSubOrderRequest, BatchPrintOrderRequest, CreateOrderRequest,
    CreateOrderResponse, DeliveryListResponse, OrderDetailResponse, PreCreateRequest,
    PreCreateResponse, PrintContentResponse, PrintOrderRequest, TemplateConfigResponse,
    TemplateCreateRequest, TemplateIdResponse, TemplateInfoResponse, TemplateUpdateRequest,
};
use crate::enums::url_ewaybill as url;

/// 电子面单服务实现。
pub struct WxStoreEwaybillServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreEwaybillServiceImpl {
    /// 构建电子面单服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreEwaybillService for WxStoreEwaybillServiceImpl {
    async fn get_template_config(&self) -> Result<TemplateConfigResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.post(url::GET_TEMPLATE_CONFIG_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn create_template(
        &self,
        req: TemplateCreateRequest,
    ) -> Result<TemplateIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::CREATE_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn delete_template(
        &self,
        template_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"template_id": template_id}).to_string();
        let response = svc.post(url::DELETE_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn update_template(
        &self,
        req: TemplateUpdateRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_template(
        &self,
        template_code: String,
    ) -> Result<TemplateInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"template_code": template_code}).to_string();
        let response = svc.post(url::GET_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_template_by_id(
        &self,
        template_id: String,
    ) -> Result<TemplateInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"template_id": template_id}).to_string();
        let response = svc.post(url::GET_TEMPLATE_BY_ID_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_account(&self) -> Result<AccountInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.post(url::GET_ACCOUNT_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_delivery_list(&self) -> Result<DeliveryListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.post(url::GET_DELIVERY_LIST_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn pre_create_order(
        &self,
        req: PreCreateRequest,
    ) -> Result<PreCreateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::PRE_CREATE_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn create_order(
        &self,
        req: CreateOrderRequest,
    ) -> Result<CreateOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::CREATE_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn add_sub_order(
        &self,
        req: AddSubOrderRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_SUB_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn cancel_order(
        &self,
        req: PrintOrderRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::CANCEL_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_order(
        &self,
        ewaybill_order_id: String,
    ) -> Result<OrderDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"ewaybill_order_id": ewaybill_order_id}).to_string();
        let response = svc.post(url::GET_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_print_content(
        &self,
        ewaybill_order_id: String,
        template_id: String,
    ) -> Result<PrintContentResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({
            "ewaybill_order_id": ewaybill_order_id,
            "template_id": template_id
        })
        .to_string();
        let response = svc.post(url::GET_PRINT_CONTENT_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn print_order(
        &self,
        req: PrintOrderRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::PRINT_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn batch_print_order(
        &self,
        req: BatchPrintOrderRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::BATCH_PRINT_ORDER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
