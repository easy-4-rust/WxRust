//! 对应 Java `service.impl.MerchantTransferServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{MerchantTransferService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// MerchantTransferService 实现（对应 Java `MerchantTransferServiceImpl`）。
pub struct MerchantTransferServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl MerchantTransferServiceImpl {
    /// 构建实现（对应 Java 构造器 `MerchantTransferServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用（对应 Java `this.payService` 直接使用）。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }
}

#[async_trait]
impl MerchantTransferService for MerchantTransferServiceImpl {
    async fn create_transfer(
        &self,
        request: &TransferCreateRequest,
    ) -> Result<TransferCreateResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：appid 为空时从配置补齐
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        let mut json =
            serde_json::to_value(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`：user_name（TransferCreateRequest
        // 的 @SpecEncrypt 字段为 user_name）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["user_name"])?;
        let url = format!("{}/v3/transfer/batches", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_wx_batches(
        &self,
        request: &WxBatchesQueryRequest,
    ) -> Result<BatchesQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/transfer/batches/batch-id/{}?need_query_detail={}",
            svc.get_pay_base_url(),
            request.batch_id.as_deref().unwrap_or_default(),
            request.need_query_detail.unwrap_or_default()
        );
        if let Some(v) = request.offset {
            url.push_str(&format!("&offset={v}"));
        }
        if let Some(v) = request.limit {
            url.push_str(&format!("&limit={v}"));
        }
        if let Some(v) = request.detail_status.as_deref() {
            if !v.is_empty() {
                url.push_str(&format!("&detail_status={v}"));
            }
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_wx_details(
        &self,
        request: &WxDetailsQueryRequest,
    ) -> Result<DetailsQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/transfer/batches/batch-id/{}/details/detail-id/{}",
            svc.get_pay_base_url(),
            request.batch_id.as_deref().unwrap_or_default(),
            request.detail_id.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_merchant_batches(
        &self,
        request: &MerchantBatchesQueryRequest,
    ) -> Result<BatchesQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/transfer/batches/out-batch-no/{}?need_query_detail={}",
            svc.get_pay_base_url(),
            request.out_batch_no.as_deref().unwrap_or_default(),
            request.need_query_detail.unwrap_or_default()
        );
        if let Some(v) = request.offset {
            url.push_str(&format!("&offset={v}"));
        }
        if let Some(v) = request.limit {
            url.push_str(&format!("&limit={v}"));
        }
        if let Some(v) = request.detail_status.as_deref() {
            if !v.is_empty() {
                url.push_str(&format!("&detail_status={v}"));
            }
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_merchant_details(
        &self,
        request: &MerchantDetailsQueryRequest,
    ) -> Result<DetailsQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/transfer/batches/out-batch-no/{}/details/out-detail-no/{}",
            svc.get_pay_base_url(),
            request.out_batch_no.as_deref().unwrap_or_default(),
            request.out_detail_no.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn apply_electronic_bill(
        &self,
        request: &ElectronicBillApplyRequest,
    ) -> Result<ElectronicBillResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/elecsign/out-bill-no",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_electronic_bill(
        &self,
        out_bill_no: &str,
    ) -> Result<ElectronicBillResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/elecsign/out-bill-no/{out_bill_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn apply_detail_electronic_bill(
        &self,
        request: &DetailElectronicBillRequest,
    ) -> Result<DetailElectronicBillResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/transfer-detail/electronic-receipts",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_detail_electronic_bill(
        &self,
        request: &DetailElectronicBillRequest,
    ) -> Result<DetailElectronicBillResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/transfer-detail/electronic-receipts?accept_type={}&out_detail_no={}",
            svc.get_pay_base_url(),
            request.accept_type.as_deref().unwrap_or_default(),
            request.out_detail_no.as_deref().unwrap_or_default()
        );
        if let Some(v) = request.out_batch_no.as_deref() {
            if !v.trim().is_empty() {
                url.push_str(&format!("&out_batch_no={v}"));
            }
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_user_authorization_status(
        &self,
        openid: &str,
        transfer_scene_id: &str,
    ) -> Result<UserAuthorizationStatusResult, WxErrorException> {
        // 对应 Java：委托 `wxPayService.getTransferService()`
        let svc = self.svc()?;
        let transfer = svc
            .transfer_service()
            .ok_or_else(|| impl_utils::runtime("transferService 未装配"))?;
        transfer
            .get_user_authorization_status(openid, transfer_scene_id)
            .await
    }

    async fn reservation_transfer_batch(
        &self,
        request: &ReservationTransferBatchRequest,
    ) -> Result<ReservationTransferBatchResult, WxErrorException> {
        let svc = self.svc()?;
        let transfer = svc
            .transfer_service()
            .ok_or_else(|| impl_utils::runtime("transferService 未装配"))?;
        transfer.reservation_transfer_batch(request).await
    }

    async fn get_reservation_transfer_batch_by_out_batch_no(
        &self,
        out_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException> {
        let svc = self.svc()?;
        let transfer = svc
            .transfer_service()
            .ok_or_else(|| impl_utils::runtime("transferService 未装配"))?;
        transfer
            .get_reservation_transfer_batch_by_out_batch_no(
                out_batch_no,
                need_query_detail,
                offset,
                limit,
                detail_state,
            )
            .await
    }

    async fn get_reservation_transfer_batch_by_reservation_batch_no(
        &self,
        reservation_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException> {
        let svc = self.svc()?;
        let transfer = svc
            .transfer_service()
            .ok_or_else(|| impl_utils::runtime("transferService 未装配"))?;
        transfer
            .get_reservation_transfer_batch_by_reservation_batch_no(
                reservation_batch_no,
                need_query_detail,
                offset,
                limit,
                detail_state,
            )
            .await
    }

    async fn parse_reservation_transfer_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<ReservationTransferNotifyResult, WxErrorException> {
        let svc = self.svc()?;
        let transfer = svc
            .transfer_service()
            .ok_or_else(|| impl_utils::runtime("transferService 未装配"))?;
        transfer
            .parse_reservation_transfer_notify_result(notify_data, header)
            .await
    }

    async fn close_reservation_transfer_batch(
        &self,
        out_batch_no: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let transfer = svc
            .transfer_service()
            .ok_or_else(|| impl_utils::runtime("transferService 未装配"))?;
        transfer
            .close_reservation_transfer_batch(out_batch_no)
            .await
    }
}
