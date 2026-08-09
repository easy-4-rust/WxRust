//! 对应 Java `service.impl.CustomDeclarationServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{CustomDeclarationService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// CustomDeclarationService 实现（对应 Java `CustomDeclarationServiceImpl`）。
pub struct CustomDeclarationServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl CustomDeclarationServiceImpl {
    /// 构建实现（对应 Java 构造器 `CustomDeclarationServiceImpl(WxPayService)`）。
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
impl CustomDeclarationService for CustomDeclarationServiceImpl {
    async fn declare(
        &self,
        request: &DeclarationRequest,
    ) -> Result<DeclarationResult, WxErrorException> {
        let svc = self.svc()?;
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc
            .post_v3(&format!("{DECLARATION_BASE_URL}/orders"), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query(
        &self,
        request: &DeclarationQueryRequest,
    ) -> Result<DeclarationQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{DECLARATION_BASE_URL}/orders?appid={}&mchid={}&order_type={}&order_no={}&customs={}&offset={}&limit={}",
            request.appid.as_deref().unwrap_or_default(),
            request.mchid.as_deref().unwrap_or_default(),
            request.order_type.as_deref().unwrap_or_default(),
            request.order_no.as_deref().unwrap_or_default(),
            request.customs.as_deref().unwrap_or_default(),
            request.offset.as_deref().unwrap_or_default(),
            request.limit.as_deref().unwrap_or_default(),
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn verify_certificate(
        &self,
        request: &VerifyCertificateRequest,
    ) -> Result<VerifyCertificateResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java `encryptFields`：certificate_id/certificate_name 以
        // RSA/ECB/PKCS1Padding 加密（Java encryptOAEP 通道，非 OAEP）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        if let Some(v) = request.certificate_id.take() {
            if !v.trim().is_empty() {
                request.certificate_id = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_pkcs1_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        if let Some(v) = request.certificate_name.take() {
            if !v.trim().is_empty() {
                request.certificate_name = Some(
                    crate::util::crypto::wx_pay_v3_crypto_utils::rsa_pkcs1_encrypt(
                        &public_key,
                        v.trim(),
                    )
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
                );
            }
        }
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc
            .post_v3_with_wechatpay_serial(
                &format!("{DECLARATION_BASE_URL}/verify-certificate"),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn modify(
        &self,
        request: &DeclarationRequest,
    ) -> Result<DeclarationResult, WxErrorException> {
        let svc = self.svc()?;
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc
            .patch_v3(&format!("{DECLARATION_BASE_URL}/orders"), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn redeclare(
        &self,
        request: &RedeclareRequest,
    ) -> Result<RedeclareResult, WxErrorException> {
        let svc = self.svc()?;
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc
            .post_v3(&format!("{DECLARATION_BASE_URL}/redeclare"), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

/// 海关报关接口基地址（对应 Java `CustomDeclarationService.DECLARATION_BASE_URL`
/// 常量：全球报关地址，不走 `getPayBaseUrl()`）。
const DECLARATION_BASE_URL: &str = "https://apihk.mch.weixin.qq.com/global/v3/customs";
