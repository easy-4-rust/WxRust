//! 代小程序（ma）服务桥接。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaServiceImpl`（extends
//! `WxMaServiceImpl`）与 `WxOpenFastMaServiceImpl`（`@Deprecated`，
//! 2021-06-23 起以 `WxOpenMaService` 替代）：Java 以「继承完整 ma 服务 +
//! 覆写」表达代运营语义——
//! - `getAccessToken(boolean forceRefresh)` → 委托
//!   `wxOpenComponentService.getAuthorizerAccessToken(appId, forceRefresh)`
//!   （三方 token 刷新链）；
//! - `getWxMaConfig()` → `WxOpenConfigStorage.getWxMaConfig(appId)`
//!   （`WxOpenInnerConfigStorage` 内层桥接配置，与 mp 桥接同一结构）；
//! - `jsCode2SessionInfo(String jsCode)` → 委托组件服务
//!   `miniappJscode2Session(appId, jsCode)`（component_access_token 注入的
//!   `/sns/component/jscode2session`，普通小程序接口无 component_appid
//!   参数，必须走组件链路）。
//!
//! Rust 侧 trait 默认实现 + 组合表达同一语义（与 ma 模块设计原则一致）：
//! `WxMaService` 的 `get`/`post` 默认实现经 ma 执行引擎调用
//! [`Self::get_access_token_with_force`] 注入 authorizer access_token，
//! 因此仅需覆写核心方法，其余方法经默认实现自动走桥接语义。
//!
//! Wave 5 更新（Ma 子域服务装配，对应 Java `WxOpenMaServiceImpl` 构造
//! 器内 `new WxOpenMaBasicServiceImpl(this, wxOpenComponentService)` 等
//! 七个子服务的 `@Getter final` 字段）：本结构按 appid 装配
//! [`WxOpenMaBasicServiceImpl`]/[`WxOpenMaAuthServiceImpl`]/
//! [`WxOpenMaIcpServiceImpl`]/[`WxOpenMaPrivacyServiceImpl`]/
//! [`WxOpenMaShoppingOrdersServiceImpl`]/[`WxOpenMaEmbeddedServiceImpl`]/
//! [`WxOpenMaAuthAndIcpServiceImpl`]，getter 与 Java 同名（
//! `getBasicService`/`getAuthService`/`getIcpService`/`getPrivacyService`/
//! `getShoppingOrdersService`/`getEmbeddedService`/`getAuthAndIcpService`）。
//! 子服务以 `Weak<dyn WxOpenService>` + appid 持有门面，请求时经组件
//! `getWxMaServiceByAppid` 取回本桥接实例（避免「桥接 ↔ 子服务」构造
//! 环，ADAPTED）。
//!
//! 配置桥接（[`OpenMaConfigBridge`]，对应 Java `WxOpenInnerConfigStorage
//! implements WxMaConfig`）：access_token / jsapi ticket / card ticket /
//! 锁 → open 存储按 appId 分桶缓存；`token()`/`aes_key()` →
//! component 凭证（构造快照，ADAPTED）；`api_signature_*` →
//! open 的 component API 签名凭证（Java 实时委托）；`wechat_mp_appid()`
//! → component_appid（Java 委托）；`api_host_url()` → open host 配置的
//! api 域名（Java 复制外层 `apiHostUrl`，Rust 以 host 配置映射，
//! ADAPTED）；`access_token_url()` 无法委托（open 存储无对应字段，
//! ADAPTED，返回 None）；`original_id()`/`cloud_env()`/`msg_data_format()`
//! → Java 恒 null/未设置。
//!
//! 注意：`get_wx_ma_service_by_appid`/`get_wx_fast_ma_service_by_appid`
//! 返回值以 `Arc<dyn Any + Send + Sync>` 承载，调用方经
//! [`downcast_ma_service`] 取下转后的 `Arc<dyn WxMaService>`（Java
//! fast_ma 为独立类但语义相同，Rust 统一以本桥接服务承载，ADAPTED）。

use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use async_trait::async_trait;

