//! 对应 Java `service.impl.MiPayServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::MiPayService;
use crate::api::WxPayService;
use crate::bean::*;
use crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// MiPay 服务实现（对应 Java `MiPayServiceImpl`）。
pub struct MiPayServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl MiPayServiceImpl {
    /// 构建实现（对应 Java 构造器 `MiPayServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用（对应 Java `this.payService` 直接使用）。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }

    /// 加密支付人身份信息敏感字段（对应 Java `RsaCryptoUtil.encryptField` 的
    /// `PersonIdentification`：`name`/`id_digest` 标注 `@SpecEncrypt`）。
    fn encrypt_person_identification(
        &self,
        pid: &mut PersonIdentification,
        public_key: &rsa::RsaPublicKey,
    ) -> Result<(), WxErrorException> {
        if let Some(name) = pid.name.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            pid.name = Some(
                rsa_oaep_encrypt(public_key, name)
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
            );
        }
        if let Some(digest) = pid
            .id_digest
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            pid.id_digest = Some(
                rsa_oaep_encrypt(public_key, digest)
                    .map_err(|e| impl_utils::runtime(e.to_string()))?,
            );
        }
        Ok(())
    }
}

#[async_trait]
impl MiPayService for MiPayServiceImpl {
    async fn med_ins_orders(
        &self,
        request: &MedInsOrdersRequest,
    ) -> Result<MedInsOrdersResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java `RsaCryptoUtil.encryptFields(request, validCertificate)`
        // （@SpecEncrypt 字段：payer/relative 对象及其 name/id_digest）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        if let Some(payer) = request.payer.as_mut() {
            self.encrypt_person_identification(payer, &public_key)?;
        }
        if let Some(relative) = request.relative.as_mut() {
            self.encrypt_person_identification(relative, &public_key)?;
        }
        let url = format!("{}/v3/med-ins/orders", svc.get_pay_base_url());
        let json =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &json).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_med_ins_order_by_mix_trade_no(
        &self,
        mix_trade_no: &str,
        sub_mchid: &str,
    ) -> Result<MedInsOrdersResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/med-ins/orders/mix-trade-no/{mix_trade_no}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_med_ins_order_by_out_trade_no(
        &self,
        out_trade_no: &str,
        sub_mchid: &str,
    ) -> Result<MedInsOrdersResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/med-ins/orders/out-trade-no/{out_trade_no}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_mi_pay_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<MiPayNotifyV3Result, WxErrorException> {
        // 对应 Java `payService.baseParseOrderNotifyV3Result(...)`：验签 +
        // AES-GCM 解密 resource 后反序列化为 `MiPayNotifyV3Result`。
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
            notify_data,
            Some(header),
            api_v3_key,
            move |_serial, message, signature| {
                crate::util::crypto::wx_pay_v3_crypto_utils::verify_sha256_rsa(
                    &public_key,
                    message,
                    signature,
                )
                .unwrap_or(false)
            },
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        Ok(parsed.result)
    }

    async fn med_ins_refund_notify(
        &self,
        request: &MedInsRefundNotifyRequest,
        mix_trade_no: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/med-ins/refunds/notify?mix_trade_no={mix_trade_no}",
            svc.get_pay_base_url()
        );
        let json =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &json).await?;
        Ok(())
    }
}
