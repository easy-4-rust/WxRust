//! OAuth2 服务接口。
//!
//! 对应 Java `me.chanjar.weixin.common.service.WxOAuth2Service`。

use async_trait::async_trait;

use crate::bean::WxOAuth2UserInfo;
use crate::bean::oauth2::WxOAuth2AccessToken;
use crate::error::WxErrorException;

/// OAuth2 网页授权服务接口。
#[async_trait]
pub trait WxOAuth2Service: Send + Sync {
    /// 构造网页授权 URL。
    ///
    /// # 参数
    /// - `redirect_uri`：授权后重定向的 URI
    /// - `scope`：授权作用域（`snsapi_base`/`snsapi_userinfo` 等）
    /// - `state`：重定向后会带上 state 参数（防 CSRF）
    ///
    /// # 返回
    /// 授权 URL
    fn build_authorization_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String;

    /// 通过 code 获取 access token（使用默认 appId/appSecret）。
    ///
    /// # 参数
    /// - `code`：授权回调中的 code
    ///
    /// # 返回
    /// OAuth2 access token
    async fn get_access_token(&self, code: &str) -> Result<WxOAuth2AccessToken, WxErrorException>;

    /// 通过 code 获取 access token（指定 appId/appSecret）。
    ///
    /// # 参数
    /// - `app_id`：appId
    /// - `app_secret`：appSecret
    /// - `code`：授权回调中的 code
    ///
    /// # 返回
    /// OAuth2 access token
    async fn get_access_token_with(
        &self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException>;

    /// 刷新 access token。
    ///
    /// # 参数
    /// - `refresh_token`：refresh token
    ///
    /// # 返回
    /// 新的 OAuth2 access token
    async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException>;

    /// 获取用户信息。
    ///
    /// # 参数
    /// - `access_token`：OAuth2 access token
    /// - `lang`：语言（如 `zh_CN`）
    ///
    /// # 返回
    /// 用户信息
    async fn get_user_info(
        &self,
        access_token: &WxOAuth2AccessToken,
        lang: &str,
    ) -> Result<WxOAuth2UserInfo, WxErrorException>;
}
