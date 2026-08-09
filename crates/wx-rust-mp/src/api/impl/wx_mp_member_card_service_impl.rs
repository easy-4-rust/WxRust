//! WxMpMemberCardService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpMemberCardServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpMemberCardService, WxMpService};
use crate::bean::card::CardUpdateResult;
use crate::bean::card::membercard::{
    MemberCardActivateUserFormRequest, MemberCardActivateUserFormResult, MemberCardUpdateRequest,
    WxMpMemberCardActivateTempInfoResult, WxMpMemberCardActivatedMessage,
    WxMpMemberCardCreateMessage, WxMpMemberCardUpdateMessage, WxMpMemberCardUpdateResult,
    WxMpMemberCardUserInfoResult,
};
use crate::enums::wx_mp_api_url::member_card;

/// 公众号MemberCardService实现。
pub struct WxMpMemberCardServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpMemberCardServiceImpl {
    /// 构建 公众号MemberCardService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpMemberCardService for WxMpMemberCardServiceImpl {
    async fn create_member_card(
        &self,
        message: &WxMpMemberCardCreateMessage,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&member_card::member_card_create(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("card_id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "card_id 缺失"))
    }

    async fn activate_member_card(
        &self,
        message: &WxMpMemberCardActivatedMessage,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&member_card::member_card_activate(config.as_ref()), &body)
            .await?;
        Ok(response)
    }

    async fn get_user_info(
        &self,
        card_id: &str,
        code: &str,
    ) -> Result<WxMpMemberCardUserInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"card_id": card_id, "code": code});
        let response = svc
            .post(
                &member_card::member_card_user_info_get(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpMemberCardUserInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_user_member_card(
        &self,
        message: &WxMpMemberCardUpdateMessage,
    ) -> Result<WxMpMemberCardUpdateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(
                &member_card::member_card_update_user(config.as_ref()),
                &body,
            )
            .await?;
        WxMpMemberCardUpdateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn set_activate_user_form(
        &self,
        request: &MemberCardActivateUserFormRequest,
    ) -> Result<MemberCardActivateUserFormResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(
                &member_card::member_card_activate_user_form(config.as_ref()),
                &body,
            )
            .await?;
        MemberCardActivateUserFormResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_card_info(
        &self,
        request: &MemberCardUpdateRequest,
    ) -> Result<CardUpdateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&member_card::member_card_update(config.as_ref()), &body)
            .await?;
        CardUpdateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_activate_temp_info(
        &self,
        activate_ticket: &str,
    ) -> Result<WxMpMemberCardActivateTempInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"activate_ticket": activate_ticket});
        let response = svc
            .post(
                &member_card::member_card_activate_temp_info(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpMemberCardActivateTempInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}
