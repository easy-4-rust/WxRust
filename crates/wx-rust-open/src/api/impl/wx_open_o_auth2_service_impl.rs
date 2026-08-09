//! oauth2 网页授权服务实现（普通 appid/secret 链路）。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenOAuth2ServiceImpl`
//! （`extends WxOpenServiceImpl implements WxOAuth2Service`，构造
//! `new WxOpenOAuth2ServiceImpl(appId, appSecret, openConfigStorage)`）。
//!
//! Java 经 `this.get(url, null)`（`WxOpenServiceAbstractImpl` 裸 GET，
//! 不做 token 注入）执行；Rust 以门面 `http_client` + 裸 GET 执行器
//! 承载（ADAPTED：不经过门面 get 的 component_access_token 注入——OAuth2
//! URL 已内嵌 appid/secret 或 access_token）。
//!
//! 与组件服务内联 oauth2（[`crate::api::WxOpenComponentService`] 的
//! `oauth2_get_access_token`/`oauth2_refresh_access_token`）的关系：
//! Java 组件服务方法为 component 链路（`/sns/oauth2/component/access_token`，
//! `@Deprecated`）；本服务为普通链路（`/sns/oauth2/access_token`，
//! appid/secret 直换），两者 URL 与语义均不同，Java 文档建议改用
//! `getWxMpServiceByAppid(mpAppId).getOAuth2Service()`（即
//! [`crate::api::r#impl::WxOpenMpOAuth2ServiceImpl`]）。
//!
//! `validateAccessToken` 不在 Rust `WxOAuth2Service` trait 方法面内
//! （trait 冻结时未收录），以固有方法承载（ADAPTED）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::bean::WxOAuth2UserInfo;
use wx_rust_common::bean::oauth2::WxOAuth2AccessToken;
use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::service::WxOAuth2Service;
use wx_rust_common::util::http::{RequestExecutor, SimpleGetRequestExecutor};

use crate::api::WxOpenService;
use crate::enums::url_ma_domain::{
    oauth2_access_token_url, oauth2_refresh_token_url, oauth2_userinfo_url,
    oauth2_validate_token_url, qrconnect_url,
};

/// oauth2 网页授权服务实现（对应 Java `WxOpenOAuth2ServiceImpl`）。
pub struct WxOpenOAuth2ServiceImpl {
    /// 授权方 appid（Java 构造入参）。
    app_id: String,
    /// 授权方 appsecret（Java 构造入参）。
    app_secret: String,
    /// 门面服务弱引用（对应 Java 继承 `WxOpenServiceImpl` 持有的
    /// `WxOpenConfigStorage`；Rust 以弱引用 + http_client 表达，ADAPTED）。
    wx_open_service: Weak<dyn WxOpenService>,
}

impl WxOpenOAuth2ServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenOAuth2ServiceImpl(String appId, String appSecret,
    /// WxOpenConfigStorage openConfigStorage)`）。
    ///
    /// # 参数
    /// - `app_id`：授权方 appid
    /// - `app_secret`：授权方 appsecret
    /// - `wx_open_service`：门面服务强引用（内部降级为弱引用）
    pub fn new(
        app_id: String,
        app_secret: String,
        wx_open_service: Arc<dyn WxOpenService>,
    ) -> Self {
        Self {
            app_id,
            app_secret,
            wx_open_service: Arc::downgrade(&wx_open_service),
        }
    }

    /// 裸 GET（对应 Java `WxOpenServiceAbstractImpl.get(String, String)`：
    /// 直调执行器，不做 token 注入；OAuth2 URL 已内嵌 appid/secret 或
    /// access_token）。
    async fn bare_get(&self, url: &str) -> Result<String, WxErrorException> {
        let svc = self
            .wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))?;
        let executor = SimpleGetRequestExecutor::new(svc.http_client().clone());
        executor.execute(url, String::new(), WxType::Open).await
    }

    /// 验证 access token 是否有效（对应 Java
    /// `validateAccessToken(WxOAuth2AccessToken token)`）。
    ///
    /// Java：GET `/sns/auth`，`WxErrorException` 捕获返回 false，
    /// `IOException` 抛 `WxRuntimeException`；Rust 以 HTTP/解析错误抛
    /// `-99`、业务错误返回 false 表达（ADAPTED：Rust 无
    /// `WxRuntimeException`，统一以 `Err` 表达 IO 异常）。
    pub async fn validate_access_token(
        &self,
        token: &WxOAuth2AccessToken,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))?;
        let config = svc.wx_open_config_storage();
        let url = oauth2_validate_token_url(config.as_ref(), &token.access_token, &token.open_id);
        match self.bare_get(&url).await {
            Ok(_) => Ok(true),
            Err(e) if e.error_code() != Some(-99) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl WxOAuth2Service for WxOpenOAuth2ServiceImpl {
    /// 构建网页授权 URL（对应 Java `buildAuthorizationUrl(...)`：
    /// `QRCONNECT_URL` 格式化——redirect_uri 经 JS `encodeURIComponent`
    /// 语义编码、state 经 trim）。
    ///
    /// 注意：Java 使用 `WxMpApiUrl.Other.QRCONNECT_URL`（网站应用授权
    /// 登录链接 `/connect/qrconnect`），Rust 严格镜像。
    fn build_authorization_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String {
        use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
        // Java `URIUtil.encodeURIComponent(redirectUri)`：JS encodeURIComponent 语义
        let encoded_redirect = utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string();
        // Java `StringUtils.trimToEmpty(state)`
        let state = state.trim();
        qrconnect_url(&self.app_id, &encoded_redirect, scope, state)
    }

    /// 用 code 获取 access token（对应 Java
    /// `getAccessToken(String code)` → 委托
    /// `getAccessToken(appId, appSecret, code)`）。
    async fn get_access_token(&self, code: &str) -> Result<WxOAuth2AccessToken, WxErrorException> {
        self.get_access_token_with(&self.app_id, &self.app_secret, code)
            .await
    }

    /// 用 code 获取 access token（对应 Java
    /// `getAccessToken(String appId, String appSecret, String code)`：
    /// GET `/sns/oauth2/access_token?appid=&secret=&code=`）。
    async fn get_access_token_with(
        &self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self
            .wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))?;
        let config = svc.wx_open_config_storage();
        let url = oauth2_access_token_url(config.as_ref(), app_id, app_secret, code);
        let response = self.bare_get(&url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 刷新 access token（对应 Java
    /// `refreshAccessToken(String refreshToken)`：GET
    /// `/sns/oauth2/refresh_token?appid=&grant_type=refresh_token&refresh_token=`）。
    async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self
            .wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))?;
        let config = svc.wx_open_config_storage();
        let url = oauth2_refresh_token_url(config.as_ref(), &self.app_id, refresh_token);
        let response = self.bare_get(&url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取用户信息（对应 Java
    /// `getUserInfo(WxOAuth2AccessToken token, String lang)`：lang 为
    /// null 时默认 `zh_CN`，GET `/sns/userinfo?access_token=&openid=&lang=`）。
    async fn get_user_info(
        &self,
        token: &WxOAuth2AccessToken,
        lang: &str,
    ) -> Result<WxOAuth2UserInfo, WxErrorException> {
        let lang = if lang.is_empty() { "zh_CN" } else { lang };
        let svc = self
            .wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))?;
        let config = svc.wx_open_config_storage();
        let url = oauth2_userinfo_url(config.as_ref(), &token.access_token, &token.open_id, lang);
        let response = self.bare_get(&url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
