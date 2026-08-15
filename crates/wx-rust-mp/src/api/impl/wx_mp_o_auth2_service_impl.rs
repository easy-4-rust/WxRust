//! OAuth2 网页授权服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpOAuth2ServiceImpl`（实现
//! common 的 `WxOAuth2Service`）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::bean::WxOAuth2UserInfo;
use wx_rust_common::bean::oauth2::WxOAuth2AccessToken;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::service::WxOAuth2Service;

use crate::api::WxMpService;
use crate::enums::wx_mp_api_url::oauth2 as oauth2_url;

/// OAuth2 网页授权服务实现。
pub struct WxMpOAuth2ServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpOAuth2ServiceImpl {
    /// 构建 OAuth2 服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxOAuth2Service for WxMpOAuth2ServiceImpl {
    fn build_authorization_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String {
        // 对应 Java buildAuthorizationUrl：open 域名 + connect/oauth2/authorize
        self.service
            .upgrade()
            .map(|svc| svc.build_qr_connect_url(redirect_uri, scope, state))
            .unwrap_or_default()
    }

    async fn get_access_token(&self, code: &str) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let app_id = config.app_id();
        let secret = config.secret();
        let url = oauth2_url::sns_oauth2_access_token(config.as_ref(), app_id, secret, code);
        let response = svc.get(&url, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_access_token_with(
        &self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = oauth2_url::sns_oauth2_access_token(config.as_ref(), app_id, app_secret, code);
        let response = svc.get(&url, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let app_id = config.app_id();
        let url = oauth2_url::sns_oauth2_refresh_token(config.as_ref(), app_id, refresh_token);
        let response = svc.get(&url, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_user_info(
        &self,
        token: &WxOAuth2AccessToken,
        lang: &str,
    ) -> Result<WxOAuth2UserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        // URL 已内嵌 access_token（Java OAuth2 走独立请求路径，不经 token 注入），
        // 直连 http_client 绕开门面执行器的 token 注入守卫
        let url = oauth2_url::sns_userinfo(config.as_ref(), &token.access_token, lang);
        let response = Self::get_raw(svc.as_ref(), &url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

impl WxMpOAuth2ServiceImpl {
    /// 直连 GET（URL 已含 access_token，绕过执行器注入守卫）。
    async fn get_raw(svc: &dyn WxMpService, url: &str) -> Result<String, WxErrorException> {
        let text = svc
            .http_client()
            .get(url)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("OAuth2 请求失败: {e}")))?
            .text()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("OAuth2 请求失败: {e}")))?;
        let error = wx_rust_common::error::WxError::from_json_with_type(
            &text,
            Some(wx_rust_common::enums::WxType::Mp),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(text)
    }
}
