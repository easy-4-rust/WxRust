//! 开放平台组件服务。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenComponentService`（1184 行、
//! 100+ 方法）。Wave 0 冻结与「component 预授权码 / component_access_token /
//! 三方 token 刷新 / POST 回调消息解密」相关的核心签名（B0 签名冻结）；
//! Wave 2 补齐其余全部方法签名（授权方信息/选项/列表、代码模板、open
//! 帐号、快速创建、minishop、tcb、oauth2、服务器域名等），默认实现返回
//! `Err(-99)`（待定清单见各方法文档与 `impl` 覆写），核心方法由
//! [`crate::api::impl::WxOpenComponentServiceImpl`] 覆写。
//!
//! 镜像说明：
//! - Java `void` → `Result<(), WxErrorException>`；Java `boolean` →
//!   `bool`；`Integer`/`Long` → `i32`/`i64`（可空取 `Option`）；
//!   `List<T>` → `Vec<T>`（Java 可返回 null 处取 `Option<Vec<T>>`）。
//! - Java `String` 可返回 null（如 `minishopGetCouponList`/`minishopCommonPost`
//!   的 `return null`）→ `Option<String>`/`Option<T>` 镜像同一语义。
//! - Java `File` 入参 → `&str` 文件路径（ADAPTED）。
//! - Java `WxMaJscode2SessionResult`（wx-rust-miniapp 的 bean）→
//!   `serde_json::Value`（ADAPTED，Wave 2 引入 miniapp 依赖后换型）。
//! - Java `WxOpenMpService`/`WxOpenMaService`/`WxOpenFastMaService`/
//!   `WxOpenMinishopService`（代 mp/ma 桥接子服务）→
//!   `Option<Arc<dyn Any + Send + Sync>>`（ADAPTED，待依赖接线，当前恒
//!   返回 `None`；接线后调用方 `downcast_arc` 取具体服务，签名不变）。
//! - Java `getWxOpenConfigStorage()` → [`Self::wx_open_config_storage`]
//!   （可推导的真实默认实现，不占位）。

use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;

use wx_rust_common::bean::oauth2::WxOAuth2AccessToken;
use wx_rust_common::bean::result::WxMinishopImageUploadResult;
use wx_rust_common::error::WxErrorException;

use crate::api::WxOpenService;
use crate::bean::message::WxOpenXmlMessage;
use crate::bean::{
    GetShareCloudBaseEnvResponse, GetTcbEnvListResponse, LimitDiscountGoods, MinishopBrandList,
    MinishopBusiLicense, MinishopCategories, MinishopDeliveryTemplateResult, MinishopIdcardInfo,
    MinishopNameInfo, MinishopOrganizationCodeInfo, MinishopReturnInfo, MinishopShopCatList,
    MinishopSuperAdministratorInfo, ShareCloudBaseEnvRequest, ShareCloudBaseEnvResponse,
    WxMinishopAddGoodsSpuResult, WxMinishopCoupon, WxMinishopCouponStock, WxMinishopSku,
    WxMinishopSpu, WxOpenAuthorizerInfoResult, WxOpenAuthorizerListResult,
    WxOpenAuthorizerOptionResult, WxOpenCreateResult, WxOpenGetResult, WxOpenHaveResult,
    WxOpenMaApplyOrderPathInfo, WxOpenMaCodeTemplate, WxOpenMaDomainConfirmFileResult,
    WxOpenMaDomainResult, WxOpenMaWebDomainResult, WxOpenQueryAuthResult,
    WxOpenRegisterBetaWeappResult, WxOpenRegisterPersonalWeappResult, WxOpenResult,
};
use crate::config::WxOpenConfigStorage;

/// 开放平台组件服务（第三方平台核心子服务）。
#[async_trait]
pub trait WxOpenComponentService: Send + Sync {
    /// 持有的门面服务（对应 Java `getWxOpenService()`）。
    ///
    /// Rust 以弱引用打破循环（Java `new WxOpenComponentServiceImpl(this)`），
    /// 升级失败（门面已释放）时返回 `None`。
    fn wx_open_service(&self) -> Option<Arc<dyn WxOpenService>>;

    /// 校验消息签名（对应 Java `checkSignature(String, String, String)`）。
    ///
    /// SHA1（componentToken + timestamp + nonce 排序后无分隔符拼接）与
    /// signature 比较；验签失败或门面缺失时返回 false。
    fn check_signature(&self, timestamp: &str, nonce: &str, signature: &str) -> bool {
        use wx_rust_common::util::crypto::Sha1;
        match self.wx_open_service() {
            Some(svc) => {
                let config = svc.wx_open_config_storage();
                let token = config.component_token().unwrap_or_default();
                match Sha1::digest(&[token.as_str(), timestamp, nonce]) {
                    Ok(s) => s == signature,
                    Err(_) => false,
                }
            }
            None => false,
        }
    }

