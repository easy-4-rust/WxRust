//! OAuth2 相关管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOAuth2Service`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpOauth2UserInfo, WxCpSecondVerificationInfo, WxCpUserDetail};

/// OAuth2 相关管理服务。
#[async_trait]
pub trait WxCpOAuth2Service: Send + Sync {
    /// 构造 oauth2 授权的 url 连接（对应 Java
    /// `WxCpOAuth2Service.buildAuthorizationUrl(String)`；纯本地构造，
    /// 不抛异常，同步方法）。
    fn build_authorization_url(&self, state: &str) -> String;

    /// 构造 oauth2 授权的 url 连接（对应 Java
    /// `WxCpOAuth2Service.buildAuthorizationUrl(String, String)`）。
    fn build_authorization_url_with_redirect_uri(&self, redirect_uri: &str, state: &str) -> String;

    /// 构造 oauth2 授权的 url 连接（对应 Java
    /// `WxCpOAuth2Service.buildAuthorizationUrl(String, String, String)`；
    /// `scope` 取值参考 `WxConsts.OAuth2Scope`）。
    fn build_authorization_url_with_scope(
        &self,
        redirect_uri: &str,
        state: &str,
        scope: &str,
    ) -> String;

    /// 用 oauth2 获取用户信息（对应 Java
    /// `WxCpOAuth2Service.getUserInfo(String)`；使用配置里的 agentId）。
    async fn get_user_info(&self, code: &str) -> Result<WxCpOauth2UserInfo, WxErrorException>;

    /// 根据 code 获取成员信息（对应 Java
    /// `WxCpOAuth2Service.getUserInfo(Integer, String)`；不使用配置里的
    /// agentId，由调用方给出）。
    async fn get_user_info_with_agent_id(
        &self,
        agent_id: i32,
        code: &str,
    ) -> Result<WxCpOauth2UserInfo, WxErrorException>;

    /// 获取家校访问用户身份（对应 Java
    /// `WxCpOAuth2Service.getSchoolUserInfo(String)`）。
    async fn get_school_user_info(
        &self,
        code: &str,
    ) -> Result<WxCpOauth2UserInfo, WxErrorException>;

    /// 使用 user_ticket 获取成员详情（对应 Java
    /// `WxCpOAuth2Service.getUserDetail(String)`）。
    async fn get_user_detail(&self, user_ticket: &str) -> Result<WxCpUserDetail, WxErrorException>;

    /// 获取用户登录身份（对应 Java
    /// `WxCpOAuth2Service.getAuthUserInfo(String)`）。
    async fn get_auth_user_info(&self, code: &str) -> Result<WxCpOauth2UserInfo, WxErrorException>;

    /// 获取用户二次验证信息（对应 Java
    /// `WxCpOAuth2Service.getTfaInfo(String)`）。
    async fn get_tfa_info(
        &self,
        code: &str,
    ) -> Result<WxCpSecondVerificationInfo, WxErrorException>;
}
