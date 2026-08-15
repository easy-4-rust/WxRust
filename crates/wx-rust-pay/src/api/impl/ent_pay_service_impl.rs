//! 对应 Java `service.impl.EntPayServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{EntPayService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;
use crate::util::wx_pay_service_impl_utils::V2Request;

/// EntPayService 实现（对应 Java `EntPayServiceImpl`）。
pub struct EntPayServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl EntPayServiceImpl {
    /// 构建实现（对应 Java 构造器 `EntPayServiceImpl(WxPayService)`）。
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
impl EntPayService for EntPayServiceImpl {
    async fn ent_pay(&self, request: &EntPayRequest) -> Result<EntPayResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!(
            "{}/mmpaymkttransfers/promotion/transfers",
            svc.get_pay_base_url()
        );
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            EntPayResult::from_xml,
        )
    }

    async fn query_ent_pay(
        &self,
        partner_trade_no: &str,
    ) -> Result<EntPayQueryResult, WxErrorException> {
        let mut request = EntPayQueryRequest::default();
        request.partner_trade_no = Some(partner_trade_no.to_string());
        self.query_ent_pay_with_request(&request).await
    }

    async fn query_ent_pay_with_request(
        &self,
        request: &EntPayQueryRequest,
    ) -> Result<EntPayQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!(
            "{}/mmpaymkttransfers/gettransferinfo",
            svc.get_pay_base_url()
        );
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            EntPayQueryResult::from_xml,
        )
    }

    async fn get_public_key(&self) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        // 对应 Java：WxPayDefaultRequest（mchId + nonce）签名后 POST 固定地址
        let mut request = WxPayDefaultRequest::default();
        request.mch_id = config.mch_id().map(str::to_string);
        request.nonce_str = Some(impl_utils::current_time_millis());
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = "https://fraud.mch.weixin.qq.com/risk/getpublickey";
        let response_content = svc
            .post(url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        let result = impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            GetPublicKeyResult::from_xml,
        )?;
        Ok(result.pub_key.unwrap_or_default())
    }

    async fn pay_bank(
        &self,
        request: &EntPayBankRequest,
    ) -> Result<EntPayBankResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：先取商户公钥，RSA-OAEP(SHA1) 加密银行卡号与姓名
        let public_key_str = self.get_public_key().await?;
        let public_key = crate::util::crypto::wx_pay_cert_utils::load_public_key_from_pem(
            public_key_str.as_bytes(),
        )
        .map_err(WxErrorException::from)?;
        if let Some(v) = request.enc_bank_no.as_deref() {
            if !v.trim().is_empty() {
                request.enc_bank_no = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        if let Some(v) = request.enc_true_name.as_deref() {
            if !v.trim().is_empty() {
                request.enc_true_name = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/mmpaysptrans/pay_bank", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            EntPayBankResult::from_xml,
        )
    }

    async fn query_pay_bank(
        &self,
        partner_trade_no: &str,
    ) -> Result<EntPayBankQueryResult, WxErrorException> {
        let mut request = EntPayBankQueryRequest::default();
        request.partner_trade_no = Some(partner_trade_no.to_string());
        self.query_pay_bank_with_request(&request).await
    }

    async fn query_pay_bank_with_request(
        &self,
        request: &EntPayBankQueryRequest,
    ) -> Result<EntPayBankQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/mmpaysptrans/query_bank", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            EntPayBankQueryResult::from_xml,
        )
    }

    async fn send_enterprise_redpack(
        &self,
        request: &EntPayRedpackRequest,
    ) -> Result<EntPayRedpackResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：work_wx_sign（企业微信签名，MD5）
        request.nonce_str = Some(impl_utils::current_time_millis());
        let mut map = std::collections::HashMap::new();
        map.insert(
            "act_name".to_string(),
            request.act_name.clone().unwrap_or_default(),
        );
        map.insert(
            "mch_billno".to_string(),
            request.mch_bill_no.clone().unwrap_or_default(),
        );
        map.insert(
            "mch_id".to_string(),
            request.mch_id.clone().unwrap_or_default(),
        );
        map.insert(
            "nonce_str".to_string(),
            request.nonce_str.clone().unwrap_or_default(),
        );
        map.insert(
            "re_openid".to_string(),
            request.re_openid.clone().unwrap_or_default(),
        );
        map.insert(
            "total_amount".to_string(),
            request.total_amount.unwrap_or_default().to_string(),
        );
        map.insert(
            "wxappid".to_string(),
            request.wx_app_id.clone().unwrap_or_default(),
        );
        request.work_wx_sign = Some(crate::util::sign_utils::SignUtils::create_sign(
            &map,
            Some(crate::constant::wx_pay_constants::sign_type::MD5),
            config.ent_pay_key().unwrap_or_default(),
            &[],
        )?);
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!(
            "{}/mmpaymkttransfers/sendworkwxredpack",
            svc.get_pay_base_url()
        );
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            EntPayRedpackResult::from_xml,
        )
    }

    async fn query_enterprise_redpack(
        &self,
        request: &EntPayRedpackQueryRequest,
    ) -> Result<EntPayRedpackQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!(
            "{}/mmpaymkttransfers/queryworkwxredpack",
            svc.get_pay_base_url()
        );
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            EntPayRedpackQueryResult::from_xml,
        )
    }

    async fn to_emp_pay(
        &self,
        request: &EntWxEmpPayRequest,
    ) -> Result<EntPayResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：work_wx_sign（企业微信签名，MD5，字段序 amount/appid/desc/mch_id/nonce_str/openid/partner_trade_no/ww_msg_type）
        request.nonce_str = Some(impl_utils::current_time_millis());
        let mut map = std::collections::HashMap::new();
        map.insert(
            "amount".to_string(),
            request.amount.unwrap_or_default().to_string(),
        );
        map.insert(
            "appid".to_string(),
            request.appid.clone().unwrap_or_default(),
        );
        map.insert(
            "desc".to_string(),
            request.description.clone().unwrap_or_default(),
        );
        map.insert(
            "mch_id".to_string(),
            request.mch_id.clone().unwrap_or_default(),
        );
        map.insert(
            "nonce_str".to_string(),
            request.nonce_str.clone().unwrap_or_default(),
        );
        map.insert(
            "openid".to_string(),
            request.openid.clone().unwrap_or_default(),
        );
        map.insert(
            "partner_trade_no".to_string(),
            request.partner_trade_no.clone().unwrap_or_default(),
        );
        map.insert(
            "ww_msg_type".to_string(),
            request.ww_msg_type.clone().unwrap_or_default(),
        );
        request.work_wx_sign = Some(crate::util::sign_utils::SignUtils::create_sign(
            &map,
            Some(crate::constant::wx_pay_constants::sign_type::MD5),
            config.ent_pay_key().unwrap_or_default(),
            &[],
        )?);
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!(
            "{}/mmpaymkttransfers/promotion/paywwsptrans2pocket",
            svc.get_pay_base_url()
        );
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            EntPayResult::from_xml,
        )
    }
}
