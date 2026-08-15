//! 对应 Java `service.impl.PartnerPayScoreSignPlanServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{PartnerPayScoreSignPlanService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// PartnerPayScoreSignPlanService 实现（对应 Java `PartnerPayScoreSignPlanServiceImpl`）。
pub struct PartnerPayScoreSignPlanServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl PartnerPayScoreSignPlanServiceImpl {
    /// 构建实现（对应 Java 构造器 `PartnerPayScoreSignPlanServiceImpl(WxPayService)`）。
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
impl PartnerPayScoreSignPlanService for PartnerPayScoreSignPlanServiceImpl {
    async fn create_plans(
        &self,
        request: &WxPartnerPayScoreSignPlanRequest,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        if request
            .service_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.service_id = config.service_id().map(str::to_string);
        }
        let url = format!(
            "{}/v3/payscore/plan/partner/payscore-plans",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPartnerPayScoreSignPlanResult::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_plans(
        &self,
        merchant_plan_no: &str,
        sub_mchid: &str,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payscore/plan/partner/payscore-plans/merchant-plan-no/{merchant_plan_no}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        WxPartnerPayScoreSignPlanResult::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn stop_plans(
        &self,
        merchant_plan_no: &str,
        sub_mchid: &str,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payscore/plan/partner/payscore-plans/merchant-plan-no/{merchant_plan_no}/stop",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({ "sub_mchid": sub_mchid });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPartnerPayScoreSignPlanResult::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sign_plan_service_order(
        &self,
        request: &WxPartnerPayScoreSignPlanRequest,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        if request
            .service_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.service_id = config.service_id().map(str::to_string);
        }
        let url = format!(
            "{}/v3/payscore/sign-plan/partner/serviceorder",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPartnerPayScoreSignPlanResult::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn create_user_sign_plans(
        &self,
        request: &WxPartnerPayScoreSignPlanRequest,
    ) -> Result<WxPartnerPayScoreUserSignPlanResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        if request
            .service_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.service_id = config.service_id().map(str::to_string);
        }
        let url = format!(
            "{}/v3/payscore/sign-plan/partner/user-sign-plans",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPartnerPayScoreUserSignPlanResult::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_user_sign_plans(
        &self,
        merchant_sign_plan_no: &str,
        sub_mchid: &str,
    ) -> Result<PartnerUserSignPlanEntity, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payscore/sign-plan/partner/user-sign-plans/merchant-sign-plan-no/{merchant_sign_plan_no}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        PartnerUserSignPlanEntity::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn stop_user_sign_plans(
        &self,
        merchant_sign_plan_no: &str,
        sub_mchid: &str,
        stop_reason: &str,
    ) -> Result<PartnerUserSignPlanEntity, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payscore/sign-plan/partner/user-sign-plans/merchant-sign-plan-no/{merchant_sign_plan_no}/stop",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "sub_mchid": sub_mchid,
            "stop_reason": stop_reason,
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        PartnerUserSignPlanEntity::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_sign_plan_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<PartnerUserSignPlanEntity, WxErrorException> {
        // 对应 Java：parseNotifyData（验签）→ decryptNotifyDataResource（AES-GCM）
        let response = self.parse_notify_data(notify_data, header).await?;
        let resource = response
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        let decrypted = self.decrypt_resource(resource)?;
        PartnerUserSignPlanEntity::from_json(&decrypted)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

impl PartnerPayScoreSignPlanServiceImpl {
    /// 验签解析通知（对应 Java `parseNotifyData`）。
    async fn parse_notify_data(
        &self,
        data: &str,
        header: &SignatureHeader,
    ) -> Result<PayScoreNotifyData, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        crate::util::wx_pay_notify_utils::verify_notify_signature(&public_key, header, data)?;
        serde_json::from_str(data).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    /// AES-GCM 解密通知 resource（对应 Java `AesUtils.decryptToString`）。
    fn decrypt_resource(
        &self,
        resource: &crate::bean::payscore::pay_score_notify_data::Resource,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        crate::util::crypto::wx_pay_v3_crypto_utils::aes_gcm_decrypt(
            api_v3_key,
            resource.associated_data.as_deref().unwrap_or_default(),
            resource.nonce.as_deref().unwrap_or_default(),
            resource.cipher_text.as_deref().unwrap_or_default(),
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))
    }
}