use wx_rust_common::config::TicketType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::{SimpleGetRequestExecutor, SimplePostRequestExecutor};
use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::bean::WxMaJscode2SessionResult;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::WxMaHostConfig;

use crate::api::WxOpenService;
use crate::api::r#impl::{
    WxOpenMaAuthAndIcpServiceImpl, WxOpenMaAuthServiceImpl, WxOpenMaBasicServiceImpl,
    WxOpenMaEmbeddedServiceImpl, WxOpenMaIcpServiceImpl, WxOpenMaPrivacyServiceImpl,
    WxOpenMaShoppingOrdersServiceImpl,
};
use crate::config::WxOpenConfigStorage;

/// 将组件服务的代 ma 服务返回值下转为 `Arc<dyn WxMaService>`。
///
/// 对应 Java 调用方直接持有的 `WxOpenMaService` 强类型引用；Rust 组件
/// trait 冻结为 `Arc<dyn Any + Send + Sync>`，此处按具体类型
/// `WxOpenMaService` 下转后上转为 ma 门面（std downcast 无法直接到
/// trait 对象，ADAPTED）。类型不匹配时返回 `None`。
pub fn downcast_ma_service(any: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn WxMaService>> {
    any.downcast::<WxOpenMaService>()
        .ok()
        .map(|svc| svc as Arc<dyn WxMaService>)
}

/// 代小程序（ma）服务桥接实现（对应 Java `WxOpenMaServiceImpl` /
/// `WxOpenFastMaServiceImpl`）。
pub struct WxOpenMaService {
    /// 门面服务弱引用（对应 Java 强持有 `WxOpenComponentService`；Rust 以
    /// 弱引用打破「open service → component service → 桥接服务」环）。
    wx_open_service: Weak<dyn WxOpenService>,
    /// 授权方 appid（Java 构造入参，代运营目标账号）。
    app_id: String,
    /// 内层桥接配置（对应 Java `WxOpenInnerConfigStorage`）。
    config: Arc<OpenMaConfigBridge>,
    /// HTTP 客户端（对应 Java 构造时 `initHttp()`）。
    http_client: reqwest::Client,
    /// 小程序基础信息服务（对应 Java `@Getter final basicService`）。
    basic_service: Arc<WxOpenMaBasicServiceImpl>,
    /// 小程序认证（年审）服务（对应 Java `@Getter final authService`）。
    auth_service: Arc<WxOpenMaAuthServiceImpl>,
    /// 小程序备案服务（对应 Java `@Getter final icpService`）。
    icp_service: Arc<WxOpenMaIcpServiceImpl>,
    /// 小程序用户隐私保护指引服务（对应 Java `@Getter final
    /// privacyService`）。
    privacy_service: Arc<WxOpenMaPrivacyServiceImpl>,
    /// 购物订单服务（对应 Java `@Getter final shoppingOrdersService`）。
    shopping_orders_service: Arc<WxOpenMaShoppingOrdersServiceImpl>,
    /// 半屏小程序管理服务（对应 Java `@Getter final embeddedService`）。
    embedded_service: Arc<WxOpenMaEmbeddedServiceImpl>,
    /// 小程序认证及备案服务（对应 Java `@Getter final authAndIcpService`）。
    auth_and_icp_service: Arc<WxOpenMaAuthAndIcpServiceImpl>,
}

impl WxOpenMaService {
    /// 构建代 ma 服务（对应 Java
    /// `new WxOpenMaServiceImpl(WxOpenComponentService, appId,
    /// WxMaConfig)`；fast_ma 构造入参相同，语义等价）。
    ///
    /// 构造时快照门面的配置存储与 HTTP 客户端（镜像 Java），并装配七个
    /// Ma 子域服务（镜像 Java 构造器内 `new WxOpenMaXxxServiceImpl(this,
    /// ...)`）；后续 `setWxOpenConfigStorage` 不追溯重建已缓存实例
    /// （镜像 Java）。
    ///
    /// # 参数
    /// - `wx_open_service`：门面服务强引用（内部降级为弱引用）
    /// - `app_id`：授权方 appid
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        let config = wx_open_service.wx_open_config_storage();
        let http_client = wx_open_service.http_client().clone();
        // 子服务构造期间持有强引用（Arc::downgrade 后原入参仍存活至函数
        // 末尾，升级必然成功）；子服务内部降级为弱引用
        let strong = wx_open_service.clone();
        let wx_open_service = Arc::downgrade(&wx_open_service);
        let bridge_config = Arc::new(OpenMaConfigBridge::new(config, &app_id));
        // 子服务以门面弱引用 + appid 持有（Java 传 `this` 即本桥接实例；
        // Rust 请求时经组件 `getWxMaServiceByAppid` 取回，避免构造环，
        // ADAPTED）
        let basic_service = Arc::new(WxOpenMaBasicServiceImpl::new(
            strong.clone(),
            app_id.clone(),
        ));
        let auth_service = Arc::new(WxOpenMaAuthServiceImpl::new(strong.clone(), app_id.clone()));
        let icp_service = Arc::new(WxOpenMaIcpServiceImpl::new(strong.clone(), app_id.clone()));
        let privacy_service = Arc::new(WxOpenMaPrivacyServiceImpl::new(
            strong.clone(),
            app_id.clone(),
        ));
        let shopping_orders_service = Arc::new(WxOpenMaShoppingOrdersServiceImpl::new(
            strong.clone(),
            app_id.clone(),
        ));
        let embedded_service = Arc::new(WxOpenMaEmbeddedServiceImpl::new(
            strong.clone(),
            app_id.clone(),
        ));
        let auth_and_icp_service =
            Arc::new(WxOpenMaAuthAndIcpServiceImpl::new(strong, app_id.clone()));
        Self {
            wx_open_service,
            app_id,
            config: bridge_config,
            http_client,
            basic_service,
            auth_service,
            icp_service,
            privacy_service,
            shopping_orders_service,
            embedded_service,
            auth_and_icp_service,
        }
    }

    /// 授权方 appid。
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// 小程序基础信息服务（对应 Java `getBasicService()`）。
    pub fn get_basic_service(&self) -> Arc<WxOpenMaBasicServiceImpl> {
        self.basic_service.clone()
    }

    /// 小程序认证（年审）服务（对应 Java `getAuthService()`）。
    pub fn get_auth_service(&self) -> Arc<WxOpenMaAuthServiceImpl> {
        self.auth_service.clone()
    }

    /// 小程序备案服务（对应 Java `getIcpService()`）。
    pub fn get_icp_service(&self) -> Arc<WxOpenMaIcpServiceImpl> {
        self.icp_service.clone()
    }

    /// 小程序用户隐私保护指引服务（对应 Java `getPrivacyService()`）。
    pub fn get_privacy_service(&self) -> Arc<WxOpenMaPrivacyServiceImpl> {
        self.privacy_service.clone()
    }

    /// 购物订单服务（对应 Java `getShoppingOrdersService()`）。
    pub fn get_shopping_orders_service(&self) -> Arc<WxOpenMaShoppingOrdersServiceImpl> {
        self.shopping_orders_service.clone()
    }

    /// 半屏小程序管理服务（对应 Java `getEmbeddedService()`）。
    pub fn get_embedded_service(&self) -> Arc<WxOpenMaEmbeddedServiceImpl> {
        self.embedded_service.clone()
    }

    /// 小程序认证及备案服务（对应 Java `getAuthAndIcpService()`）。
    pub fn get_auth_and_icp_service(&self) -> Arc<WxOpenMaAuthAndIcpServiceImpl> {
        self.auth_and_icp_service.clone()
    }

    /// 升级门面服务引用；门面已释放时返回业务错误。
    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }
}

