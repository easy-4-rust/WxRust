//! 电子发票报销方服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaReimburseInvoiceServiceImpl`：
//! `getInvoiceInfo` 经 `InvoiceInfoResponse.fromJson` 解析；
//! `getInvoiceBatch` 经 Java `InvoiceInfoResponse.toList` 语义解析
//! （取响应 `item_list` 数组）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaReimburseInvoiceService;
use crate::bean::invoice::{
    InvoiceBatchRequest, InvoiceInfoRequest, InvoiceInfoResponse, UpdateInvoiceStatusRequest,
    UpdateStatusBatchRequest,
};
use crate::enums::g4_urls::url_g4_ability::invoice as invoice_url;

/// 电子发票报销方服务实现。
pub struct WxMaReimburseInvoiceServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaReimburseInvoiceServiceImpl {
    /// 构建电子发票报销方服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 序列化请求对象为 JSON（对应 Java `request.toJson()`）。
    fn to_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(request).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxMaReimburseInvoiceService for WxMaReimburseInvoiceServiceImpl {
    /// 查询报销发票信息（对应 Java
    /// `WxMaReimburseInvoiceServiceImpl.getInvoiceInfo`）。
    async fn get_invoice_info(
        &self,
        request: &InvoiceInfoRequest,
    ) -> Result<InvoiceInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &invoice_url::get_invoice_info_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        InvoiceInfoResponse::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    /// 批量查询报销发票信息（对应 Java
    /// `WxMaReimburseInvoiceServiceImpl.getInvoiceBatch`）。
    ///
    /// 响应取 `item_list` 数组（Java `InvoiceInfoResponse.toList`）。
    async fn get_invoice_batch(
        &self,
        request: &InvoiceBatchRequest,
    ) -> Result<Vec<InvoiceInfoResponse>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &invoice_url::get_invoice_batch_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        let item_list = json
            .get("item_list")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));
        serde_json::from_value(item_list).map_err(WxErrorException::from)
    }

    /// 更新发票状态（对应 Java
    /// `WxMaReimburseInvoiceServiceImpl.updateInvoiceStatus`）。
    async fn update_invoice_status(
        &self,
        request: &UpdateInvoiceStatusRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        svc.post(
            &invoice_url::update_invoice_status_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await?;
        Ok(())
    }

    /// 批量更新发票状态（对应 Java
    /// `WxMaReimburseInvoiceServiceImpl.updateStatusBatch`）。
    async fn update_status_batch(
        &self,
        request: &UpdateStatusBatchRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        svc.post(
            &invoice_url::update_status_batch_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await?;
        Ok(())
    }
}
