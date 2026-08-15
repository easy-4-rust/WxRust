//! 对应 Java `service.impl.BusinessCircleServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{BusinessCircleService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// BusinessCircleService 实现（对应 Java `BusinessCircleServiceImpl`）。
pub struct BusinessCircleServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl BusinessCircleServiceImpl {
    /// 构建实现（对应 Java 构造器 `BusinessCircleServiceImpl(WxPayService)`）。
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
impl BusinessCircleService for BusinessCircleServiceImpl {
    async fn notify_points(&self, request: &PointsNotifyRequest) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        // 对应 Java `RsaCryptoUtil.encryptFields`：PointsNotifyRequest 无
        // @SpecEncrypt 字段（Java 反射遍历无操作），故直接序列化
        let url = format!("{}/v3/businesscircle/points/notify", svc.get_pay_base_url());
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        Ok(())
    }

    async fn parse_notify_data(
        &self,
        data: &str,
        header: &SignatureHeader,
    ) -> Result<BusinessCircleNotifyData, WxErrorException> {
        // 对应 Java：验签（探测流量识别）后 GSON 解析
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        crate::util::wx_pay_notify_utils::verify_notify_signature(&public_key, header, data)?;
        serde_json::from_str(data).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn decrypt_paid_notify_data_resource(
        &self,
        data: &BusinessCircleNotifyData,
    ) -> Result<PaidResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let resource = data
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        // 对应 Java `AesUtils.decryptToString(associatedData, nonce, cipherText, apiV3Key)`
        let decrypted = crate::util::crypto::wx_pay_v3_crypto_utils::aes_gcm_decrypt(
            api_v3_key,
            resource.associated_data.as_deref().unwrap_or_default(),
            resource.nonce.as_deref().unwrap_or_default(),
            resource.cipher_text.as_deref().unwrap_or_default(),
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        serde_json::from_str(&decrypted).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn decrypt_refund_notify_data_resource(
        &self,
        data: &BusinessCircleNotifyData,
    ) -> Result<RefundResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let resource = data
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        let decrypted = crate::util::crypto::wx_pay_v3_crypto_utils::aes_gcm_decrypt(
            api_v3_key,
            resource.associated_data.as_deref().unwrap_or_default(),
            resource.nonce.as_deref().unwrap_or_default(),
            resource.cipher_text.as_deref().unwrap_or_default(),
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        serde_json::from_str(&decrypted).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