#[async_trait]
impl WxMaService for WxOpenMaService {
    fn wx_ma_config(&self) -> Arc<dyn WxMaConfig> {
        self.config.clone()
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// 获取（刷新）authorizer access_token（对应 Java
    /// `WxOpenMaServiceImpl.getAccessToken(boolean forceRefresh)`）。
    ///
    /// 委托组件服务 `getAuthorizerAccessToken(appId, forceRefresh)`（三方
    /// 刷新链）；`get`/`post` 默认实现经 ma 执行引擎调用本方法完成 token
    /// 注入与 40001 自动刷新。
    async fn get_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let component = svc.wx_open_component_service().ok_or_else(|| {
            WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )
        })?;
        component
            .get_authorizer_access_token(&self.app_id, force_refresh)
            .await
    }

    /// 小程序登录 code 换 session（对应 Java
    /// `WxOpenMaServiceImpl.jsCode2SessionInfo(String jsCode)`）。
    ///
    /// Java 显式覆写：委托组件服务 `miniappJscode2Session(appId, jsCode)`
    /// （component_access_token 注入 `/sns/component/jscode2session`），
    /// 而非普通小程序的 appid/secret 换取；Rust 经组件服务返回
    /// `serde_json::Value` 后解析为 [`WxMaJscode2SessionResult`]
    /// （组件服务返回型 ADAPTED，Java 直接返回 bean）。
    async fn js_code_to_session(
        &self,
        js_code: &str,
    ) -> Result<WxMaJscode2SessionResult, WxErrorException> {
        let svc = self.svc()?;
        let component = svc.wx_open_component_service().ok_or_else(|| {
            WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )
        })?;
        let value = component
            .miniapp_jscode2_session(&self.app_id, js_code)
            .await?;
        serde_json::from_value(value).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// GET 请求（对应 Java `WxMaServiceImpl.get(String, String)`）。
    ///
    /// 显式覆写仅为标注：默认实现的执行引擎
    /// （`base_wx_ma_service_impl::execute_with_retry`）经
    /// [`Self::get_access_token_with_force`] 注入 authorizer access_token，
    /// 与 Java 继承语义一致。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        wx_rust_miniapp::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            query_param.to_string(),
        )
        .await
    }

    /// POST 请求（对应 Java `WxMaServiceImpl.post(String, String)`）。
    ///
    /// 覆写说明同 [`Self::get`]；open_account 系列（Java
    /// `openAccountServicePost` 经 `wxOpenMaService.post`）由此注入
    /// authorizer access_token。
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        wx_rust_miniapp::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            post_data.to_string(),
        )
        .await
    }
}

