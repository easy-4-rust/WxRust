//! 对应 Java `service.impl.ComplaintServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{ComplaintService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// ComplaintService 实现（对应 Java `ComplaintServiceImpl`）。
pub struct ComplaintServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl ComplaintServiceImpl {
    /// 构建实现（对应 Java 构造器 `ComplaintServiceImpl(WxPayService)`）。
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
impl ComplaintService for ComplaintServiceImpl {
    async fn query_complaints(
        &self,
        request: &ComplaintRequest,
    ) -> Result<ComplaintResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaints-v2?limit={}&offset={}&begin_date={}&end_date={}&complainted_mchid={}",
            svc.get_pay_base_url(),
            request.limit.unwrap_or_default(),
            request.offset.unwrap_or_default(),
            request.begin_date.as_deref().unwrap_or_default(),
            request.end_date.as_deref().unwrap_or_default(),
            request.complainted_mchid.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        let mut result: ComplaintResult =
            serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：payer_phone 以商户私钥 RSA-OAEP 解密
        for item in result.data.iter_mut() {
            self.decrypt_payer_phone(&mut item.payer_phone)?;
        }
        Ok(result)
    }

    async fn get_complaint(
        &self,
        request: &ComplaintDetailRequest,
    ) -> Result<ComplaintDetailResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaints-v2/{}",
            svc.get_pay_base_url(),
            request.complaint_id.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        let mut result: ComplaintDetailResult =
            serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))?;
        self.decrypt_payer_phone(&mut result.payer_phone)?;
        Ok(result)
    }

    async fn query_negotiation_historys(
        &self,
        request: &NegotiationHistoryRequest,
    ) -> Result<NegotiationHistoryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaints-v2/{}/negotiation-historys?limit={}&offset={}",
            svc.get_pay_base_url(),
            request.complaint_id.as_deref().unwrap_or_default(),
            request.limit.unwrap_or_default(),
            request.offset.unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn add_complaint_notify_url(
        &self,
        request: &ComplaintNotifyUrlRequest,
    ) -> Result<ComplaintNotifyUrlResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaint-notifications",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_complaint_notify_url(&self) -> Result<ComplaintNotifyUrlResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaint-notifications",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn update_complaint_notify_url(
        &self,
        request: &ComplaintNotifyUrlRequest,
    ) -> Result<ComplaintNotifyUrlResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaint-notifications",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.put_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn delete_complaint_notify_url(&self) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaint-notifications",
            svc.get_pay_base_url()
        );
        svc.delete_v3(&url).await?;
        Ok(())
    }

    async fn submit_response(&self, request: &ResponseRequest) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaints-v2/{}/response",
            svc.get_pay_base_url(),
            request.complaint_id.as_deref().unwrap_or_default()
        );
        // 对应 Java：complaint_id 置空后序列化（路径中已携带）
        let mut body_json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        if let Some(obj) = body_json.as_object_mut() {
            obj.remove("complaint_id");
        }
        let body =
            serde_json::to_string(&body_json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn complete(&self, request: &CompleteRequest) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaints-v2/{}/complete",
            svc.get_pay_base_url(),
            request.complaint_id.as_deref().unwrap_or_default()
        );
        let mut body_json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        if let Some(obj) = body_json.as_object_mut() {
            obj.remove("complaint_id");
        }
        let body =
            serde_json::to_string(&body_json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn update_refund_progress(
        &self,
        request: &UpdateRefundProgressRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/complaints-v2/{}/update-refund-progress",
            svc.get_pay_base_url(),
            request.complaint_id.as_deref().unwrap_or_default()
        );
        let mut body_json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        if let Some(obj) = body_json.as_object_mut() {
            obj.remove("complaint_id");
        }
        let body =
            serde_json::to_string(&body_json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn upload_response_image(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<ImageUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant-service/images/upload",
            svc.get_pay_base_url()
        );
        // 对应 Java `DigestUtils.sha256Hex` + `WechatPayUploadHttpPost.Builder.withImage`
        let sha256 = crate::util::crypto::wx_pay_crypto_utils::sha256_hex(file_data);
        let meta = format!("{{\"filename\":\"{file_name}\",\"sha256\":\"{sha256}\"}}");
        let (content_type, body) = impl_utils::build_multipart_meta_file(
            file_name,
            impl_utils::guess_file_content_type(file_name),
            file_data,
            &meta,
        );
        let result = impl_utils::execute_v3_upload(
            svc.wx_pay_config().as_ref(),
            svc.http_client(),
            &url,
            &content_type,
            &body,
        )
        .await?;
        ImageUploadResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

impl ComplaintServiceImpl {
    /// 解密投诉人手机号（对应 Java `RsaCryptoUtil.decryptOAEP(payerPhone, config.getPrivateKey())`）。
    fn decrypt_payer_phone(
        &self,
        payer_phone: &mut Option<String>,
    ) -> Result<(), WxErrorException> {
        let Some(v) = payer_phone.as_deref() else {
            return Ok(());
        };
        if v.trim().is_empty() {
            return Ok(());
        }
        let svc = self.svc()?;
        let private_key = impl_utils::load_merchant_private_key(svc.wx_pay_config().as_ref())?;
        let decrypted =
            crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_decrypt(&private_key, v.trim())
                .map_err(|e| impl_utils::runtime(e.to_string()))?;
        *payer_phone = Some(decrypted);
        Ok(())
    }
}
