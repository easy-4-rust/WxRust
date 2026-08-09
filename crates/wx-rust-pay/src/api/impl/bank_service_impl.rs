//! 对应 Java `service.impl.BankServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{BankService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// BankService 实现（对应 Java `BankServiceImpl`）。
pub struct BankServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl BankServiceImpl {
    /// 构建实现（对应 Java 构造器 `BankServiceImpl(WxPayService)`）。
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
impl BankService for BankServiceImpl {
    async fn search_banks_by_bank_account(
        &self,
        account_number: &str,
    ) -> Result<BankAccountResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        // 对应 Java：RSA-OAEP 加密银行卡号后 URL 编码
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let encrypted = crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
            &public_key,
            account_number,
        )
        .map_err(|e| impl_utils::runtime(e.to_string()))?;
        let encoded = urlencoding(&encrypted);
        let url = format!(
            "{}/v3/capital/capitallhh/banks/search-banks-by-bank-account?account_number={encoded}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn personal_banking(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<BankingResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/capital/capitallhh/banks/personal-banking?offset={offset}&limit={limit}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn corporate_banking(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<BankingResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/capital/capitallhh/banks/corporate-banking?offset={offset}&limit={limit}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn areas_provinces(&self) -> Result<ProvincesResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/capital/capitallhh/areas/provinces",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn areas_cities(&self, province_code: i32) -> Result<CitiesResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/capital/capitallhh/areas/provinces/{province_code}/cities",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn bank_branches(
        &self,
        bank_alias_code: &str,
        city_code: i32,
        offset: i32,
        limit: i32,
    ) -> Result<BankBranchesResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/capital/capitallhh/banks/{bank_alias_code}/branches?city_code={city_code}&offset={offset}&limit={limit}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

/// URL 编码（对应 Java `URLEncoder.encode(s, "UTF-8")`；ADAPTED：以
/// `application/x-www-form-urlencoded` 语义编码）。
fn urlencoding(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
