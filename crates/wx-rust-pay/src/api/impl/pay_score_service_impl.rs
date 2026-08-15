//! 对应 Java `service.impl.PayScoreServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{PayScoreService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// PayScoreService 实现（对应 Java `PayScoreServiceImpl`）。
pub struct PayScoreServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl PayScoreServiceImpl {
    /// 构建实现（对应 Java 构造器 `PayScoreServiceImpl(WxPayService)`）。
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
impl PayScoreService for PayScoreServiceImpl {
    async fn permissions(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：appid/service_id/notify_url 从配置补齐
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
            let permission_notify_url = config.pay_score_permission_notify_url();
            if permission_notify_url
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                return Err(impl_utils::runtime("授权回调地址未配置"));
            }
            request.notify_url = permission_notify_url.map(str::to_string);
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
        let url = format!("{}/v3/payscore/permissions", svc.get_pay_base_url());
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_query_by_authorization_code(
        &self,
        authorization_code: &str,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        if authorization_code.trim().is_empty() {
            return Err(impl_utils::runtime("authorizationCode不允许为空"));
        }
        let url = format!(
            "{}/v3/payscore/permissions/authorization-code/{authorization_code}?service_id={}",
            svc.get_pay_base_url(),
            config.service_id().unwrap_or_default()
        );
        let result = svc.get_v3(&url).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_terminate_by_authorization_code(
        &self,
        authorization_code: &str,
        reason: &str,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        if authorization_code.trim().is_empty() {
            return Err(impl_utils::runtime("authorizationCode不允许为空"));
        }
        let url = format!(
            "{}/v3/payscore/permissions/authorization-code/{authorization_code}/terminate",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "service_id": config.service_id().unwrap_or_default(),
            "reason": reason,
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_query_by_open_id(
        &self,
        open_id: &str,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        if open_id.trim().is_empty() {
            return Err(impl_utils::runtime("openId不允许为空"));
        }
        let url = format!(
            "{}/v3/payscore/permissions/openid/{open_id}?appid={}&service_id={}",
            svc.get_pay_base_url(),
            config.app_id().unwrap_or_default(),
            config.service_id().unwrap_or_default()
        );
        let result = svc.get_v3(&url).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn permissions_terminate_by_open_id(
        &self,
        open_id: &str,
        reason: &str,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let url = format!(
            "{}/v3/payscore/permissions/openid/{open_id}/terminate",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "service_id": config.service_id().unwrap_or_default(),
            "appid": config.app_id().unwrap_or_default(),
            "reason": reason,
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn create_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let need_user_confirm = request.need_user_confirm.unwrap_or_default();
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
        let url = format!("{}/v3/payscore/serviceorder", svc.get_pay_base_url());
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        let mut create_result =
            WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：组装 pay_score_sign_info（HMAC-SHA256 签名）
        let current_time_millis = impl_utils::current_time_millis();
        let mut sign_map = std::collections::HashMap::new();
        sign_map.insert(
            "mch_id".to_string(),
            config.mch_id().unwrap_or_default().to_string(),
        );
        if need_user_confirm {
            sign_map.insert(
                "package".to_string(),
                create_result.package_x.clone().unwrap_or_default(),
            );
        } else {
            sign_map.insert(
                "service_id".to_string(),
                config.service_id().unwrap_or_default().to_string(),
            );
            sign_map.insert(
                "out_order_no".to_string(),
                request.out_order_no.clone().unwrap_or_default(),
            );
        }
        sign_map.insert("timestamp".to_string(), current_time_millis.clone());
        sign_map.insert("nonce_str".to_string(), current_time_millis.clone());
        sign_map.insert("sign_type".to_string(), "HMAC-SHA256".to_string());
        let sign = crate::util::sign_utils::SignUtils::create_sign(
            &sign_map,
            Some(crate::constant::wx_pay_constants::sign_type::HMAC_SHA256),
            config.mch_key().unwrap_or_default(),
            &[],
        )?;
        sign_map.insert("sign".to_string(), sign);
        create_result.pay_score_sign_info = sign_map
            .into_iter()
            .map(|(k, v)| (Some(k), Some(v)))
            .collect();
        Ok(create_result)
    }

    async fn query_service_order(
        &self,
        out_order_no: &str,
        query_id: &str,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        // 对应 Java：out_order_no/query_id 二选一
        let out_empty = out_order_no.trim().is_empty();
        let query_empty = query_id.trim().is_empty();
        if (out_empty && query_empty) || (!out_empty && !query_empty) {
            return Err(impl_utils::runtime(
                "out_order_no,query_id不允许都填写或都不填写",
            ));
        }
        let mut url = format!("{}/v3/payscore/serviceorder", svc.get_pay_base_url());
        let mut parts = Vec::new();
        if !out_empty {
            parts.push(format!("out_order_no={out_order_no}"));
        }
        if !query_empty {
            parts.push(format!("query_id={query_id}"));
        }
        parts.push(format!(
            "service_id={}",
            config.service_id().unwrap_or_default()
        ));
        parts.push(format!("appid={}", config.app_id().unwrap_or_default()));
        url.push('?');
        url.push_str(&parts.join("&"));
        let result = svc.get_v3(&url).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn cancel_service_order(
        &self,
        out_order_no: &str,
        reason: &str,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let url = format!(
            "{}/v3/payscore/serviceorder/{out_order_no}/cancel",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "appid": config.app_id().unwrap_or_default(),
            "service_id": config.service_id().unwrap_or_default(),
            "reason": reason,
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn modify_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException> {
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
        // 对应 Java：out_order_no 置空（路径已携带）
        request.out_order_no = None;
        let url = format!(
            "{}/v3/payscore/serviceorder/{out_order_no}/modify",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn complete_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException> {
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
        request.out_order_no = None;
        let url = format!(
            "{}/v3/payscore/serviceorder/{out_order_no}/complete",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn pay_service_order(
        &self,
        out_order_no: &str,
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let url = format!(
            "{}/v3/payscore/serviceorder/{out_order_no}/pay",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({
            "appid": config.app_id().unwrap_or_default(),
            "service_id": config.service_id().unwrap_or_default(),
        });
        let result = svc.post_v3(&url, &body.to_string()).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sync_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException> {
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
        request.out_order_no = None;
        let url = format!(
            "{}/v3/payscore/serviceorder/{out_order_no}/sync",
            svc.get_pay_base_url()
        );
        let body = request.to_json().map_err(impl_utils::runtime)?;
        let result = svc.post_v3(&url, &body).await?;
        WxPayScoreResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_user_authorization_status_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<UserAuthorizationStatusNotifyResult, WxErrorException> {
        let response = self.parse_notify_data(notify_data, header).await?;
        let resource = response
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        let decrypted = self.decrypt_resource(resource)?;
        let mut notify_result: UserAuthorizationStatusNotifyResult =
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
    ) -> Result<WxPayScoreResult, WxErrorException> {
        let resource = data
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        let decrypted = self.decrypt_resource(resource)?;
        WxPayScoreResult::from_json(&decrypted).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

impl PayScoreServiceImpl {
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
