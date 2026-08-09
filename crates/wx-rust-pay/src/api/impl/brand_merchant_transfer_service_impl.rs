//! 对应 Java `service.impl.BrandMerchantTransferServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{BrandMerchantTransferService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// BrandMerchantTransferService 实现（对应 Java `BrandMerchantTransferServiceImpl`）。
pub struct BrandMerchantTransferServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl BrandMerchantTransferServiceImpl {
    /// 构建实现（对应 Java 构造器 `BrandMerchantTransferServiceImpl(WxPayService)`）。
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
impl BrandMerchantTransferService for BrandMerchantTransferServiceImpl {
    async fn create_brand_transfer(
        &self,
        request: &BrandTransferBatchesRequest,
    ) -> Result<BrandTransferBatchesResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`：user_name + detail_list[*].user_name
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &["user_name", "detail_list.*.user_name"],
        )?;
        let url = format!(
            "{}/v3/fund-app/brand-redpacket/brand-merchant-batches",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_brand_wx_batches(
        &self,
        request: &BrandWxBatchesQueryRequest,
    ) -> Result<BrandBatchesQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/fund-app/brand-redpacket/brand-merchant-batches/{}",
            svc.get_pay_base_url(),
            request.batch_no.as_deref().unwrap_or_default()
        );
        if let Some(need) = request.need_query_detail {
            url.push_str(&format!("?need_query_detail={need}"));
        }
        if let Some(state) = request.detail_state.as_deref() {
            if !state.is_empty() {
                url.push_str(&format!("&detail_state={state}"));
            }
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_brand_wx_details(
        &self,
        request: &BrandWxDetailsQueryRequest,
    ) -> Result<BrandDetailsQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/brand-redpacket/brand-merchant-batches/{}/details/{}",
            svc.get_pay_base_url(),
            request.batch_no.as_deref().unwrap_or_default(),
            request.detail_no.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_brand_merchant_batches(
        &self,
        request: &BrandMerchantBatchesQueryRequest,
    ) -> Result<BrandBatchesQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/fund-app/brand-redpacket/brand-merchant-out-batches/{}",
            svc.get_pay_base_url(),
            request.out_batch_no.as_deref().unwrap_or_default()
        );
        if let Some(need) = request.need_query_detail {
            url.push_str(&format!("?need_query_detail={need}"));
        }
        if let Some(state) = request.detail_state.as_deref() {
            if !state.is_empty() {
                url.push_str(&format!("&detail_state={state}"));
            }
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_brand_merchant_details(
        &self,
        request: &BrandMerchantDetailsQueryRequest,
    ) -> Result<BrandDetailsQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/brand-redpacket/brand-merchant-out-batches/{}/out-details/{}",
            svc.get_pay_base_url(),
            request.out_batch_no.as_deref().unwrap_or_default(),
            request.out_detail_no.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
