//! 对应 Java `service.impl.Applyment4SubServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{Applyment4SubService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// Applyment4SubService 实现（对应 Java `Applyment4SubServiceImpl`）。
pub struct Applyment4SubServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl Applyment4SubServiceImpl {
    /// 构建实现（对应 Java 构造器 `Applyment4SubServiceImpl(WxPayService)`）。
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
impl Applyment4SubService for Applyment4SubServiceImpl {
    async fn create_apply(
        &self,
        request: &WxPayApplyment4SubCreateRequest,
    ) -> Result<WxPayApplymentCreateResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`（@SpecEncrypt 字段，JSON 路径镜像嵌套类）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &[
                "contact_info.contact_name",
                "contact_info.contact_id_number",
                "contact_info.mobile_phone",
                "contact_info.contact_email",
                "subject_info.identity_info.id_card_info.id_card_name",
                "subject_info.identity_info.id_card_info.id_card_number",
                "subject_info.identity_info.id_card_info.id_card_address",
                "subject_info.identity_info.id_doc_info.id_doc_name",
                "subject_info.identity_info.id_doc_info.id_doc_number",
                "subject_info.identity_info.id_doc_info.id_doc_address",
                "subject_info.ubo_info_list.*.ubo_id_doc_name",
                "subject_info.ubo_info_list.*.ubo_id_doc_number",
                "subject_info.ubo_info_list.*.ubo_id_doc_address",
                "bank_account_info.account_name",
                "bank_account_info.account_number",
            ],
        )?;
        let url = format!("{}/v3/applyment4sub/applyment/", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_apply_status_by_business_code(
        &self,
        business_code: &str,
    ) -> Result<ApplymentStateQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/applyment4sub/applyment/business_code/{business_code}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_apply_status_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<ApplymentStateQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/applyment4sub/applyment/applyment_id/{applyment_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_settlement_by_sub_mchid(
        &self,
        sub_mchid: &str,
    ) -> Result<SettlementInfoResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/apply4sub/sub_merchants/{sub_mchid}/settlement",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn modify_settlement(
        &self,
        sub_mchid: &str,
        request: &ModifySettlementRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`：account_name/account_number
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &["account_name", "account_number"],
        )?;
        let url = format!(
            "{}/v3/apply4sub/sub_merchants/{sub_mchid}/modify-settlement",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3_with_wechatpay_serial(&url, &body).await
    }

    async fn query_settlement_modify_status_by_application_no(
        &self,
        sub_mchid: &str,
        application_no: &str,
    ) -> Result<SettlementModifyStateQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/apply4sub/sub_merchants/{sub_mchid}/application/{application_no}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
