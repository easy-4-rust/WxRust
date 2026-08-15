//! WxMpReimburseInvoiceService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpReimburseInvoiceService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::invoice::reimburse::{
    InvoiceBatchRequest, InvoiceInfoRequest, InvoiceInfoResponse, UpdateInvoiceStatusRequest,
    UpdateStatusBatchRequest,
};

/// 公众号ReimburseInvoiceService。
#[async_trait]
pub trait WxMpReimburseInvoiceService: Send + Sync {
    async fn get_invoice_info(
        &self,
        request: &InvoiceInfoRequest,
    ) -> Result<InvoiceInfoResponse, WxErrorException>;

    async fn get_invoice_batch(
        &self,
        request: &InvoiceBatchRequest,
    ) -> Result<Vec<InvoiceInfoResponse>, WxErrorException>;

    async fn update_invoice_status(
        &self,
        request: &UpdateInvoiceStatusRequest,
    ) -> Result<(), WxErrorException>;

    async fn update_status_batch(
        &self,
        request: &UpdateStatusBatchRequest,
    ) -> Result<(), WxErrorException>;
}
