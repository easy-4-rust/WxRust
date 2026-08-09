//! WxMpReimburseInvoiceService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpReimburseInvoiceServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpReimburseInvoiceService, WxMpService};
use crate::bean::invoice::reimburse::{
    InvoiceBatchRequest, InvoiceInfoRequest, InvoiceInfoResponse, UpdateInvoiceStatusRequest,
    UpdateStatusBatchRequest,
};
use crate::enums::wx_mp_api_url::reimburse_invoice;

/// 公众号ReimburseInvoiceService实现。
pub struct WxMpReimburseInvoiceServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpReimburseInvoiceServiceImpl {
    /// 构建 公众号ReimburseInvoiceService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpReimburseInvoiceService for WxMpReimburseInvoiceServiceImpl {
    async fn get_invoice_info(
        &self,
        request: &InvoiceInfoRequest,
    ) -> Result<InvoiceInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&reimburse_invoice::get_invoice_info(config.as_ref()), &body)
            .await?;
        InvoiceInfoResponse::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_invoice_batch(
        &self,
        request: &InvoiceBatchRequest,
    ) -> Result<Vec<InvoiceInfoResponse>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(
                &reimburse_invoice::get_invoice_batch(config.as_ref()),
                &body,
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("invoice_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "invoice_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn update_invoice_status(
        &self,
        request: &UpdateInvoiceStatusRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(
            &reimburse_invoice::update_invoice_status(config.as_ref()),
            &body,
        )
        .await?;
        Ok(())
    }

    async fn update_status_batch(
        &self,
        request: &UpdateStatusBatchRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(
            &reimburse_invoice::update_status_batch(config.as_ref()),
            &body,
        )
        .await?;
        Ok(())
    }
}
