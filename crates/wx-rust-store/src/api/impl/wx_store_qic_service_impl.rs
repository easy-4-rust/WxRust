//! WxStoreQicServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreQicServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_qic_service::WxStoreQicService;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::qic::{
    InspectCodeResponse, InspectConfigResponse, RegisterLogisticsRequest, SubmitConfigResponse,
    SubmitInspectRequest,
};
use crate::enums::url_qic as url;

/// 质检管理服务实现。
pub struct WxStoreQicServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreQicServiceImpl {
    /// 构建质检管理服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreQicService for WxStoreQicServiceImpl {
    async fn get_inspect_config(&self) -> Result<InspectConfigResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.get(url::GET_INSPECT_CONFIG_URL, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_submit_config_with_order(
        &self,
        order_id: String,
    ) -> Result<SubmitConfigResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let query = if order_id.is_empty() {
            String::new()
        } else {
            format!("order_id={order_id}")
        };
        let response = svc.get(url::GET_SUBMIT_CONFIG_URL, &query).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_submit_config(&self) -> Result<SubmitConfigResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.get(url::GET_SUBMIT_CONFIG_URL, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn print_inspect_code(
        &self,
        order_id: String,
    ) -> Result<InspectCodeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"order_id": order_id}).to_string();
        let response = svc.post(url::PRINT_INSPECT_CODE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn submit_inspect_info(
        &self,
        request: SubmitInspectRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SUBMIT_INSPECT_INFO_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn register_logistics(
        &self,
        request: RegisterLogisticsRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::REGISTER_LOGISTICS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
