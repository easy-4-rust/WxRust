//! OAuth2 相关管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOAuth2ServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use wx_rust_common::api::wx_consts::oauth2_scope::{
    SNSAPI_BASE, SNSAPI_PRIVATEINFO, SNSAPI_USERINFO,
};
use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOAuth2Service, WxCpService};
use crate::bean::{WxCpOauth2UserInfo, WxCpSecondVerificationInfo, WxCpUserDetail};
use crate::enums::url_oauth2::*;

/// OAuth2 相关管理服务实现。
pub struct WxCpOAuth2ServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOAuth2ServiceImpl {
    /// 构建 OAuth2 服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpOAuth2Service for WxCpOAuth2ServiceImpl {
    fn build_authorization_url(&self, state: &str) -> String {
        // Java `buildAuthorizationUrl(String state)`：redirectUri 取配置
        // `getOauth2redirectUri()`
        let redirect_uri = self
            .service
            .upgrade()
            .map(|svc| {
                svc.wx_cp_config_storage()
                    .oauth2_redirect_uri()
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        self.build_authorization_url_with_scope(&redirect_uri, state, SNSAPI_BASE)
    }

    fn build_authorization_url_with_redirect_uri(&self, redirect_uri: &str, state: &str) -> String {
        // Java `buildAuthorizationUrl(String, String)`：scope 恒为 snsapi_base
        self.build_authorization_url_with_scope(redirect_uri, state, SNSAPI_BASE)
    }

    fn build_authorization_url_with_scope(
        &self,
        redirect_uri: &str,
        state: &str,
        scope: &str,
    ) -> String {
        // Java `buildAuthorizationUrl(String, String, String)`：URL 按序拼接
        // `appid`/`redirect_uri`（URL 编码）/`response_type=code`/`scope`；
        // scope 为 snsapi_privateinfo / snsapi_userinfo 时追加 `agentid`；
        // state 非 null 追加；最后 `#wechat_redirect`
        let (app_id, agent_id) = self
            .service
            .upgrade()
            .map(|svc| {
                let config = svc.wx_cp_config_storage();
                let app_id = config.app_id().to_string();
                let agent_id = config.agent_id();
                (app_id, agent_id)
            })
            .unwrap_or_default();
        let mut url = String::from(URL_OAUTH2_AUTHORIZE);
        url.push_str("?appid=");
        url.push_str(&app_id);
        url.push_str("&redirect_uri=");
        url.push_str(&utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string());
        url.push_str("&response_type=code");
        url.push_str("&scope=");
        url.push_str(scope);

        if scope == SNSAPI_PRIVATEINFO || scope == SNSAPI_USERINFO {
            url.push_str("&agentid=");
            url.push_str(&agent_id.unwrap_or_default().to_string());
        }

        if !state.is_empty() {
            url.push_str("&state=");
            url.push_str(state);
        }

        url.push_str("#wechat_redirect");
        url
    }

    async fn get_user_info(&self, code: &str) -> Result<WxCpOauth2UserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserInfo(String)`：agentId 取配置
        let config = svc.wx_cp_config_storage();
        let agent_id = config.agent_id().unwrap_or(0);
        self.get_user_info_with_agent_id(agent_id, code).await
    }

    async fn get_user_info_with_agent_id(
        &self,
        agent_id: i32,
        code: &str,
    ) -> Result<WxCpOauth2UserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserInfo(Integer, String)`：GET
        // `String.format(GET_USER_INFO, code, agentId)`（`%s`/`%d` 替换），
        // 字段映射 UserId 优先、其次 userid；OpenId 优先、其次 openid
        let config = svc.wx_cp_config_storage();
        let url = config
            .api_url(GET_USER_INFO)
            .replace("%s", code)
            .replace("%d", &agent_id.to_string());
        let response_text = svc.get(&url, "").await?;
        parse_user_info(&response_text, true)
    }

    async fn get_school_user_info(
        &self,
        code: &str,
    ) -> Result<WxCpOauth2UserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getSchoolUserInfo`：GET
        // `String.format(GET_SCHOOL_USER_INFO, code)`，仅映射
        // DeviceId/parent_userid/student_userid
        let config = svc.wx_cp_config_storage();
        let url = config.api_url(GET_SCHOOL_USER_INFO).replace("%s", code);
        let response_text = svc.get(&url, "").await?;
        let json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(WxCpOauth2UserInfo {
            open_id: String::new(),
            device_id: str_field(&json, "DeviceId"),
            user_id: String::new(),
            user_ticket: String::new(),
            expires_in: String::new(),
            external_user_id: String::new(),
            parent_user_id: str_field(&json, "parent_userid"),
            student_user_id: str_field(&json, "student_userid"),
        })
    }

    async fn get_user_detail(&self, user_ticket: &str) -> Result<WxCpUserDetail, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserDetail`：POST `GET_USER_DETAIL` `{"user_ticket":...}`，
        // 整体响应解析为 `WxCpUserDetail`
        let body = serde_json::json!({ "user_ticket": user_ticket }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_text = svc.post(&config.api_url(GET_USER_DETAIL), &body).await?;
        serde_json::from_str::<WxCpUserDetail>(&response_text)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_auth_user_info(&self, code: &str) -> Result<WxCpOauth2UserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getAuthUserInfo`：GET `String.format(GET_USER_AUTH_INFO, code)`，
        // 映射 userid/openid/user_ticket/external_userid
        let config = svc.wx_cp_config_storage();
        let url = config.api_url(GET_USER_AUTH_INFO).replace("%s", code);
        let response_text = svc.get(&url, "").await?;
        let json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(WxCpOauth2UserInfo {
            open_id: str_field(&json, "openid"),
            device_id: String::new(),
            user_id: str_field(&json, "userid"),
            user_ticket: str_field(&json, "user_ticket"),
            expires_in: String::new(),
            external_user_id: str_field(&json, "external_userid"),
            parent_user_id: String::new(),
            student_user_id: String::new(),
        })
    }

    async fn get_tfa_info(
        &self,
        code: &str,
    ) -> Result<WxCpSecondVerificationInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getTfaInfo`：POST `GET_TFA_INFO` `{"code":...}`，整体响应
        // 解析为 `WxCpSecondVerificationInfo`
        let body = serde_json::json!({ "code": code }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_text = svc.post(&config.api_url(GET_TFA_INFO), &body).await?;
        serde_json::from_str::<WxCpSecondVerificationInfo>(&response_text)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

/// 解析 `getUserInfo` 响应为 `WxCpOauth2UserInfo`（对应 Java builder 逻辑：
/// `UserId` 优先、其次 `userid`；`OpenId` 优先、其次 `openid`；另含
/// `DeviceId`/`user_ticket`/`expires_in`/`external_userid`/
/// `parent_userid`/`student_userid`）。
fn parse_user_info(
    response_text: &str,
    with_user_id: bool,
) -> Result<WxCpOauth2UserInfo, WxErrorException> {
    let json: serde_json::Value =
        serde_json::from_str(response_text).map_err(|e| WxErrorException::Serde(e.to_string()))?;
    let user_id = if with_user_id {
        // Optional.ofNullable(UserId).orElse(userid)
        let v = str_field(&json, "UserId");
        if v.is_empty() {
            str_field(&json, "userid")
        } else {
            v
        }
    } else {
        String::new()
    };
    let open_id = {
        // Optional.ofNullable(OpenId).orElse(openid)
        let v = str_field(&json, "OpenId");
        if v.is_empty() {
            str_field(&json, "openid")
        } else {
            v
        }
    };
    Ok(WxCpOauth2UserInfo {
        open_id,
        device_id: str_field(&json, "DeviceId"),
        user_id,
        user_ticket: str_field(&json, "user_ticket"),
        expires_in: str_field(&json, "expires_in"),
        external_user_id: str_field(&json, "external_userid"),
        parent_user_id: str_field(&json, "parent_userid"),
        student_user_id: str_field(&json, "student_userid"),
    })
}

/// 取字符串字段（缺失/非字符串返回空串，对应 Gson `getAsString` 的
/// 宽松语义在字段缺失时返回 null → builder 空值）。
fn str_field(json: &serde_json::Value, field: &str) -> String {
    json.get(field)
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string()
}
