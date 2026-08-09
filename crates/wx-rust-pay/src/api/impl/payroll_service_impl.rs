//! 对应 Java `service.impl.PayrollServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{PayrollService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// PayrollService 实现（对应 Java `PayrollServiceImpl`）。
pub struct PayrollServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl PayrollServiceImpl {
    /// 构建实现（对应 Java 构造器 `PayrollServiceImpl(WxPayService)`）。
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
impl PayrollService for PayrollServiceImpl {
    async fn payroll_card_tokens(
        &self,
        request: &TokensRequest,
    ) -> Result<TokensResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：user_name/id_card_number RSA-OAEP 加密
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        if let Some(v) = request.user_name.as_deref() {
            if !v.trim().is_empty() {
                request.user_name = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        if let Some(v) = request.id_card_number.as_deref() {
            if !v.trim().is_empty() {
                request.id_card_number = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        let url = format!("{}/v3/payroll-card/tokens", svc.get_pay_base_url());
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn payroll_card_relations(
        &self,
        request: &RelationsRequest,
    ) -> Result<RelationsResult, WxErrorException> {
        let svc = self.svc()?;
        let mut query = format!(
            "?sub_mchid={}",
            request.sub_mchid.as_deref().unwrap_or_default()
        );
        if let Some(v) = request.appid.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&appid={v}"));
            }
        }
        if let Some(v) = request.sub_appid.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&sub_appid={v}"));
            }
        }
        let url = format!(
            "{}/v3/payroll-card/relations/{}{query}",
            svc.get_pay_base_url(),
            request.openid.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn payroll_card_pre_order(
        &self,
        request: &PreOrderRequest,
    ) -> Result<PreOrderResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payroll-card/authentications/pre-order",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn payroll_card_authentications_number(
        &self,
        sub_mchid: &str,
        authenticate_number: &str,
    ) -> Result<AuthenticationsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payroll-card/authentications/{authenticate_number}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn payroll_card_authentications(
        &self,
        request: &AuthRecordRequest,
    ) -> Result<AuthRecordResult, WxErrorException> {
        let svc = self.svc()?;
        // 对应 Java（原样镜像其查询串拼接）：openid/sub_mchid/authenticate_date
        // 恒拼接，appid/sub_appid/authenticate_state 非空追加
        let mut query = format!(
            "?openid={}&sub_mchid={}&authenticate_date={}",
            request.openid.as_deref().unwrap_or_default(),
            request.sub_mchid.as_deref().unwrap_or_default(),
            request.authenticate_date.as_deref().unwrap_or_default()
        );
        if let Some(v) = request.appid.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&appid={v}"));
            }
        }
        if let Some(v) = request.sub_appid.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&sub_appid={v}"));
            }
        }
        if let Some(v) = request.authenticate_state.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&authenticate_state={v}"));
            }
        }
        let url = format!(
            "{}/v3/payroll-card/authentications{query}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn payroll_card_pre_order_with_auth(
        &self,
        request: &PreOrderWithAuthRequest,
    ) -> Result<PreOrderWithAuthResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：user_name/id_card_number RSA-OAEP 加密
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        if let Some(v) = request.user_name.as_deref() {
            if !v.trim().is_empty() {
                request.user_name = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        if let Some(v) = request.id_card_number.as_deref() {
            if !v.trim().is_empty() {
                request.id_card_number = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        let url = format!(
            "{}/v3/payroll-card/authentications/pre-order-with-auth",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn merchant_fund_withdraw_bill_type(
        &self,
        bill_type: &str,
        bill_date: &str,
        tar_type: &str,
    ) -> Result<WxPayApplyBillV3Result, WxErrorException> {
        let svc = self.svc()?;
        let mut query = format!("?bill_date={bill_date}");
        if !tar_type.trim().is_empty() {
            query.push_str(&format!("&tar_type={tar_type}"));
        }
        let url = format!(
            "{}/v3/merchant/fund/withdraw/bill-type/{bill_type}{query}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn payroll_card_transfer_batches(
        &self,
        request: &PayrollTransferBatchesRequest,
    ) -> Result<PayrollTransferBatchesResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：transfer_detail_list 每项 encryptFields（user_name/user_id_card）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &[
                "transfer_detail_list.*.user_name",
                "transfer_detail_list.*.user_id_card",
            ],
        )?;
        let url = format!(
            "{}/v3/payroll-card/transfer-batches",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
