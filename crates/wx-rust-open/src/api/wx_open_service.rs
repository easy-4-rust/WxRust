//! 开放平台（第三方平台）服务门面。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenService`（7 个方法）+
//! `WxOpenServiceAbstractImpl`（执行引擎）在门面上暴露的方法，并叠加
//! open 模块特有的组件能力入口（对应 Java `WxOpenComponentService` 的
//! component 预授权码 / component_access_token / 三方 token 刷新 / POST
//! 回调消息解密，见任务约定）。
//!
//! Java 继承链（`WxOpenServiceImpl` → `WxOpenServiceHttpComponentsImpl` →
//! `WxOpenServiceAbstractImpl`）在 Rust 以 trait 默认实现 + 组合表达
//! （与 mp/ma 模块同一设计原则）。执行引擎（指数退避重试 + 自动刷新 +
//! component_access_token 注入）在 `crate::api::impl::base_wx_open_service_impl`
//! 中以泛型自由函数承载（trait 无法携带泛型方法，破坏 dyn 兼容）。
//!
//! 说明：
//! - Java `get(String, String)`/`post(String, String)` 为裸请求（
//!   `WxOpenServiceAbstractImpl.execute` 直调执行器）；Rust 侧与 mp/ma
//!   执行引擎对齐，门面 get/post 自动注入 `component_access_token`
//!   （ADAPTED，token 注入在 Java 中位于组件服务层）。
//! - 组件子服务 getter 默认返回 `None`，`WxOpenServiceImpl` 覆写为装配后的
//!   实例。

use std::sync::Arc;

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMinishopImageUploadResult;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::{SimpleGetRequestExecutor, SimplePostRequestExecutor};

use crate::api::WxOpenComponentService;
use crate::bean::message::WxOpenXmlMessage;
use crate::config::WxOpenConfigStorage;
use crate::constant::wx_open_constants::ACCESS_TOKEN_KEY_COMPONENT;

/// 开放平台（第三方平台）服务门面。
#[async_trait]
pub trait WxOpenService: Send + Sync {
    // ---- 配置与子服务（对应 Java WxOpenService） ----

    /// 组件子服务（对应 Java `getWxOpenComponentService()`）。
    ///
    /// 默认返回 `None`，`WxOpenServiceImpl` 覆写为装配后的实例。
    fn wx_open_component_service(&self) -> Option<Arc<dyn WxOpenComponentService>> {
        None
    }

    /// 配置存储（对应 Java `getWxOpenConfigStorage()`）。
    fn wx_open_config_storage(&self) -> Arc<dyn WxOpenConfigStorage>;

    /// 设置配置存储（对应 Java `setWxOpenConfigStorage(WxOpenConfigStorage)`）。
    fn set_wx_open_config_storage(&self, wx_open_config_storage: Arc<dyn WxOpenConfigStorage>);

    /// HTTP 客户端（对应 Java `RequestHttp.getRequestHttpClient()`）。
    fn http_client(&self) -> &reqwest::Client;

    // ---- 核心能力（对应 Java WxOpenService get/post + 执行引擎） ----

    /// GET 请求（对应 Java `get(String, String)`）。
    ///
    /// 当本 Service 没有实现某个 API 时可用，针对所有微信 API 中的 GET
    /// 请求；执行引擎自动附加 `component_access_token`。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_open_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            query_param.to_string(),
            ACCESS_TOKEN_KEY_COMPONENT,
        )
        .await
    }

    /// POST 请求（对应 Java `post(String, String)`）。
    ///
    /// 当本 Service 没有实现某个 API 时可用，针对所有微信 API 中的 POST
    /// 请求；执行引擎自动附加 `component_access_token`。
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_open_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            post_data.to_string(),
            ACCESS_TOKEN_KEY_COMPONENT,
        )
        .await
    }

    /// 上传图片到小程序/开放平台素材库（对应 Java
    /// `uploadMinishopMediaFile(String url, File file)`）。
    ///
    /// ADAPTED：Java 以 `File` 入参，Rust 以文件路径字符串表达；需要
    /// `MinishopUploadRequestExecutor`（multipart 上传 + JSON 结果解析），
    /// Wave 1 实现，当前返回未实现错误。
    async fn upload_minishop_media_file(
        &self,
        _url: &str,
        _file_path: &str,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "upload_minishop_media_file 未实现（Wave 1：MinishopUploadRequestExecutor）",
        ))
    }

    // ---- 开放平台特殊能力入口（对应 Java WxOpenComponentService 核心方法，
    // 任务约定在门面暴露；默认委托组件子服务，未装配时返回错误） ----

    /// 获取 component_access_token（对应 Java
    /// `WxOpenComponentService.getComponentAccessToken(boolean)`，
    /// 开放平台接口调用凭据）。
    async fn get_component_access_token(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => svc.get_component_access_token(force_refresh).await,
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }

    /// 获取预授权码（对应 Java `WxOpenComponentServiceImpl.createPreAuthUrl`
    /// 内部 POST `API_CREATE_PREAUTHCODE_URL` 的 `pre_auth_code` 字段）。
    async fn get_pre_auth_code(&self) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => svc.get_pre_auth_code().await,
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }

    /// 获取网页授权预授权链接（对应 Java `WxOpenComponentService.getPreAuthUrl(String)`）。
    async fn get_pre_auth_url(&self, redirect_uri: &str) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => svc.get_pre_auth_url(redirect_uri).await,
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }

    /// 获取网页授权预授权链接（对应 Java
    /// `WxOpenComponentService.getPreAuthUrl(String, String, String)`）。
    async fn get_pre_auth_url_with(
        &self,
        redirect_uri: &str,
        auth_type: Option<&str>,
        biz_appid: Option<&str>,
    ) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => {
                svc.get_pre_auth_url_with(redirect_uri, auth_type, biz_appid)
                    .await
            }
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }

    /// 获取移动端预授权链接（对应 Java
    /// `WxOpenComponentService.getMobilePreAuthUrl(String)`）。
    async fn get_mobile_pre_auth_url(
        &self,
        redirect_uri: &str,
    ) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => svc.get_mobile_pre_auth_url(redirect_uri).await,
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }

    /// 获取移动端预授权链接（对应 Java
    /// `WxOpenComponentService.getMobilePreAuthUrl(String, String, String)`）。
    async fn get_mobile_pre_auth_url_with(
        &self,
        redirect_uri: &str,
        auth_type: Option<&str>,
        biz_appid: Option<&str>,
    ) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => {
                svc.get_mobile_pre_auth_url_with(redirect_uri, auth_type, biz_appid)
                    .await
            }
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }

    /// 获取（刷新）授权方 access_token（对应 Java
    /// `WxOpenComponentService.getAuthorizerAccessToken(String, boolean)`，
    /// 三方 token 刷新：component_access_token + authorizer refresh_token
    /// 换新 token）。
    async fn get_authorizer_access_token(
        &self,
        app_id: &str,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => svc.get_authorizer_access_token(app_id, force_refresh).await,
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }

    /// 路由第三方平台推送的加密回调消息（对应 Java
    /// `WxOpenComponentService.route(WxOpenXmlMessage)`，POST 回调消息
    /// 解密 + 分发）。
    ///
    /// 入参为解密后的 [`WxOpenXmlMessage`]（解密经
    /// `WxOpenXmlMessage::from_encrypted_xml` 完成）。
    async fn route(&self, message: &WxOpenXmlMessage) -> Result<String, WxErrorException> {
        match self.wx_open_component_service() {
            Some(svc) => svc.route(message).await,
            None => Err(WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )),
        }
    }
}