    /// 启动 verify ticket 推送服务（对应 Java `startPushTicket()`）。
    async fn start_push_ticket(&self) -> Result<(), WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "start_push_ticket 未实现（Wave 2）",
        ))
    }

    /// 获取 component_access_token（对应 Java
    /// `getComponentAccessToken(boolean forceRefresh)`，开放平台接口调用凭据）。
    ///
    /// 双检锁缓存：未过期直接返回；否则持锁以
    /// component_appid/component_appsecret/component_verify_ticket 调
    /// `api_component_token` 刷新。
    async fn get_component_access_token(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_component_access_token 未实现（Wave 2）",
        ))
    }

    /// POST 请求（对应 Java `post(String, String)`，默认注入键
    /// `component_access_token`）。
    async fn post(&self, _uri: &str, _post_data: &str) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(-99, "post 未实现（Wave 2）"))
    }

    /// POST 请求（对应 Java `post(String, String, String accessTokenKey)`，
    /// 自定义 token 注入键）。
    async fn post_with_key(
        &self,
        _uri: &str,
        _post_data: &str,
        _access_token_key: &str,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "post_with_key 未实现（Wave 2）",
        ))
    }

    /// POST 请求（对应 Java `post(String, String, String accessTokenKey,
    /// String accessToken)`，调用方显式传 token，不做自动刷新）。
    async fn post_with_token(
        &self,
        _uri: &str,
        _post_data: &str,
        _access_token_key: &str,
        _access_token: &str,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "post_with_token 未实现（Wave 2）",
        ))
    }

    /// GET 请求（对应 Java `get(String uri)`，默认注入键
    /// `component_access_token`）。
    async fn get(&self, _uri: &str) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(-99, "get 未实现（Wave 2）"))
    }

    /// GET 请求（对应 Java `get(String uri, String accessTokenKey)`）。
    async fn get_with_key(
        &self,
        _uri: &str,
        _access_token_key: &str,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_with_key 未实现（Wave 2）",
        ))
    }

    /// 获取预授权码（对应 Java `WxOpenComponentServiceImpl.createPreAuthUrl`
    /// 内部 POST `API_CREATE_PREAUTHCODE_URL` 的 `pre_auth_code` 字段）。
    async fn get_pre_auth_code(&self) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_pre_auth_code 未实现（Wave 2）",
        ))
    }

    /// 获取网页授权预授权链接（对应 Java `getPreAuthUrl(String redirectUri)`）。
    async fn get_pre_auth_url(&self, _redirect_uri: &str) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_pre_auth_url 未实现（Wave 2）",
        ))
    }

    /// 获取网页授权预授权链接（对应 Java
    /// `getPreAuthUrl(String redirectUri, String authType, String bizAppid)`）。
    async fn get_pre_auth_url_with(
        &self,
        _redirect_uri: &str,
        _auth_type: Option<&str>,
        _biz_appid: Option<&str>,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_pre_auth_url_with 未实现（Wave 2）",
        ))
    }

    /// 获取移动端预授权链接（对应 Java `getMobilePreAuthUrl(String redirectUri)`）。
    async fn get_mobile_pre_auth_url(
        &self,
        _redirect_uri: &str,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_mobile_pre_auth_url 未实现（Wave 2）",
        ))
    }

    /// 获取移动端预授权链接（对应 Java
    /// `getMobilePreAuthUrl(String redirectUri, String authType, String bizAppid)`）。
    async fn get_mobile_pre_auth_url_with(
        &self,
        _redirect_uri: &str,
        _auth_type: Option<&str>,
        _biz_appid: Option<&str>,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_mobile_pre_auth_url_with 未实现（Wave 2）",
        ))
    }

    /// 获取（刷新）授权方 access_token（对应 Java
    /// `getAuthorizerAccessToken(String appid, boolean forceRefresh)`，
    /// 三方 token 刷新链：refresh_token 换新 token）。
    async fn get_authorizer_access_token(
        &self,
        _app_id: &str,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_authorizer_access_token 未实现（Wave 2）",
        ))
    }

    /// 路由第三方平台推送的加密回调消息（对应 Java
    /// `route(WxOpenXmlMessage wxMessage)`，POST 回调消息解密 + 分发）。
    ///
    /// 入参为解密后的 [`WxOpenXmlMessage`]（解密经
    /// `WxOpenXmlMessage::from_encrypted_xml` 完成，对应 Java 调用方先用
    /// `fromEncryptedXml` 解密再 `route` 的流程）；Wave 2 实现。
    async fn route(&self, _message: &WxOpenXmlMessage) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(-99, "route 未实现（Wave 2）"))
    }

    // ---- 配置与子服务（对应 Java WxOpenComponentService 同名列） ----

    /// 配置存储（对应 Java `getWxOpenConfigStorage()`，委托门面）。
    ///
    /// 门面已释放时返回 `None`（Java 强引用不可能为空，ADAPTED）。
    fn wx_open_config_storage(&self) -> Option<Arc<dyn WxOpenConfigStorage>> {
        self.wx_open_service()
            .map(|svc| svc.wx_open_config_storage())
    }

    /// 获取指定 appid 的开放平台公众号服务（对应 Java
    /// `getWxMpServiceByAppid(String appid)`，双检锁缓存按 appid 装配
    /// `WxOpenMpServiceImpl`）。
    ///
    /// ADAPTED（待依赖接线）：wx-rust-open 尚未依赖 wx-rust-mp，恒返回
    /// `None`；Wave 2+ 接线方案：Cargo.toml 引入 wx-rust-mp 后以
    /// `Arc<dyn WxMpService>` 装配 `WxOpenMpService`，调用方
    /// `downcast_arc::<dyn WxMpService>()` 取服务，本签名不变。
    fn get_wx_mp_service_by_appid(&self, _app_id: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        None
    }

    /// 获取指定 appid 的开放平台小程序服务（对应 Java
    /// `getWxMaServiceByAppid(String appid)`，继承一般小程序服务能力）。
    ///
    /// 接线说明同 [`Self::get_wx_mp_service_by_appid`]（待依赖接线，
    /// 恒返回 `None`）。
    fn get_wx_ma_service_by_appid(&self, _app_id: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        None
    }

    /// 获取指定 appid 的快速创建的小程序服务（对应 Java
    /// `getWxFastMaServiceByAppid(String appid)`）。
    ///
    /// Java `@Deprecated`（2021-06-23：本接口原有方法并非仅快速创建小程序的
    /// 专用接口，请使用 `WxOpenMaService.getBasicService()`）。接线说明同
    /// [`Self::get_wx_mp_service_by_appid`]（恒返回 `None`）。
    fn get_wx_fast_ma_service_by_appid(&self, _app_id: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        None
    }

    /// 获取指定 appid 的小商店服务（对应 Java
    /// `getWxMinishopServiceByAppid(String appid)`）。
    ///
    /// Wave 5 已接线：实现侧按 appid 双检锁缓存装配
    /// [`crate::api::r#impl::WxOpenMinishopServiceImpl`]（镜像 Java 静态
    /// `WX_OPEN_MINISHOP_SERVICE_MAP`）；返回值经 downcast 下转（本 trait
    /// 默认实现仍为 `None`，由组件实现覆写）。
    fn get_wx_minishop_service_by_appid(
        &self,
        _app_id: &str,
    ) -> Option<Arc<dyn Any + Send + Sync>> {
        None
    }

    // ---- 授权方信息/选项/列表（对应 Java 同名方法） ----

    /// 使用授权码换取公众号或小程序的接口调用凭据和授权信息（对应 Java
    /// `getQueryAuth(String authorizationCode)`）。
    ///
    /// 成功后回写授权方 access_token/refresh_token 到配置存储
    /// （`updateAuthorizerAccessToken`/`updateAuthorizerRefreshToken`）。
    async fn get_query_auth(
        &self,
        _authorization_code: &str,
    ) -> Result<WxOpenQueryAuthResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_query_auth 未实现（Wave 2）",
        ))
    }

    /// 获取授权方的帐号基本信息（对应 Java
    /// `getAuthorizerInfo(String authorizerAppid)`）。
    async fn get_authorizer_info(
        &self,
        _authorizer_appid: &str,
    ) -> Result<WxOpenAuthorizerInfoResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_authorizer_info 未实现（Wave 2）",
        ))
    }

    /// 获取所有授权方列表（对应 Java
    /// `getAuthorizerList(int begin, int len)`）。
    ///
    /// 成功后将列表中的 authorizer_appid/refresh_token 回写配置存储。
    async fn get_authorizer_list(
        &self,
        _begin: i32,
        _len: i32,
    ) -> Result<WxOpenAuthorizerListResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_authorizer_list 未实现（Wave 2）",
        ))
    }

    /// 获取授权方的选项设置信息（对应 Java
    /// `getAuthorizerOption(String authorizerAppid, String optionName)`，
    /// 以授权方 access_token 为 key 调用）。
    async fn get_authorizer_option(
        &self,
        _authorizer_appid: &str,
        _option_name: &str,
    ) -> Result<WxOpenAuthorizerOptionResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_authorizer_option 未实现（Wave 2）",
        ))
    }

    /// 设置授权方的选项信息（对应 Java
    /// `setAuthorizerOption(String authorizerAppid, String optionName,
    /// String optionValue)`）。
    async fn set_authorizer_option(
        &self,
        _authorizer_appid: &str,
        _option_name: &str,
        _option_value: &str,
    ) -> Result<(), WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "set_authorizer_option 未实现（Wave 2）",
        ))
    }

    /// 校验消息签名（对应 Java `checkSignature(String appid, String timestamp,
    /// String nonce, String signature)`）。
    ///
    /// Java 实现恒返回 false（appid 维度签名未实现），原样镜像。
    fn check_signature_with_appid(
        &self,
        _app_id: &str,
        _timestamp: &str,
        _nonce: &str,
        _signature: &str,
    ) -> bool {
        false
    }

    // ---- oauth2 与小程序登录（对应 Java 同名方法） ----

    /// 用 code 换取 oauth2 的 access token（对应 Java `oauth2getAccessToken
    /// (String appid, String code)`）。
    ///
    /// Java `@Deprecated`（2021-05-21：请使用
    /// `getWxMpServiceByAppid(mpAppId).getOAuth2Service().getAccessToken(code)`）。
    async fn oauth2_get_access_token(
        &self,
        _app_id: &str,
        _code: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "oauth2_get_access_token 未实现（Wave 2）",
        ))
    }

    /// 刷新 oauth2 的 access token（对应 Java
    /// `oauth2refreshAccessToken(String appid, String refreshToken)`）。
    async fn oauth2_refresh_access_token(
        &self,
        _app_id: &str,
        _refresh_token: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "oauth2_refresh_access_token 未实现（Wave 2）",
        ))
    }

    /// 构建 oauth2 授权链接（对应 Java `oauth2buildAuthorizationUrl(String
    /// appid, String redirectUri, String scope, String state)`）。
    ///
    /// Java `@Deprecated`（2021-05-21，见 [`Self::oauth2_get_access_token`]）。
    /// 纯字符串构建（redirect_uri 经 encodeURIComponent 语义编码），不抛错。
    fn oauth2_build_authorization_url(
        &self,
        app_id: &str,
        redirect_uri: &str,
        scope: &str,
        state: &str,
    ) -> String {
        use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
        let component_app_id = self
            .wx_open_config_storage()
            .and_then(|c| c.component_app_id())
            .unwrap_or_default();
        let encoded_redirect = utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string();
        format!(
            "https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope={}&state={}&component_appid={}#wechat_redirect",
            app_id,
            encoded_redirect,
            scope,
            state.trim(),
            component_app_id
        )
    }

    /// 小程序登录 code 换 session（对应 Java
    /// `miniappJscode2Session(String appId, String jsCode)`）。
    ///
    /// ADAPTED：Java 返回 `WxMaJscode2SessionResult`（wx-rust-miniapp 的
    /// bean，open 模块暂不依赖），Rust 返回解析后的
    /// `serde_json::Value`，接线后换型。
    async fn miniapp_jscode2_session(
        &self,
        _app_id: &str,
        _js_code: &str,
    ) -> Result<serde_json::Value, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "miniapp_jscode2_session 未实现（Wave 2）",
        ))
    }

    // ---- 小程序代码模板（对应 Java 同名方法，access_token 注入键为
    // "access_token"（component_access_token），非默认键） ----

    /// 获取草稿箱内的所有临时代码草稿（对应 Java `getTemplateDraftList()`）。
    ///
    /// Java 无 `draft_list` 字段时返回 null → `Ok(None)` 镜像。
    async fn get_template_draft_list(
        &self,
    ) -> Result<Option<Vec<WxOpenMaCodeTemplate>>, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_template_draft_list 未实现（Wave 2）",
        ))
    }

    /// 获取代码模版库中的所有小程序代码模版（对应 Java `getTemplateList()`，
    /// Java `@Deprecated`，请使用 [`Self::get_template_list_with_type`]）。
    async fn get_template_list(
        &self,
    ) -> Result<Option<Vec<WxOpenMaCodeTemplate>>, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_template_list 未实现（Wave 2）",
        ))
    }

    /// 获取代码模版库中的所有小程序代码模版（对应 Java
    /// `getTemplateList(Integer templateType)`；`template_type` 可空，
    /// 默认全部，0 普通模板，1 标准模板）。
    async fn get_template_list_with_type(
        &self,
        _template_type: Option<i32>,
    ) -> Result<Option<Vec<WxOpenMaCodeTemplate>>, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_template_list_with_type 未实现（Wave 2）",
        ))
    }

    /// 将草稿箱的草稿选为小程序代码模版（对应 Java `addToTemplate(long
    /// draftId)`，Java `@Deprecated`，请使用
    /// [`Self::add_to_template_with_type`]）。
    async fn add_to_template(&self, _draft_id: i64) -> Result<(), WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "add_to_template 未实现（Wave 2）",
        ))
    }

    /// 将草稿添加到代码模板库（对应 Java `addToTemplate(long draftId,
    /// int templateType)`；`template_type`：普通模板 0，标准模板 1）。
    async fn add_to_template_with_type(
        &self,
        _draft_id: i64,
        _template_type: i32,
    ) -> Result<(), WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "add_to_template_with_type 未实现（Wave 2）",
        ))
    }

    /// 删除指定小程序代码模版（对应 Java `deleteTemplate(long templateId)`）。
    async fn delete_template(&self, _template_id: i64) -> Result<(), WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "delete_template 未实现（Wave 2）",
        ))
    }

    // ---- open 帐号管理（对应 Java 同名方法） ----

    /// 创建开放平台帐号并绑定公众号/小程序（对应 Java
    /// `createOpenAccount(String appId, String appIdType)`；
    /// `app_id_type`：mp-公众号 / mini-小程序）。
    ///
    /// 待接线：Java 经 `openAccountServicePost` 走代 mp/ma 子服务的 post
    /// （`getWxMpServiceByAppid`/`getWxMaServiceByAppid`），Rust 侧子服务
    /// 桥接尚未接线（恒 None）→ 返回未实现错误。
    async fn create_open_account(
        &self,
        _app_id: &str,
        _app_id_type: &str,
    ) -> Result<WxOpenCreateResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "create_open_account 未实现（Wave 2，待代 mp/ma 接线）",
        ))
    }

    /// 将公众号/小程序绑定到开放平台帐号下（对应 Java
    /// `bindOpenAccount(String appId, String appIdType, String openAppid)`）。
    ///
    /// 接线说明同 [`Self::create_open_account`]。
    async fn bind_open_account(
        &self,
        _app_id: &str,
        _app_id_type: &str,
        _open_appid: &str,
    ) -> Result<bool, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "bind_open_account 未实现（Wave 2，待代 mp/ma 接线）",
        ))
    }

    /// 将公众号/小程序从开放平台帐号下解绑（对应 Java
    /// `unbindOpenAccount(String appId, String appIdType, String openAppid)`）。
    ///
    /// 接线说明同 [`Self::create_open_account`]。
    async fn unbind_open_account(
        &self,
        _app_id: &str,
        _app_id_type: &str,
        _open_appid: &str,
    ) -> Result<bool, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "unbind_open_account 未实现（Wave 2，待代 mp/ma 接线）",
        ))
    }

    /// 获取公众号/小程序所绑定的开放平台帐号（对应 Java
    /// `getOpenAccount(String appId, String appIdType)`）。
    ///
    /// 接线说明同 [`Self::create_open_account`]。
    async fn get_open_account(
        &self,
        _app_id: &str,
        _app_id_type: &str,
    ) -> Result<WxOpenGetResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_open_account 未实现（Wave 2，待代 mp/ma 接线）",
        ))
    }

    /// 查询公众号/小程序是否绑定 open 帐号（对应 Java `haveOpen()`，
    /// 走 component_access_token，注入键 "access_token"）。
    async fn have_open(&self) -> Result<WxOpenHaveResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "have_open 未实现（Wave 2）",
        ))
    }

    // ---- 快速创建小程序（对应 Java 同名方法） ----

    /// 第三方平台快速创建小程序（对应 Java `fastRegisterWeapp(String name,
    /// String code, String codeType, String legalPersonaWechat, String
    /// legalPersonaName, String componentPhone)`）。
    async fn fast_register_weapp(
        &self,
        _name: &str,
        _code: &str,
        _code_type: &str,
        _legal_persona_wechat: &str,
        _legal_persona_name: &str,
        _component_phone: &str,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "fast_register_weapp 未实现（Wave 2）",
        ))
    }

    /// 查询第三方平台快速创建小程序的任务状态（对应 Java
    /// `fastRegisterWeappSearch(String name, String legalPersonaWechat,
    /// String legalPersonaName)`）。
    async fn fast_register_weapp_search(
        &self,
        _name: &str,
        _legal_persona_wechat: &str,
        _legal_persona_name: &str,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "fast_register_weapp_search 未实现（Wave 2）",
        ))
    }

    /// 快速创建个人小程序（对应 Java `fastRegisterPersonalWeapp(String
    /// idname, String wxuser, String componentPhone)`）。
    async fn fast_register_personal_weapp(
        &self,
        _idname: &str,
        _wxuser: &str,
        _component_phone: &str,
    ) -> Result<WxOpenRegisterPersonalWeappResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "fast_register_personal_weapp 未实现（Wave 2）",
        ))
    }

    /// 查询个人小程序注册任务状态（对应 Java
    /// `fastRegisterPersonalWeappSearch(String taskid)`）。
    async fn fast_register_personal_weapp_search(
        &self,
        _taskid: &str,
    ) -> Result<WxOpenRegisterPersonalWeappResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "fast_register_personal_weapp_search 未实现（Wave 2）",
        ))
    }

    /// 注册试用小程序（对应 Java `fastRegisterBetaWeapp(String name,
    /// String openid)`；注入键 "access_token"）。
    async fn fast_register_beta_weapp(
        &self,
        _name: &str,
        _openid: &str,
    ) -> Result<WxOpenRegisterBetaWeappResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "fast_register_beta_weapp 未实现（Wave 2）",
        ))
    }

    // ---- minishop 小商店（对应 Java 同名方法） ----

    /// 注册小商店账号（对应 Java `registerShop(String wxName, String
    /// idCardName, String idCardNumber, String channelId, Integer
    /// apiOpenstoreType, String authPageUrl)`）。
    async fn register_shop(
        &self,
        _wx_name: &str,
        _id_card_name: &str,
        _id_card_number: &str,
        _channel_id: Option<&str>,
        _api_openstore_type: Option<i32>,
        _auth_page_url: Option<&str>,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "register_shop 未实现（Wave 2）",
        ))
    }

    /// 异步状态查询（对应 Java `checkAuditStatus(String wxName)`，
    /// component_access_token 查询小商店注册状态）。
    async fn check_audit_status(&self, _wx_name: &str) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "check_audit_status 未实现（Wave 2）",
        ))
    }

    /// 已获取小商店 appId 后以授权方 access_token 查询状态（对应 Java
    /// `checkAuditStatus(String appId, String wxName)`）。
    async fn check_audit_status_with_appid(
        &self,
        _app_id: &str,
        _wx_name: &str,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "check_audit_status_with_appid 未实现（Wave 2）",
        ))
    }

    /// 提交小商店商户信息（对应 Java `submitMerchantInfo(String appId,
    /// String subjectType, MinishopBusiLicense busiLicense,
    /// MinishopOrganizationCodeInfo organizationCodeInfo,
    /// MinishopIdcardInfo idcardInfo, MinishopSuperAdministratorInfo
    /// superAdministratorInfo, String merchantShoprtName)`，授权方
    /// access_token）。
    async fn submit_merchant_info(
        &self,
        _app_id: &str,
        _subject_type: &str,
        _busi_license: &MinishopBusiLicense,
        _organization_code_info: Option<&MinishopOrganizationCodeInfo>,
        _idcard_info: Option<&MinishopIdcardInfo>,
        _super_administrator_info: Option<&MinishopSuperAdministratorInfo>,
        _merchant_shortname: Option<&str>,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "submit_merchant_info 未实现（Wave 2）",
        ))
    }

    /// 提交小商店基础信息（对应 Java `submitBasicInfo(String appId,
    /// MinishopNameInfo nameInfo, MinishopReturnInfo returnInfo)`，
    /// 授权方 access_token）。
    async fn submit_basic_info(
        &self,
        _app_id: &str,
        _name_info: &MinishopNameInfo,
        _return_info: &MinishopReturnInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "submit_basic_info 未实现（Wave 2）",
        ))
    }

    /// 上传小商店图片素材（对应 Java `uploadMinishopImagePicFile(String
    /// appId, Integer height, Integer width, File file)`）。
    ///
    /// ADAPTED：Java `File` 入参 → Rust 文件路径 `&str`；实际 multipart
    /// 上传经门面 [`crate::api::WxOpenService::upload_minishop_media_file`]
    /// （MinishopUploadRequestExecutor）。
    async fn upload_minishop_image_pic_file(
        &self,
        _app_id: &str,
        _height: i32,
        _width: i32,
        _file_path: &str,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "upload_minishop_image_pic_file 未实现（Wave 2）",
        ))
    }

    /// 获取小商店的类目详情（对应 Java `getMinishopCategories(String appId,
    /// Integer fCatId)`，`f_cat_id` 可先填 0 获取根部类目）。
    async fn get_minishop_categories(
        &self,
        _app_id: &str,
        _f_cat_id: i32,
    ) -> Result<MinishopCategories, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_minishop_categories 未实现（Wave 2）",
        ))
    }

    /// 获取小商店品牌信息（对应 Java `getMinishopBrands(String appId)`）。
    async fn get_minishop_brands(
        &self,
        _app_id: &str,
    ) -> Result<MinishopBrandList, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_minishop_brands 未实现（Wave 2）",
        ))
    }

    /// 获取小商店运费模版信息（对应 Java
    /// `getMinishopDeliveryTemplate(String appId)`）。
    async fn get_minishop_delivery_template(
        &self,
        _app_id: &str,
    ) -> Result<MinishopDeliveryTemplateResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_minishop_delivery_template 未实现（Wave 2）",
        ))
    }

    /// 获取小商店商品分类信息（对应 Java `getMinishopCatList(String appId)`）。
    async fn get_minishop_cat_list(
        &self,
        _app_id: &str,
    ) -> Result<MinishopShopCatList, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_minishop_cat_list 未实现（Wave 2）",
        ))
    }

    /// 获取小商店的快递公司列表（对应 Java
    /// `getMinishopDeliveryCompany(String appId)`，返回
    /// `WxMinishopAddGoodsSpuResult<List<WxMinishopDeliveryCompany>>`；
    /// Rust 侧 `WxMinishopAddGoodsSpuResult.data` 为 `serde_json::Value`
    /// 承载 company_list 数组，ADAPTED）。
    async fn get_minishop_delivery_company(
        &self,
        _app_id: &str,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_minishop_delivery_company 未实现（Wave 2）",
        ))
    }

    /// 创建小商店优惠券（对应 Java `minishopCreateCoupon(String appId,
    /// WxMinishopCoupon couponInfo)`，返回 couponId）。
    async fn minishop_create_coupon(
        &self,
        _app_id: &str,
        _coupon_info: &WxMinishopCoupon,
    ) -> Result<i32, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_create_coupon 未实现（Wave 2）",
        ))
    }

    /// 获取小商店的优惠券信息（对应 Java `minishopGetCouponList(String
    /// appId, String startCreateTime, String endCreateTime, Integer status,
    /// Integer page, Integer pageSize)`）。
    ///
    /// Java 实现恒 `return null` → `Ok(None)` 镜像。
    async fn minishop_get_coupon_list(
        &self,
        _app_id: &str,
        _start_create_time: &str,
        _end_create_time: &str,
        _status: i32,
        _page: i32,
        _page_size: i32,
    ) -> Result<Option<WxMinishopCouponStock>, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_get_coupon_list 未实现（Wave 2）",
        ))
    }

    /// 将优惠券发送给某人（对应 Java `minishopPushCouponToUser(String appid,
    /// String openId, Integer couponId)`）。
    async fn minishop_push_coupon_to_user(
        &self,
        _app_id: &str,
        _open_id: &str,
        _coupon_id: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_push_coupon_to_user 未实现（Wave 2）",
        ))
    }

    /// 更新商城优惠券（对应 Java `minishopUpdateCoupon(String appId,
    /// WxMinishopCoupon couponInfo)`，返回 couponId）。
    async fn minishop_update_coupon(
        &self,
        _app_id: &str,
        _coupon_info: &WxMinishopCoupon,
    ) -> Result<i32, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_update_coupon 未实现（Wave 2）",
        ))
    }

    /// 更新优惠券状态（对应 Java `minishopUpdateCouponStatus(String appId,
    /// Integer couponId, Integer status)`；1 创建 2 生效 4 作废 5 删除）。
    async fn minishop_update_coupon_status(
        &self,
        _app_id: &str,
        _coupon_id: i32,
        _status: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_update_coupon_status 未实现（Wave 2）",
        ))
    }

    /// 小商店添加商品（对应 Java `minishopGoodsAddSpu(String appId,
    /// WxMinishopSpu spu)`，添加后需上架并过审才展示）。
    async fn minishop_goods_add_spu(
        &self,
        _app_id: &str,
        _spu: &WxMinishopSpu,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_add_spu 未实现（Wave 2）",
        ))
    }

    /// 小商店删除商品（对应 Java `minishopGoodsDelSpu(String appId, Long
    /// productId, Long outProductId)`，直接删除不进回收站）。
    async fn minishop_goods_del_spu(
        &self,
        _app_id: &str,
        _product_id: i64,
        _out_product_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_del_spu 未实现（Wave 2）",
        ))
    }

    /// 小商店更新商品（对应 Java `minishopGoodsUpdateSpu(String appId,
    /// WxMinishopSpu spu)`，更新入草稿箱，需上架过审）。
    async fn minishop_goods_update_spu(
        &self,
        _app_id: &str,
        _spu: &WxMinishopSpu,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_update_spu 未实现（Wave 2）",
        ))
    }

    /// 上架商品（对应 Java `minishopGoodsListingSpu(String appId, Long
    /// productId, Long outProductId)`）。
    async fn minishop_goods_listing_spu(
        &self,
        _app_id: &str,
        _product_id: i64,
        _out_product_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_listing_spu 未实现（Wave 2）",
        ))
    }

    /// 下架商品（对应 Java `minishopGoodsDelistingSpu(String appId, Long
    /// productId, Long outProductId)`）。
    async fn minishop_goods_delisting_spu(
        &self,
        _app_id: &str,
        _product_id: i64,
        _out_product_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_delisting_spu 未实现（Wave 2）",
        ))
    }

    /// 小商店新增 sku 信息（对应 Java `minishiopGoodsAddSku(String appId,
    /// WxMinishopSku sku)`，Java 方法名拼写为 minishop + Goods 少 i）。
    async fn minishop_goods_add_sku(
        &self,
        _app_id: &str,
        _sku: &WxMinishopSku,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_add_sku 未实现（Wave 2）",
        ))
    }

    /// 小商店批量新增 sku 信息（对应 Java `minishopGoodsBatchAddSku(String
    /// appId, List<WxMinishopSku> skuList)`）。
    async fn minishop_goods_batch_add_sku(
        &self,
        _app_id: &str,
        _sku_list: &[WxMinishopSku],
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_batch_add_sku 未实现（Wave 2）",
        ))
    }

    /// 小商店删除 sku 信息（对应 Java `minishopGoodsDelSku(String appId, Long
    /// productId, Long outProductId, String outSkuId, Long skuId)`）。
    async fn minishop_goods_del_sku(
        &self,
        _app_id: &str,
        _product_id: i64,
        _out_product_id: i64,
        _out_sku_id: &str,
        _sku_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_del_sku 未实现（Wave 2）",
        ))
    }

    /// 小商店更新 sku（对应 Java `minishopGoodsUpdateSku(String appId,
    /// WxMinishopSku sku)`）。
    async fn minishop_goods_update_sku(
        &self,
        _app_id: &str,
        _sku: &WxMinishopSku,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_update_sku 未实现（Wave 2）",
        ))
    }

    /// 小商店更新 sku 价格（对应 Java `minishopGoodsUpdateSkuPrice(String
    /// appId, Long productId, Long outProductId, String outSkuId, Long skuId,
    /// Long salePrice, Long marketPrice)`）。
    ///
    /// 注意：Java 实现将 `sale_price`/`market_price` 均写成 `outSkuId`
    /// （WxJava 上游 bug），严格镜像（见 impl 注释）。
    async fn minishop_goods_update_sku_price(
        &self,
        _app_id: &str,
        _product_id: i64,
        _out_product_id: i64,
        _out_sku_id: &str,
        _sku_id: i64,
        _sale_price: i64,
        _market_price: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_update_sku_price 未实现（Wave 2）",
        ))
    }

    /// 小商店更新 sku 库存（对应 Java `minishopGoodsUpdateSkuStock(String
    /// appId, Long productId, Long outProductId, String outSkuId, Long skuId,
    /// Integer type, Integer stockNum)`）。
    async fn minishop_goods_update_sku_stock(
        &self,
        _app_id: &str,
        _product_id: i64,
        _out_product_id: i64,
        _out_sku_id: &str,
        _sku_id: i64,
        r#_type: i32,
        _stock_num: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_goods_update_sku_stock 未实现（Wave 2）",
        ))
    }

    /// 小商店通用 Post 接口（对应 Java `minishopCommonPost(String appId,
    /// String url, String requestParam)`）。
    ///
    /// Java 实现恒 `return null` → `Ok(None)` 镜像。
    async fn minishop_common_post(
        &self,
        _app_id: &str,
        _url: &str,
        _request_param: &str,
    ) -> Result<Option<String>, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "minishop_common_post 未实现（Wave 2）",
        ))
    }

    /// 添加抢购任务（对应 Java `addLimitDiscountGoods(String appId,
    /// LimitDiscountGoods limitDiscountGoods)`，返回 taskId）。
    async fn add_limit_discount_goods(
        &self,
        _app_id: &str,
        _limit_discount_goods: &LimitDiscountGoods,
    ) -> Result<i32, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "add_limit_discount_goods 未实现（Wave 2）",
        ))
    }

    /// 获取抢购任务列表（对应 Java `getLimitDiscountList(String appId,
    /// Integer status)`；status 0 未结束 1 已结束，不填则都拉取）。
    async fn get_limit_discount_list(
        &self,
        _app_id: &str,
        _status: Option<i32>,
    ) -> Result<Vec<LimitDiscountGoods>, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_limit_discount_list 未实现（Wave 2）",
        ))
    }

    /// 修改抢购任务状态（对应 Java `updateLimitDiscountStatus(String appId,
    /// Long taskId, Integer status)`，结束后不可再开启）。
    async fn update_limit_discount_status(
        &self,
        _app_id: &str,
        _task_id: i64,
        _status: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "update_limit_discount_status 未实现（Wave 2）",
        ))
    }

    // ---- tcb 云开发（对应 Java 同名方法） ----

    /// 查询环境共享信息（对应 Java `getShareCloudBaseEnv(List<String>
    /// appids)`）。
    async fn get_share_cloud_base_env(
        &self,
        _appids: &[String],
    ) -> Result<GetShareCloudBaseEnvResponse, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_share_cloud_base_env 未实现（Wave 2）",
        ))
    }

    /// 获取环境信息（对应 Java `getTcbEnvList()`）。
    async fn get_tcb_env_list(&self) -> Result<GetTcbEnvListResponse, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_tcb_env_list 未实现（Wave 2）",
        ))
    }

    /// 转换云环境（对应 Java `changeTcbEnv(String env)`）。
    async fn change_tcb_env(&self, _env: &str) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "change_tcb_env 未实现（Wave 2）",
        ))
    }

    /// 环境共享（对应 Java `shareCloudBaseEnv(ShareCloudBaseEnvRequest
    /// request)`）。
    async fn share_cloud_base_env(
        &self,
        _request: &ShareCloudBaseEnvRequest,
    ) -> Result<ShareCloudBaseEnvResponse, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "share_cloud_base_env 未实现（Wave 2）",
        ))
    }

    /// 使用 AppSecret 重置第三方平台 API 调用次数（对应 Java
    /// `clearQuotaV2(String appid)`，裸 post 不经 token 注入）。
    async fn clear_quota_v2(&self, _appid: &str) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "clear_quota_v2 未实现（Wave 2）",
        ))
    }

    // ---- 订单页 path 与服务器域名（对应 Java 同名方法） ----

    /// 申请设置订单页 path 信息（对应 Java `applySetOrderPathInfo(WxOpenMa
    /// ApplyOrderPathInfo info)`，一次提交不超过 100 个 appid）。
    async fn apply_set_order_path_info(
        &self,
        _info: &WxOpenMaApplyOrderPathInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "apply_set_order_path_info 未实现（Wave 2）",
        ))
    }

    /// 设置第三方平台服务器域名（对应 Java `modifyWxaServerDomain(String
    /// action, List<String> requestDomains, ...)`；action：add 添加 /
    /// delete 删除 / set 覆盖 / get 获取，get 时不需要域名参数）。
    async fn modify_wxa_server_domain(
        &self,
        _action: &str,
        _request_domains: &[String],
        _ws_request_domains: &[String],
        _upload_domains: &[String],
        _download_domains: &[String],
        _udp_domains: &[String],
        _tcp_domains: &[String],
    ) -> Result<WxOpenMaDomainResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "modify_wxa_server_domain 未实现（Wave 2）",
        ))
    }

    /// 获取第三方平台业务域名校验文件（对应 Java `getDomainConfirmFile()`）。
    async fn get_domain_confirm_file(
        &self,
    ) -> Result<WxOpenMaDomainConfirmFileResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "get_domain_confirm_file 未实现（Wave 2）",
        ))
    }

    /// 设置第三方平台业务域名（对应 Java `modifyWxaJumpDomain(String action,
    /// List<String> domainList)`，直接返回字符串）。
    async fn modify_wxa_jump_domain(
        &self,
        _action: &str,
        _domain_list: &[String],
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "modify_wxa_jump_domain 未实现（Wave 2）",
        ))
    }

    /// 设置第三方平台业务域名（对应 Java `modifyWxaJumpDomainInfo(String
    /// action, List<String> domainList)`，解析为 webview domain 信息）。
    async fn modify_wxa_jump_domain_info(
        &self,
        _action: &str,
        _domain_list: &[String],
    ) -> Result<WxOpenMaWebDomainResult, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "modify_wxa_jump_domain_info 未实现（Wave 2）",
        ))
    }
}