/// 内层桥接配置（对应 Java `WxOpenInnerConfigStorage implements
/// WxMaConfig`）。
///
/// access_token / jsapi ticket / card ticket 及其锁实时委托 open 存储的
/// 按 appId 分桶缓存；component 凭证与代理字段为构造快照（ADAPTED）；
/// `access_token_url()` 无法委托（open 存储无对应字段，恒 None，ADAPTED）。
struct OpenMaConfigBridge {
    open_config: Arc<dyn WxOpenConfigStorage>,
    app_id: String,
    /// 是否使用稳定版 access token 接口（Java 内层私有字段，本地存储）。
    use_stable_access_token: AtomicBool,
    /// component 凭证快照（Java 实时委托 open 存储，ADAPTED）。
    component_token: Option<String>,
    component_aes_key: Option<String>,
    /// 代理快照（Java 实时委托 open 存储的代理 getter，ADAPTED）。
    http_proxy_host: Option<String>,
    http_proxy_port: Option<u16>,
    /// host 配置（Java 未初始化恒 null；Rust 以 open host 映射，ADAPTED）。
    host_config: Mutex<WxMaHostConfig>,
}

impl OpenMaConfigBridge {
    /// 构建内层桥接配置（对应 Java
    /// `new WxOpenInnerConfigStorage(WxOpenConfigStorage, appId)`）。
    fn new(open_config: Arc<dyn WxOpenConfigStorage>, app_id: &str) -> Self {
        let mut host_config = WxMaHostConfig::new();
        if let Some(h) = open_config.wx_open_host_config() {
            if !h.api_host.is_empty() {
                host_config.api_host = h.api_host;
            }
            if !h.mp_host.is_empty() {
                host_config.mp_host = h.mp_host;
            }
            if !h.open_host.is_empty() {
                host_config.open_host = h.open_host;
            }
        }
        Self {
            component_token: open_config.component_token(),
            component_aes_key: open_config.component_aes_key(),
            http_proxy_host: open_config.http_proxy_host(),
            http_proxy_port: match open_config.http_proxy_port() {
                p if p > 0 => Some(p as u16),
                _ => None,
            },
            host_config: Mutex::new(host_config),
            open_config,
            app_id: app_id.to_string(),
            use_stable_access_token: AtomicBool::new(false),
        }
    }
}

