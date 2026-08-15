//! 对应 Java `service.impl.PartnerPayScoreServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{PartnerPayScoreService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// PartnerPayScoreService 实现（对应 Java `PartnerPayScoreServiceImpl`）。
pub struct PartnerPayScoreServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl PartnerPayScoreServiceImpl {
    /// 构建实现（对应 Java 构造器 `PartnerPayScoreServiceImpl(WxPayService)`）。
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
impl PartnerPayScoreService for PartnerPayScoreServiceImpl {
    async fn permissions(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
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
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.pay_score_permission_notify_url().map(str::to_string);
        }
        if request
            .authorization_code
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            return Err(impl_utils::runtime("authorizationCode不允许为空"));
        }
        let url = format!("{}/v3/payscore/partner/permissions", svc.get_pay_base_url());
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_query_by_authorization_code(
        &self,
        service_id: &str,
        sub_mchid: &str,
        authorization_code: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        if authorization_code.trim().is_empty() {
            return Err(impl_utils::runtime("authorizationCode不允许为空"));
        }
        let url = format!(
            "{}/v3/payscore/partner/permissions/authorization-code/{authorization_code}?service_id={service_id}&sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_terminate_by_authorization_code(
        &self,
        service_id: &str,
        sub_mchid: &str,
        authorization_code: &str,
        reason: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        if authorization_code.trim().is_empty() {
            return Err(impl_utils::runtime("authorizationCode不允许为空"));
        }
        let url = format!(
            "{}/v3/payscore/partner/permissions/authorization-code/{authorization_code}/terminate",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "service_id": service_id,
            "sub_mchid": sub_mchid,
            "reason": reason,
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_query_by_open_id(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        sub_appid: &str,
        open_id: &str,
        sub_openid: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        // 对应 Java：open_id/sub_openid 二选一
        let open_empty = open_id.trim().is_empty();
        let sub_empty = sub_openid.trim().is_empty();
        if (open_empty && sub_empty) || (!open_empty && !sub_empty) {
            return Err(impl_utils::runtime(
                "open_id,sub_openid不允许都填写或都不填写",
            ));
        }
        if sub_mchid.trim().is_empty() {
            return Err(impl_utils::runtime("sub_mchid不允许都为空"));
        }
        let mut url = format!(
            "{}/v3/payscore/partner/permissions/search",
            svc.get_pay_base_url()
        );
        let mut parts = vec![
            format!("appid={app_id}"),
            format!("service_id={service_id}"),
            format!("sub_mchid={sub_mchid}"),
            format!("sub_appid={sub_appid}"),
        ];
        if !open_empty {
            parts.push(format!("openid={open_id}"));
        }
        if !sub_empty {
            parts.push(format!("sub_openid={sub_openid}"));
        }
        url.push('?');
        url.push_str(&parts.join("&"));
        let result = svc.get_v3(&url).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_terminate_by_open_id(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        sub_appid: &str,
        open_id: &str,
        sub_openid: &str,
        reason: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let open_empty = open_id.trim().is_empty();
        let sub_empty = sub_openid.trim().is_empty();
        if (open_empty && sub_empty) || (!open_empty && !sub_empty) {
            return Err(impl_utils::runtime(
                "open_id,sub_openid不允许都填写或都不填写",
            ));
        }
        let url = format!(
            "{}/v3/payscore/partner/permissions/terminate",
            svc.get_pay_base_url()
        );
        let mut map = serde_json::Map::new();
        map.insert(
            "appid".to_string(),
            serde_json::Value::String(app_id.to_string()),
        );
        map.insert(
            "sub_appid".to_string(),
            serde_json::Value::String(sub_appid.to_string()),
        );
        map.insert(
            "service_id".to_string(),
            serde_json::Value::String(service_id.to_string()),
        );
        if !open_empty {
            map.insert(
                "openid".to_string(),
                serde_json::Value::String(open_id.to_string()),
            );
        }
        if !sub_empty {
            map.insert(
                "sub_openid".to_string(),
                serde_json::Value::String(sub_openid.to_string()),
            );
        }
        map.insert(
            "sub_mchid".to_string(),
            serde_json::Value::String(sub_mchid.to_string()),
        );
        map.insert(
            "reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );
        let result = svc
            .post_v3(&url, &serde_json::Value::Object(map).to_string())
            .await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn create_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
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
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.pay_score_notify_url().map(str::to_string);
        }
        let url = format!(
            "{}/v3/payscore/partner/serviceorder",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_service_order(
        &self,
        service_id: &str,
        sub_mchid: &str,
        out_order_no: &str,
        query_id: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let out_empty = out_order_no.trim().is_empty();
        let query_empty = query_id.trim().is_empty();
        if (out_empty && query_empty) || (!out_empty && !query_empty) {
            return Err(impl_utils::runtime(
                "out_order_no,query_id不允许都填写或都不填写",
            ));
        }
        let mut url = format!(
            "{}/v3/payscore/partner/serviceorder",
            svc.get_pay_base_url()
        );
        let mut parts = vec![
            format!("service_id={service_id}"),
            format!("sub_mchid={sub_mchid}"),
        ];
        if !out_empty {
            parts.push(format!("out_order_no={out_order_no}"));
        }
        if !query_empty {
            parts.push(format!("query_id={query_id}"));
        }
        url.push('?');
        url.push_str(&parts.join("&"));
        let result = svc.get_v3(&url).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn cancel_service_order(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        out_order_no: &str,
        reason: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payscore/partner/serviceorder/{out_order_no}/cancel",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "appid": app_id,
            "service_id": service_id,
            "sub_mchid": sub_mchid,
            "reason": reason,
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn modify_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        let out_order_no = request.out_order_no.clone().unwrap_or_default();
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
        if request
            .sub_mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sub_mchid = config.sub_mch_id().map(str::to_string);
        }
        request.out_order_no = None;
        let url = format!(
            "{}/v3/payscore/partner/serviceorder/{out_order_no}/modify",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn complete_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        let out_order_no = request.out_order_no.clone().unwrap_or_default();
        if request
            .service_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.service_id = config.service_id().map(str::to_string);
        }
        if request
            .sub_mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sub_mchid = config.sub_mch_id().map(str::to_string);
        }
        request.out_order_no = None;
        let url = format!(
            "{}/v3/payscore/partner/serviceorder/{out_order_no}/complete",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn pay_service_order(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        out_order_no: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        if app_id.trim().is_empty() || service_id.trim().is_empty() || sub_mchid.trim().is_empty() {
            return Err(impl_utils::runtime(
                "appid, service_id, sub_mchid都不能为空",
            ));
        }
        let url = format!(
            "{}/v3/payscore/partner/serviceorder/{out_order_no}/pay",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "appid": app_id,
            "service_id": service_id,
            "sub_mchid": sub_mchid,
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sync_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        let out_order_no = request.out_order_no.clone().unwrap_or_default();
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        request.out_order_no = None;
        let url = format!(
            "{}/v3/payscore/partner/serviceorder/{out_order_no}/sync",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn apply_service_account(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payscore/partner/service-account-applications",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "service_id": request.service_id.as_deref().unwrap_or_default(),
            "appid": request.appid.as_deref().unwrap_or_default(),
            "sub_mchid": request.sub_mchid.as_deref().unwrap_or_default(),
            "sub_appid": request.sub_appid.as_deref().unwrap_or_default(),
            "out_apply_no": request.out_apply_no.as_deref().unwrap_or_default(),
            "result_notify_url": request.result_notify_url.as_deref().unwrap_or_default(),
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_service_account_state(
        &self,
        out_apply_no: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/payscore/partner/service-account-applications/{out_apply_no}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        WxPartnerPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_user_authorization_status_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPartnerUserAuthorizationStatusNotifyResult, WxErrorException> {
        let response = self.parse_notify_data(notify_data, header).await?;
        let resource = response
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        let decrypted = self.decrypt_resource(resource)?;
        let mut notify_result: WxPartnerUserAuthorizationStatusNotifyResult =
            serde_json::from_str(&decrypted).map_err(|e| impl_utils::runtime(e.to_string()))?;
        notify_result.raw_data = Some(response);
        Ok(notify_result)
    }

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

    async fn decrypt_notify_data_resource(
        &self,
        data: &PayScoreNotifyData,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException> {
        let resource = data
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        let decrypted = self.decrypt_resource(resource)?;
        WxPartnerPayScoreResult::from_json(&decrypted)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

impl PartnerPayScoreServiceImpl {
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
