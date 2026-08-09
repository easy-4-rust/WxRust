//! 对应 Java `service.impl.PartnerTransferServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{PartnerTransferService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// PartnerTransferService 实现（对应 Java `PartnerTransferServiceImpl`）。
pub struct PartnerTransferServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl PartnerTransferServiceImpl {
    /// 构建实现（对应 Java 构造器 `PartnerTransferServiceImpl(WxPayService)`）。
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
impl PartnerTransferService for PartnerTransferServiceImpl {
    async fn batch_transfer(
        &self,
        request: &PartnerTransferRequest,
    ) -> Result<PartnerTransferResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：transfer_detail_list 每项 user_name 加密、user_id_card 非空加密
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &[
                "transfer_detail_list.*.user_name",
                "transfer_detail_list.*.user_id_card",
            ],
        )?;
        let url = format!("{}/v3/partner-transfer/batches", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_batch_by_batch_id(
        &self,
        request: &BatchNumberRequest,
    ) -> Result<BatchNumberResult, WxErrorException> {
        let svc = self.svc()?;
        let mut request = request.clone();
        if request.offset.is_none() {
            request.offset = Some(0);
        }
        if request.limit.is_none() || request.limit.unwrap_or_default() <= 0 {
            request.limit = Some(20);
        }
        let detail_status = if request
            .detail_status
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            "ALL"
        } else {
            request.detail_status.as_deref().unwrap_or_default()
        };
        let url = format!(
            "{}/v3/partner-transfer/batches/batch-id/{}?need_query_detail={}&detail_status={detail_status}&offset={}&limit={}",
            svc.get_pay_base_url(),
            request.batch_id.as_deref().unwrap_or_default(),
            request.need_query_detail.unwrap_or_default(),
            request.offset.unwrap_or_default(),
            request.limit.unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_batch_by_out_batch_no(
        &self,
        request: &MerchantBatchRequest,
    ) -> Result<BatchNumberResult, WxErrorException> {
        let svc = self.svc()?;
        let mut request = request.clone();
        if request.offset.is_none() {
            request.offset = Some(0);
        }
        if request.limit.is_none() || request.limit.unwrap_or_default() <= 0 {
            request.limit = Some(20);
        }
        let mut url = format!(
            "{}/v3/partner-transfer/batches/out-batch-no/{}?need_query_detail={}&offset={}&limit={}",
            svc.get_pay_base_url(),
            request.out_batch_no.as_deref().unwrap_or_default(),
            request.need_query_detail.unwrap_or_default(),
            request.offset.unwrap_or_default(),
            request.limit.unwrap_or_default()
        );
        if let Some(v) = request.detail_status.as_deref() {
            if !v.trim().is_empty() {
                url.push_str(&format!("&detail_status={v}"));
            }
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_batch_detail_by_we_chat(
        &self,
        batch_id: &str,
        detail_id: &str,
    ) -> Result<BatchDetailsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/partner-transfer/batches/batch-id/{batch_id}/details/detail-id/{detail_id}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        let mut result: BatchDetailsResult =
            serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))?;
        self.decrypt_user_name(&mut result.user_name)?;
        Ok(result)
    }

    async fn query_batch_detail_by_mch(
        &self,
        out_batch_no: &str,
        out_detail_no: &str,
    ) -> Result<BatchDetailsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/partner-transfer/batches/out-batch-no/{out_batch_no}/details/out-detail-no/{out_detail_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        let mut result: BatchDetailsResult =
            serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))?;
        self.decrypt_user_name(&mut result.user_name)?;
        Ok(result)
    }

    async fn receipt_bill(
        &self,
        request: &ReceiptBillRequest,
    ) -> Result<BillReceiptResult, WxErrorException> {
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

    async fn query_bill_receipt(
        &self,
        out_bill_no: &str,
    ) -> Result<BillReceiptResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/elecsign/out-bill-no/{out_bill_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn transfer_electronic(
        &self,
        request: &ElectronicReceiptsRequest,
    ) -> Result<ElectronicReceiptsResult, WxErrorException> {
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

    async fn query_transfer_electronic_result(
        &self,
        request: &ElectronicReceiptsRequest,
    ) -> Result<ElectronicReceiptsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/transfer-detail/electronic-receipts?accept_type={}&out_batch_no={}&out_detail_no={}",
            svc.get_pay_base_url(),
            request.accept_type.as_deref().unwrap_or_default(),
            request.out_batch_no.as_deref().unwrap_or_default(),
            request.out_detail_no.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn transfer_download(&self, url: &str) -> Result<Vec<u8>, WxErrorException> {
        // 对应 Java `payService.downloadV3(url)`（账单文件下载）
        let svc = self.svc()?;
        svc.download_v3(url).await
    }

    async fn fund_balance(
        &self,
        account_type: SpAccountTypeEnum,
    ) -> Result<FundBalanceResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant/fund/balance/{}",
            svc.get_pay_base_url(),
            account_type.value()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sp_day_end_balance(
        &self,
        account_type: SpAccountTypeEnum,
        date: &str,
    ) -> Result<FundBalanceResult, WxErrorException> {
        // 对应 Java：委托 `getEcommerceService().spDayEndBalance`
        let svc = self.svc()?;
        let ecommerce = svc
            .ecommerce_service()
            .ok_or_else(|| impl_utils::runtime("ecommerceService 未装配"))?;
        ecommerce.sp_day_end_balance(account_type, date).await
    }
}

impl PartnerTransferServiceImpl {
    /// 解密明细中的用户姓名（对应 Java `RsaCryptoUtil.decryptOAEP(userName, config.getPrivateKey())`）。
    fn decrypt_user_name(&self, user_name: &mut Option<String>) -> Result<(), WxErrorException> {
        let Some(v) = user_name.as_deref() else {
            return Ok(());
        };
        if v.trim().is_empty() {
            return Ok(());
        }
        let svc = self.svc()?;
        let private_key = impl_utils::load_merchant_private_key(svc.wx_pay_config().as_ref())?;
        let decrypted =
            crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_decrypt(&private_key, v.trim())
                .map_err(|e| impl_utils::runtime(e.to_string()))?;
        *user_name = Some(decrypted);
        Ok(())
    }
}