impl wx_rust_common::config::WxConfigStorage for OpenMaConfigBridge {
    fn app_id(&self) -> &str {
        &self.app_id
    }

    /// 代运营场景无独立 appsecret（Java 恒 null → Rust `""`，ADAPTED）。
    fn secret(&self) -> &str {
        ""
    }

    fn access_token(&self) -> Option<String> {
        self.open_config.authorizer_access_token(&self.app_id)
    }

    fn is_access_token_expired(&self) -> bool {
        self.open_config
            .is_authorizer_access_token_expired(&self.app_id)
    }

    fn expire_access_token(&self) {
        self.open_config
            .expire_authorizer_access_token(&self.app_id);
    }

    fn update_access_token(&self, access_token: &str, expires_in_seconds: i32) {
        self.open_config.update_authorizer_access_token_with_expiry(
            &self.app_id,
            access_token,
            expires_in_seconds,
        );
    }

    fn access_token_lock(&self) -> Arc<tokio::sync::Mutex<()>> {
        // Java `getLockByKey(appId + ":accessTokenLock")`
        self.open_config
            .lock_by_key(&format!("{}:accessTokenLock", self.app_id))
    }

    fn is_stable_access_token(&self) -> bool {
        self.use_stable_access_token.load(Ordering::Relaxed)
    }

    fn auto_refresh_token(&self) -> bool {
        self.open_config.auto_refresh_token()
    }

    fn ticket(&self, ticket_type: TicketType) -> Option<String> {
        match ticket_type {
            TicketType::Jsapi => self.open_config.jsapi_ticket(&self.app_id),
            TicketType::WxCard => self.open_config.card_api_ticket(&self.app_id),
            // Java switch 无 default 分支 → null
            TicketType::Sdk => None,
        }
    }

    fn is_ticket_expired(&self, ticket_type: TicketType) -> bool {
        match ticket_type {
            TicketType::Jsapi => self.open_config.is_jsapi_ticket_expired(&self.app_id),
            TicketType::WxCard => self.open_config.is_card_api_ticket_expired(&self.app_id),
            // Java switch 无 default 分支后 `return false`
            TicketType::Sdk => false,
        }
    }

    fn update_ticket(&self, ticket_type: TicketType, ticket: &str, expires_in_seconds: i32) {
        match ticket_type {
            TicketType::Jsapi => {
                self.open_config
                    .update_jsapi_ticket(&self.app_id, ticket, expires_in_seconds)
            }
            TicketType::WxCard => {
                self.open_config
                    .update_card_api_ticket(&self.app_id, ticket, expires_in_seconds)
            }
            TicketType::Sdk => {}
        }
    }

    fn ticket_lock(&self, ticket_type: TicketType) -> Arc<tokio::sync::Mutex<()>> {
        match ticket_type {
            TicketType::Jsapi => self
                .open_config
                .lock_by_key(&format!("{}:jsapiTicketLock", self.app_id)),
            TicketType::WxCard => self
                .open_config
                .lock_by_key(&format!("{}:cardApiTicketLock", self.app_id)),
            // Java 恒 null（无 default 分支）→ Rust 独立新锁（ADAPTED）
            TicketType::Sdk => Arc::new(tokio::sync::Mutex::new(())),
        }
    }

    fn expire_ticket(&self, ticket_type: TicketType) {
        match ticket_type {
            TicketType::Jsapi => self.open_config.expire_jsapi_ticket(&self.app_id),
            TicketType::WxCard => self.open_config.expire_card_api_ticket(&self.app_id),
            TicketType::Sdk => {}
        }
    }

    fn http_proxy_host(&self) -> Option<&str> {
        self.http_proxy_host.as_deref()
    }

    fn http_proxy_port(&self) -> Option<u16> {
        self.http_proxy_port
    }

