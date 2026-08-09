//! 电子发票报销方服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaReimburseInvoiceService`
//! （`impl.WxMaReimburseInvoiceServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::invoice::{
    InvoiceBatchRequest, InvoiceInfoRequest, InvoiceInfoResponse, UpdateInvoiceStatusRequest,
    UpdateStatusBatchRequest,
};

/// 电子发票报销方服务。
///
/// 对应 Java `WxMaReimburseInvoiceService`：查询/批量查询报销发票信息、
/// 更新/批量更新发票状态。
#[async_trait]
pub trait WxMaReimburseInvoiceService: Send + Sync {
    /// 查询报销发票信息（对应 Java `getInvoiceInfo`）。
    async fn get_invoice_info(
        &self,
        request: &InvoiceInfoRequest,
    ) -> Result<InvoiceInfoResponse, WxErrorException>;

    /// 批量查询报销发票信息（对应 Java `getInvoiceBatch`）。
    async fn get_invoice_batch(
        &self,
        request: &InvoiceBatchRequest,
    ) -> Result<Vec<InvoiceInfoResponse>, WxErrorException>;

    /// 更新发票状态（对应 Java `updateInvoiceStatus`）。
    async fn update_invoice_status(
        &self,
        request: &UpdateInvoiceStatusRequest,
    ) -> Result<(), WxErrorException>;

    /// 批量更新发票状态（对应 Java `updateStatusBatch`）。
    async fn update_status_batch(
        &self,
        request: &UpdateStatusBatchRequest,
    ) -> Result<(), WxErrorException>;
}
