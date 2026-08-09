//! 对应 Java `service.impl.Apply4SubjectConfirmServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{Apply4SubjectConfirmService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// Apply4SubjectConfirmService 实现（对应 Java `Apply4SubjectConfirmServiceImpl`）。
pub struct Apply4SubjectConfirmServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl Apply4SubjectConfirmServiceImpl {
    /// 构建实现（对应 Java 构造器 `Apply4SubjectConfirmServiceImpl(WxPayService)`）。
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
impl Apply4SubjectConfirmService for Apply4SubjectConfirmServiceImpl {
    async fn applyment(
        &self,
        request: &ApplySubjectConfirmCreateRequest,
    ) -> Result<ApplySubjectConfirmCreateResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`（@SpecEncrypt 字段）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &[
                "contact_info.name",
                "contact_info.contact_id_number",
                "contact_info.mobile",
                "subject_info.identity_info.identification_name",
                "subject_info.identity_info.identification_number",
                "subject_info.identity_info.identification_address",
                "subject_info.ubo_info_list.*.ubo_id_doc_name",
                "subject_info.ubo_info_list.*.ubo_id_doc_number",
                "subject_info.ubo_info_list.*.ubo_id_doc_address",
            ],
        )?;
        let url = format!("{}/v3/apply4subject/applyment", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_apply_status_by_business_code(
        &self,
        business_code: &str,
    ) -> Result<ApplySubjectConfirmStateQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/apply4subject/applyment?business_code={business_code}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_apply_status_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<ApplySubjectConfirmStateQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/apply4subject/applyment?applyment_id={applyment_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_merchant_apply_status_by_mch_id(
        &self,
        sub_mch_id: &str,
    ) -> Result<ApplySubjectConfirmMerchantStateQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/apply4subject/applyment/merchants/{sub_mch_id}/state",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn cancel_apply_by_business_code(
        &self,
        business_code: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/apply4subject/applyment/{business_code}/cancel",
            svc.get_pay_base_url()
        );
        svc.post_v3_with_wechatpay_serial(&url, "").await?;
        Ok(())
    }

    async fn cancel_apply_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/apply4subject/applyment/{applyment_id}/cancel",
            svc.get_pay_base_url()
        );
        svc.post_v3_with_wechatpay_serial(&url, "").await?;
        Ok(())
    }
}