    fn tmp_dir(&self) -> Option<&str> {
        // Java `getTmpDirFile()` 恒 null
        None
    }
}

impl WxMaConfig for OpenMaConfigBridge {
    fn use_stable_access_token(&self, use_stable_access_token: bool) {
        self.use_stable_access_token
            .store(use_stable_access_token, Ordering::Relaxed);
    }

    /// 消息校验 token（对应 Java `getToken()` → open
    /// `getComponentToken()`；构造快照，ADAPTED）。
    fn token(&self) -> Option<&str> {
        self.component_token.as_deref()
    }

    /// 消息加解密 aes key（对应 Java `getAesKey()` → open
    /// `getComponentAesKey()`；构造快照，ADAPTED）。
    fn aes_key(&self) -> Option<&str> {
        self.component_aes_key.as_deref()
    }

    /// Java 内层 `originalId` 字段默认 null。
    fn original_id(&self) -> Option<&str> {
        None
    }

    /// Java 内层 `cloudEnv` 字段默认 null。
    fn cloud_env(&self) -> Option<&str> {
        None
    }

    /// Java `getMsgDataFormat()` 恒 null。
    fn msg_data_format(&self) -> Option<&str> {
        None
    }

    fn retry_sleep_millis(&self) -> i32 {
        self.open_config.retry_sleep_millis()
    }

    fn max_retry_times(&self) -> i32 {
        self.open_config.max_retry_times()
    }

    fn host_config(&self) -> WxMaHostConfig {
        self.host_config.lock().unwrap().clone()
    }

    fn set_host_config(&self, host_config: WxMaHostConfig) {
        *self.host_config.lock().unwrap() = host_config;
    }

    /// 自定义 apiHost 地址（对应 Java 复制外层 `apiHostUrl`；Rust 以 open
    /// host 配置的 api 域名映射，ADAPTED）。
    fn api_host_url(&self) -> Option<String> {
        self.open_config.wx_open_host_config().map(|h| h.api_host)
    }

    /// open 存储无对应字段（Java 复制外层 `accessTokenUrl` 亦为 null 语义
    /// 占位），恒 None，ADAPTED。
    fn set_api_host_url(&self, _api_host_url: &str) {
        // Java 内层字段赋值；Rust 以 open host 映射为准（ADAPTED：仅保留
        // 语义位，不做本地覆盖）
    }

    fn access_token_url(&self) -> Option<String> {
        // Java 复制外层 accessTokenUrl（默认 null）→ None 镜像
        None
    }

    fn set_access_token_url(&self, _access_token_url: &str) {}

    /// 服务端 API 签名 RSA 私钥（对应 Java `getApiSignatureRsaPrivateKey()`
    /// → open `getComponentApiSignatureRsaPrivateKey()`，实时委托）。
    fn api_signature_rsa_private_key(&self) -> Option<String> {
        self.open_config.component_api_signature_rsa_private_key()
    }

    /// 服务端 API 签名 AES 密钥（对应 Java `getApiSignatureAesKey()` →
    /// open `getComponentApiSignatureAesKey()`）。
    fn api_signature_aes_key(&self) -> Option<String> {
        self.open_config.component_api_signature_aes_key()
    }

    /// API 签名 AES 密钥序号（对应 Java `getApiSignatureAesKeySn()` →
    /// open `getComponentApiSignatureAesKeySn()`）。
    fn api_signature_aes_key_sn(&self) -> Option<String> {
        self.open_config.component_api_signature_aes_key_sn()
    }

    /// API 签名 RSA 私钥序号（对应 Java `getApiSignatureRsaPrivateKeySn()`
    /// → open `getComponentApiSignatureRsaPrivateKeySn()`）。
    fn api_signature_rsa_private_key_sn(&self) -> Option<String> {
        self.open_config
            .component_api_signature_rsa_private_key_sn()
    }

    /// 签名用小程序 ID（对应 Java `getWechatMpAppid()` → open
    /// `getComponentAppId()`，实时委托）。
    fn wechat_mp_appid(&self) -> Option<String> {
        self.open_config.component_app_id()
    }
}
