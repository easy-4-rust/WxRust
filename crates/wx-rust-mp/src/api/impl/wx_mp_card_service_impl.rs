//! WxMpCardService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpCardServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpCardService, WxMpService};
use crate::bean::card::{
    WxMpCardCreateRequest, WxMpCardCreateResult, WxMpCardDeleteResult,
    WxMpCardLandingPageCreateRequest, WxMpCardLandingPageCreateResult, WxMpCardQrcodeCreateResult,
    WxMpCardResult,
};
use crate::enums::wx_mp_api_url::card as card_url;
use wx_rust_common::bean::WxCardApiSignature;
use wx_rust_common::util::crypto::Sha1;

/// 公众号CardService实现。
pub struct WxMpCardServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpCardServiceImpl {
    /// 构建 公众号CardService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpCardService for WxMpCardServiceImpl {
    async fn get_card_api_ticket(&self, force_refresh: bool) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let _config = svc.wx_mp_config_storage();
        svc.get_ticket(wx_rust_common::config::TicketType::WxCard, force_refresh)
            .await
    }

    async fn create_card_api_signature(
        &self,
        optional_sign_param: &[&str],
    ) -> Result<WxCardApiSignature, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        // Java 语义：optional + [timestamp, nonceStr, cardApiTicket] 排序后拼接 sha1
        let ticket = self.get_card_api_ticket(false).await?;
        let timestamp = chrono::Utc::now().timestamp();
        let nonce_str = format!("{}", chrono::Utc::now().timestamp_millis());
        let ts = timestamp.to_string();
        let mut params: Vec<&str> = optional_sign_param.to_vec();
        params.push(&ts);
        params.push(&nonce_str);
        params.push(&ticket);
        let signature = Sha1::digest(&params).map_err(WxErrorException::Serde)?;
        Ok(WxCardApiSignature {
            app_id: config.app_id().to_string(),
            card_id: String::new(),
            card_type: String::new(),
            location_id: None,
            code: None,
            open_id: None,
            timestamp: Some(timestamp),
            nonce_str,
            signature,
        })
    }

    async fn decrypt_card_code(&self, encrypt_code: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"encrypt_code": encrypt_code});
        let response = svc
            .post(
                &card_url::card_code_decrypt(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("code")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "code 缺失"))
    }

    async fn query_card_code(
        &self,
        card_id: &str,
        code: &str,
        check_consume: bool,
    ) -> Result<WxMpCardResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::json!({"card_id": card_id, "code": code, "check_consume": check_consume});
        let response = svc
            .post(&card_url::card_code_get(config.as_ref()), &body.to_string())
            .await?;
        WxMpCardResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn consume_card_code(&self, code: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"code": code});
        let response = svc
            .post(
                &card_url::card_code_consume(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("card")
            .and_then(|v| v.get("card_id"))
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "card_id 缺失"))
    }

    async fn mark_card_code(
        &self,
        code: &str,
        card_id: &str,
        open_id: &str,
        is_mark: bool,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"code": code, "card_id": card_id, "openid": open_id, "is_mark": if is_mark { 1 } else { 0 }});
        svc.post(
            &card_url::card_code_mark(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_card_detail(&self, card_id: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"card_id": card_id});
        let response = svc
            .post(&card_url::card_get(config.as_ref()), &body.to_string())
            .await?;
        Ok(response)
    }

    async fn add_test_white_list(&self, openid: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"openid": [openid]});
        let response = svc
            .post(
                &card_url::card_test_whitelist(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        Ok(response)
    }

    async fn create_card(
        &self,
        request: &WxMpCardCreateRequest,
    ) -> Result<WxMpCardCreateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&card_url::card_create(config.as_ref()), &body)
            .await?;
        WxMpCardCreateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_card(&self, card_id: &str) -> Result<WxMpCardDeleteResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"card_id": card_id});
        let response = svc
            .post(&card_url::card_delete(config.as_ref()), &body.to_string())
            .await?;
        WxMpCardDeleteResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn create_landing_page(
        &self,
        request: &WxMpCardLandingPageCreateRequest,
    ) -> Result<WxMpCardLandingPageCreateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&card_url::card_landing_page_create(config.as_ref()), &body)
            .await?;
        WxMpCardLandingPageCreateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn create_qrcode_card(
        &self,
        card_id: &str,
        outer_str: &str,
        expires_in: Option<i32>,
    ) -> Result<WxMpCardQrcodeCreateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        // Java 语义：action_name=QR_CARD + action_info.card{card_id, outer_str}
        let mut body = serde_json::Map::new();
        body.insert("action_name".into(), serde_json::json!("QR_CARD"));
        if let Some(e) = expires_in {
            body.insert("expire_seconds".into(), serde_json::json!(e));
        }
        body.insert(
            "action_info".into(),
            serde_json::json!({
                "card": {"card_id": card_id, "outer_str": outer_str}
            }),
        );
        let response = svc
            .post(
                &card_url::card_qrcode_create(config.as_ref()),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        WxMpCardQrcodeCreateResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}
