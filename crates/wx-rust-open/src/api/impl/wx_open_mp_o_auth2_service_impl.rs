//! 第三方平台代公众号 oauth2 服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMpOAuth2ServiceImpl`
//! （`extends WxOAuth2ServiceDecorator`，构造
//! `new WxOpenMpOAuth2ServiceImpl(wxOpenComponentService, wxOAuth2Service,
//! wxMpConfigStorage)`）。
//!
//! Java 装饰器语义：`getAccessToken(code)`/`buildAuthorizationUrl(...)`
//! 覆写为 component 链路（代公众号网页授权），其余方法（
//! `getAccessTokenWith`/`refreshAccessToken`/`getUserInfo` 等）委托内层
//! `wxOAuth2Service`；Rust 以 [`WxOAuth2Service`] trait 实现 + 内层
//! `Arc<dyn WxOAuth2Service>` 委托表达同一语义。
//!
//! component 链路（对应 Java `WxOpenComponentService.OAUTH2_ACCESS_TOKEN_URL` /
//! `CONNECT_OAUTH2_AUTHORIZE_URL`）：
//! - `getAccessToken(code)`：GET `/sns/oauth2/component/access_token?appid=
//!   {mpAppId}&code={code}&grant_type=authorization_code&component_appid=
//!   {componentAppId}`，经组件服务 `get`（注入 component_access_token，
//!   镜像 Java `wxOpenComponentService.get(url)`）；
//! - `buildAuthorizationUrl`：`CONNECT_OAUTH2_AUTHORIZE_URL` 格式化
//!   （redirect_uri 经 JS `encodeURIComponent` 语义编码、state 经 trim）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::bean::WxOAuth2UserInfo;
use wx_rust_common::bean::oauth2::WxOAuth2AccessToken;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::service::WxOAuth2Service;

use crate::api::WxOpenService;
use crate::enums::url_ma_domain::{
    connect_oauth2_authorize_url, oauth2_component_access_token_url,
};

/// 第三方平台代公众号 oauth2 服务实现（对应 Java
/// `WxOpenMpOAuth2ServiceImpl`）。
pub struct WxOpenMpOAuth2ServiceImpl {
    /// 门面服务弱引用（对应 Java 持有 `WxOpenComponentService`；
    /// Rust 以弱引用打破循环，ADAPTED）。
    wx_open_service: Weak<dyn WxOpenService>,
    /// 内层公众号 oauth2 服务（对应 Java 装饰器 `wxOAuth2Service`）。
    inner: Arc<dyn WxOAuth2Service>,
    /// 授权方公众号 appid（对应 Java `wxMpConfigStorage.getAppId()`）。
    app_id: String,
}

impl WxOpenMpOAuth2ServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMpOAuth2ServiceImpl(WxOpenComponentService,
    /// WxOAuth2Service, WxMpConfigStorage)`；Rust 以门面弱引用 + appid
    /// 表达组件服务与 mp 配置，ADAPTED）。
    ///
    /// # 参数
    /// - `wx_open_service`：门面服务强引用（内部降级为弱引用）
    /// - `inner`：内层公众号 oauth2 服务（装饰器委托目标）
    /// - `app_id`：授权方公众号 appid
    pub fn new(
        wx_open_service: Arc<dyn WxOpenService>,
        inner: Arc<dyn WxOAuth2Service>,
        app_id: String,
    ) -> Self {
        Self {
            wx_open_service: Arc::downgrade(&wx_open_service),
            inner,
            app_id,
        }
    }

    /// 组件配置存储的 component_appid（对应 Java
    /// `wxOpenComponentService.getWxOpenConfigStorage().getComponentAppId()`）。
    fn component_app_id(&self) -> Result<String, WxErrorException> {
        let svc = self
            .wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))?;
        Ok(svc
            .wx_open_config_storage()
            .component_app_id()
            .unwrap_or_default())
    }
}

#[async_trait]
impl WxOAuth2Service for WxOpenMpOAuth2ServiceImpl {
    /// 代公众号发起网页授权（对应 Java 覆写 `getAccessToken(String
    /// code)`：component 链路 GET，返回 `WxOAuth2AccessToken`）。
    async fn get_access_token(&self, code: &str) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self
            .wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))?;
        let component = svc.wx_open_component_service().ok_or_else(|| {
            WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )
        })?;
        let config = svc.wx_open_config_storage();
        let url = oauth2_component_access_token_url(config.as_ref(), &self.app_id, code);
        // 镜像 Java `wxOpenComponentService.get(url)`：组件服务 get 注入
        // component_access_token
        let response = component.get(&url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 构建网页授权 URL（对应 Java 覆写 `buildAuthorizationUrl(...)`：
    /// `CONNECT_OAUTH2_AUTHORIZE_URL` 格式化——redirect_uri 经 JS
    /// `encodeURIComponent` 语义编码、state 经 trim）。
    fn build_authorization_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String {
        use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
        let encoded_redirect = utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string();
        let component_app_id = self.component_app_id().unwrap_or_default();
        // Java `StringUtils.trimToEmpty(state)`
        let state = state.trim();
        connect_oauth2_authorize_url(
            &self.app_id,
            &encoded_redirect,
            scope,
            state,
            &component_app_id,
        )
    }

    /// 用 code 获取 access token（指定 appid/secret）。
    ///
    /// Java 装饰器未覆写，委托内层 `wxOAuth2Service`。
    async fn get_access_token_with(
        &self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        self.inner
            .get_access_token_with(app_id, app_secret, code)
            .await
    }

    /// 刷新 access token。
    ///
    /// Java 装饰器未覆写，委托内层 `wxOAuth2Service`。
    async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        self.inner.refresh_access_token(refresh_token).await
    }

    /// 获取用户信息。
    ///
    /// Java 装饰器未覆写，委托内层 `wxOAuth2Service`。
    async fn get_user_info(
        &self,
        token: &WxOAuth2AccessToken,
        lang: &str,
    ) -> Result<WxOAuth2UserInfo, WxErrorException> {
        self.inner.get_user_info(token, lang).await
    }
}
