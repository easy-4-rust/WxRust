//! 子服务实例装配（Wave 5 P5 新增）。
//!
//! 对应 Java `WxPayServiceImpl` 中 29 个子服务的实例持有与
//! `getXxxService()` 装配：所有子服务实现持有 `Weak<dyn WxPayService>`
//! （避免 Arc 环泄漏，Java GC 等价物），由门面 `WxPayServiceImpl::new_arc`
//! 构建本包后经 getter 覆写返回。

use std::sync::Arc;
use std::sync::Weak;

use crate::api::r#impl::*;
use crate::api::*;

/// 子服务集合（对应 Java `WxPayServiceImpl` 内 29 个子服务字段）。
#[derive(Default)]
pub struct SubServiceBundle {
    pub apply4_subject_confirm: Option<Arc<dyn Apply4SubjectConfirmService>>,
    pub applyment4_sub: Option<Arc<dyn Applyment4SubService>>,
    pub bank: Option<Arc<dyn BankService>>,
    pub brand_merchant_transfer: Option<Arc<dyn BrandMerchantTransferService>>,
    pub business_circle: Option<Arc<dyn BusinessCircleService>>,
    pub business_operation_transfer: Option<Arc<dyn BusinessOperationTransferService>>,
    pub complaint: Option<Arc<dyn ComplaintService>>,
    pub custom_declaration: Option<Arc<dyn CustomDeclarationService>>,
    pub ecommerce: Option<Arc<dyn EcommerceService>>,
    pub ent_pay: Option<Arc<dyn EntPayService>>,
    pub marketing_busi_favor: Option<Arc<dyn MarketingBusiFavorService>>,
    pub marketing_favor: Option<Arc<dyn MarketingFavorService>>,
    pub marketing_media: Option<Arc<dyn MarketingMediaService>>,
    pub merchant_limitation: Option<Arc<dyn MerchantLimitationService>>,
    pub merchant_media: Option<Arc<dyn MerchantMediaService>>,
    pub merchant_transfer: Option<Arc<dyn MerchantTransferService>>,
    pub mi_pay: Option<Arc<dyn MiPayService>>,
    pub partner_pay_score: Option<Arc<dyn PartnerPayScoreService>>,
    pub partner_pay_score_sign_plan: Option<Arc<dyn PartnerPayScoreSignPlanService>>,
    pub partner_transfer: Option<Arc<dyn PartnerTransferService>>,
    pub pay_score: Option<Arc<dyn PayScoreService>>,
    pub payroll: Option<Arc<dyn PayrollService>>,
    pub profit_sharing: Option<Arc<dyn ProfitSharingService>>,
    pub real_name: Option<Arc<dyn RealNameService>>,
    pub redpack: Option<Arc<dyn RedpackService>>,
    pub subscription_billing: Option<Arc<dyn SubscriptionBillingService>>,
    pub transfer: Option<Arc<dyn TransferService>>,
    pub wx_deposit: Option<Arc<dyn WxDepositService>>,
    pub wx_entrust_pap: Option<Arc<dyn WxEntrustPapService>>,
}

impl SubServiceBundle {
    /// 以门面弱引用装配全部子服务（对应 Java 构造器 `setXxxService(new XxxServiceImpl(this))`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self {
            apply4_subject_confirm: Some(Arc::new(
                apply4_subject_confirm_service_impl::Apply4SubjectConfirmServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            applyment4_sub: Some(Arc::new(
                applyment4_sub_service_impl::Applyment4SubServiceImpl::new(pay_service.clone()),
            )),
            bank: Some(Arc::new(bank_service_impl::BankServiceImpl::new(
                pay_service.clone(),
            ))),
            brand_merchant_transfer: Some(Arc::new(
                brand_merchant_transfer_service_impl::BrandMerchantTransferServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            business_circle: Some(Arc::new(
                business_circle_service_impl::BusinessCircleServiceImpl::new(pay_service.clone()),
            )),
            business_operation_transfer: Some(Arc::new(
                business_operation_transfer_service_impl::BusinessOperationTransferServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            complaint: Some(Arc::new(complaint_service_impl::ComplaintServiceImpl::new(
                pay_service.clone(),
            ))),
            custom_declaration: Some(Arc::new(
                custom_declaration_service_impl::CustomDeclarationServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            ecommerce: Some(Arc::new(ecommerce_service_impl::EcommerceServiceImpl::new(
                pay_service.clone(),
            ))),
            ent_pay: Some(Arc::new(ent_pay_service_impl::EntPayServiceImpl::new(
                pay_service.clone(),
            ))),
            marketing_busi_favor: Some(Arc::new(
                marketing_busi_favor_service_impl::MarketingBusiFavorServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            marketing_favor: Some(Arc::new(
                marketing_favor_service_impl::MarketingFavorServiceImpl::new(pay_service.clone()),
            )),
            marketing_media: Some(Arc::new(
                marketing_media_service_impl::MarketingMediaServiceImpl::new(pay_service.clone()),
            )),
            merchant_limitation: Some(Arc::new(
                merchant_limitation_service_impl::MerchantLimitationServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            merchant_media: Some(Arc::new(
                merchant_media_service_impl::MerchantMediaServiceImpl::new(pay_service.clone()),
            )),
            merchant_transfer: Some(Arc::new(
                merchant_transfer_service_impl::MerchantTransferServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            mi_pay: Some(Arc::new(mi_pay_service_impl::MiPayServiceImpl::new(
                pay_service.clone(),
            ))),
            partner_pay_score: Some(Arc::new(
                partner_pay_score_service_impl::PartnerPayScoreServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            partner_pay_score_sign_plan: Some(Arc::new(
                partner_pay_score_sign_plan_service_impl::PartnerPayScoreSignPlanServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            partner_transfer: Some(Arc::new(
                partner_transfer_service_impl::PartnerTransferServiceImpl::new(pay_service.clone()),
            )),
            pay_score: Some(Arc::new(pay_score_service_impl::PayScoreServiceImpl::new(
                pay_service.clone(),
            ))),
            payroll: Some(Arc::new(payroll_service_impl::PayrollServiceImpl::new(
                pay_service.clone(),
            ))),
            profit_sharing: Some(Arc::new(
                profit_sharing_service_impl::ProfitSharingServiceImpl::new(pay_service.clone()),
            )),
            real_name: Some(Arc::new(real_name_service_impl::RealNameServiceImpl::new(
                pay_service.clone(),
            ))),
            redpack: Some(Arc::new(redpack_service_impl::RedpackServiceImpl::new(
                pay_service.clone(),
            ))),
            subscription_billing: Some(Arc::new(
                subscription_billing_service_impl::SubscriptionBillingServiceImpl::new(
                    pay_service.clone(),
                ),
            )),
            transfer: Some(Arc::new(transfer_service_impl::TransferServiceImpl::new(
                pay_service.clone(),
            ))),
            wx_deposit: Some(Arc::new(
                wx_deposit_service_impl::WxDepositServiceImpl::new(pay_service.clone()),
            )),
            wx_entrust_pap: Some(Arc::new(
                wx_entrust_pap_service_impl::WxEntrustPapServiceImpl::new(pay_service.clone()),
            )),
        }
    }
}
