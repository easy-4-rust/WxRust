//! 微信支付 API 实现。
//!
//! 对应 Java `com.github.binarywang.wxpay.service.impl` 包：
//! `WxPayServiceImpl` 组合体 + 29 个子服务实现（每服务一个文件，
//! 对应 Java `service.impl.*ServiceImpl`；`SubServiceBundle` 承载门面装配）。

pub mod apply4_subject_confirm_service_impl;
pub mod applyment4_sub_service_impl;
pub mod bank_service_impl;
pub mod base_wx_pay_service_impl;
pub mod brand_merchant_transfer_service_impl;
pub mod business_circle_service_impl;
pub mod business_operation_transfer_service_impl;
pub mod complaint_service_impl;
pub mod custom_declaration_service_impl;
pub mod ecommerce_service_impl;
pub mod ent_pay_service_impl;
pub mod marketing_busi_favor_service_impl;
pub mod marketing_favor_service_impl;
pub mod marketing_media_service_impl;
pub mod merchant_limitation_service_impl;
pub mod merchant_media_service_impl;
pub mod merchant_transfer_service_impl;
pub mod mi_pay_service_impl;
pub mod partner_pay_score_service_impl;
pub mod partner_pay_score_sign_plan_service_impl;
pub mod partner_transfer_service_impl;
pub mod pay_score_service_impl;
pub mod payroll_service_impl;
pub mod profit_sharing_service_impl;
pub mod real_name_service_impl;
pub mod redpack_service_impl;
pub mod sub_service_bundle;
pub mod subscription_billing_service_impl;
pub mod transfer_service_impl;
pub mod wx_deposit_service_impl;
pub mod wx_entrust_pap_service_impl;

pub use base_wx_pay_service_impl::WxPayServiceImpl;
pub use sub_service_bundle::SubServiceBundle;
