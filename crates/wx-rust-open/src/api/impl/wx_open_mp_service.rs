//! 代公众号（mp）服务桥接。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMpServiceImpl`（extends
//! `WxMpServiceImpl`）：Java 以「继承完整 mp 服务 + 覆写两个方法」表达代
//! 运营语义——
//! - `getAccessToken(boolean forceRefresh)` → 委托
//!   `wxOpenComponentService.getAuthorizerAccessToken(appId, forceRefresh)`
//!   （三方 token 刷新链，不是普通公众号的 appid/secret 换 token）；
//! - `getWxMpConfigStorage()` → `WxOpenConfigStorage.getWxMpConfigStorage
//!   (appId)`（`WxOpenInnerConfigStorage` 内层桥接配置：access_token/
//!   jsapi/card ticket 均实时委托 open 存储的按 appId 分桶缓存）。
//!
//! Rust 侧 trait 默认实现 + 组合表达同一语义（与 mp 模块设计原则一致）：
//! `WxMpService` 的 `get`/`post` 默认实现经 mp 执行引擎
//! （`base_wx_mp_service_impl::execute_internal`）调用
//! [`Self::get_access_token_with_force`] 注入 authorizer access_token，
//! 因此仅需覆写三个方法（配置存储 / HTTP 客户端 / access_token），其余
//! 全部方法（短链、回调 IP、菜单子服务等）经默认实现自动走桥接语义。
//!
//! 配置桥接（[`OpenMpConfigBridge`]，对应 Java `WxOpenInnerConfigStorage`）：
//! - access_token / jsapi ticket / card ticket / 锁 → open 存储按 appId
//!   分桶缓存（Java `getAuthorizerAccessToken(appId)` 等）；
//! - `token()`/`aes_key()` → open 的 component 凭证（Java
//!   `getComponentToken()`/`getComponentAesKey()`）；构造时快照
//!   （Java 实时委托，Rust 借用规则下 component 凭证静态不变，ADAPTED）；
//! - `secret()`/`template_id()`/`oauth2_redirect_url()`/
//!   `qr_connect_redirect_url()` → Java 恒 null → Rust `""`/`None` 镜像
//!   （代运营场景无独立 appsecret，ADAPTED）；
//! - `host_config()` → open 的 host 配置映射（Java 该字段未初始化恒
//!   null，Rust 无 null，以 open 域名覆盖映射，保证桥接请求与 open 引擎
//!   同域名，ADAPTED）。
//!
//! 注意：`get_wx_mp_service_by_appid` 返回值以 `Arc<dyn Any + Send + Sync>`
//! 承载（组件 trait 签名冻结），调用方经 [`downcast_mp_service`] 取下转后
//! 的 `Arc<dyn WxMpService>`（std `Any::downcast` 要求 Sized，无法直接
//! downcast 到 trait 对象，ADAPTED）。

use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use async_trait::async_trait;

use wx_rust_common::config::TicketType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::service::WxOAuth2Service;
use wx_rust_common::util::http::{SimpleGetRequestExecutor, SimplePostRequestExecutor};
use wx_rust_mp::api::WxMpService;
use wx_rust_mp::api::r#impl::WxMpOAuth2ServiceImpl;
use wx_rust_mp::config::WxMpConfigStorage;
use wx_rust_mp::config::WxMpHostConfig;

use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMpOAuth2ServiceImpl;
use crate::config::WxOpenConfigStorage;

/// 将组件服务的代 mp 服务返回值下转为 `Arc<dyn WxMpService>`。
///
/// 对应 Java 调用方直接持有的 `WxOpenMpService` 强类型引用；Rust 组件
/// trait 冻结为 `Arc<dyn Any + Send + Sync>`，此处按具体类型
/// `WxOpenMpService` 下转后上转为 mp 门面（std downcast 无法直接到
/// trait 对象，ADAPTED）。类型不匹配（传入代 ma 服务等）时返回 `None`。
pub fn downcast_mp_service(any: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn WxMpService>> {
    any.downcast::<WxOpenMpService>()
        .ok()
        .map(|svc| svc as Arc<dyn WxMpService>)
}

/// 代公众号（mp）服务桥接实现（对应 Java `WxOpenMpServiceImpl`）。
pub struct WxOpenMpService {
    /// 门面服务弱引用（对应 Java 强持有 `WxOpenComponentService`；Rust 以
    /// 弱引用打破「open service → component service → 桥接服务」环）。
    wx_open_service: Weak<dyn WxOpenService>,
    /// 授权方 appid（Java 构造入参，代运营目标账号）。
    app_id: String,
    /// 内层桥接配置（对应 Java `WxOpenInnerConfigStorage`，open 存储按
    /// appId 分桶缓存的映射视图）。
    config: Arc<OpenMpConfigBridge>,
    /// HTTP 客户端（对应 Java 构造时 `initHttp()`；reqwest Client 为
    /// Arc 语义的浅克隆，与门面共享连接池）。
    http_client: reqwest::Client,
    /// 代公众号 oauth2 服务（对应 Java 构造器
    /// `setOAuth2Service(new WxOpenMpOAuth2ServiceImpl(...))` 装配；
    /// 经 [`Self::new_arc`] 填充，`new` 底层构造为 None）。
    oauth2_service: Mutex<Option<Arc<dyn WxOAuth2Service>>>,
}

impl WxOpenMpService {
    /// 构建代 mp 服务（对应 Java
    /// `new WxOpenMpServiceImpl(WxOpenComponentService, appId,
    /// WxMpConfigStorage)`）。
    ///
    /// 构造时快照门面的配置存储与 HTTP 客户端（Java 同一时刻的
    /// `getWxOpenConfigStorage()` + `initHttp()` 语义）；后续
    /// `setWxOpenConfigStorage` 不追溯重建已缓存实例（镜像 Java）。
    /// oauth2 服务需桥接实例 Arc 化后装配，经 [`Self::new_arc`] 完成
    /// （本构造为底层构造，oauth2 服务为 None，ADAPTED）。
    ///
    /// # 参数
    /// - `wx_open_service`：门面服务强引用（内部降级为弱引用）
    /// - `app_id`：授权方 appid
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        let config = wx_open_service.wx_open_config_storage();
        let http_client = wx_open_service.http_client().clone();
        let wx_open_service = Arc::downgrade(&wx_open_service);
        let bridge_config = Arc::new(OpenMpConfigBridge::new(config, &app_id));
        Self {
            wx_open_service,
            app_id,
            config: bridge_config,
            http_client,
            oauth2_service: Mutex::new(None),
        }
    }

    /// 构建代 mp 服务并装配代公众号 oauth2 服务（对应 Java
    /// `new WxOpenMpServiceImpl(...)` 构造器内的
    /// `setOAuth2Service(new WxOpenMpOAuth2ServiceImpl(wxOpenComponentService,
    /// getOAuth2Service(), wxMpConfigStorage))`）。
    ///
    /// 内层装饰器委托目标为默认公众号 oauth2 服务
    /// （[`WxMpOAuth2ServiceImpl`] 包本桥接实例，对应 Java
    /// `WxMpServiceImpl.getOAuth2Service()`）。
    pub fn new_arc(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Arc<Self> {
        let arc = Arc::new(Self::new(wx_open_service.clone(), app_id.clone()));
        // 内层默认 oauth2（Java `getOAuth2Service()`：WxMpOAuth2ServiceImpl
        // 包本 mp 服务；Rust 以 Weak 表达，无循环）
        let weak_self: Weak<dyn WxMpService> =
            Arc::downgrade(&(arc.clone() as Arc<dyn WxMpService>));
        let inner: Arc<dyn WxOAuth2Service> = Arc::new(WxMpOAuth2ServiceImpl::new(weak_self));
        let oauth2: Arc<dyn WxOAuth2Service> = Arc::new(WxOpenMpOAuth2ServiceImpl::new(
            wx_open_service,
            inner,
            app_id,
        ));
        *arc.oauth2_service.lock().unwrap() = Some(oauth2);
        arc
    }

    /// 授权方 appid。
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// 代公众号 oauth2 服务（对应 Java `WxMpService.getOAuth2Service()`；
    /// 经 [`Self::new_arc`] 装配后为 `Some`）。
    ///
    /// ADAPTED：wx-rust-mp 的 `WxMpService` trait 未冻结 oauth2 getter
    /// （Java `WxMpService` 接口有），本方法以桥接固有方法承载，
    /// 返回 `Option` 表达未装配（底层 `new` 构造）。
    pub fn oauth2_service(&self) -> Option<Arc<dyn WxOAuth2Service>> {
        self.oauth2_service.lock().unwrap().clone()
    }

    /// 升级门面服务引用；门面已释放时返回业务错误。
    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }
}

#[async_trait]
impl WxMpService for WxOpenMpService {
    fn wx_mp_config_storage(&self) -> Arc<dyn WxMpConfigStorage> {
        self.config.clone()
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// 获取（刷新）authorizer access_token（对应 Java
    /// `WxOpenMpServiceImpl.getAccessToken(boolean forceRefresh)`）。
    ///
    /// 委托组件服务 `getAuthorizerAccessToken(appId, forceRefresh)`：
    /// 三方刷新链（component_access_token + authorizer refresh_token 换
    /// 新 token），非普通公众号的 appid/secret 换取；`get`/`post` 默认
    /// 实现经 mp 执行引擎调用本方法完成 token 注入与 40001 自动刷新。
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

    /// GET 请求（对应 Java `WxMpServiceImpl.get(String, String)`）。
    ///
    /// 显式覆写仅为标注：默认实现的执行引擎
    /// （`base_wx_mp_service_impl::execute_with_retry`）经
    /// [`Self::get_access_token_with_force`] 注入 authorizer access_token，
    /// 与 Java 继承语义一致（Java 未覆写 get，继承自 WxMpServiceImpl）。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        wx_rust_mp::api::r#impl::base_wx_mp_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            query_param.to_string(),
        )
        .await
    }

    /// POST 请求（对应 Java `WxMpServiceImpl.post(String, String)`）。
    ///
    /// 覆写说明同 [`Self::get`]；open_account 系列（Java
    /// `openAccountServicePost` 经 `wxMpService.post`）由此注入
    /// authorizer access_token。
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        wx_rust_mp::api::r#impl::base_wx_mp_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            post_data.to_string(),
        )
        .await
    }
}

/// 内层桥接配置（对应 Java `WxOpenInnerConfigStorage implements
/// WxMpConfigStorage`）。
///
/// access_token / jsapi ticket / card ticket 及其锁实时委托 open 存储的
/// 按 appId 分桶缓存（与 Java `getAuthorizerAccessToken(appId)` 等逐一
/// 对应）；component 凭证与代理字段为构造快照（Java 实时委托，Rust
/// 借用规则下以 owned 快照表达，ADAPTED）。
struct OpenMpConfigBridge {
    open_config: Arc<dyn WxOpenConfigStorage>,
    app_id: String,
    /// 是否使用稳定版 access token 接口（Java 内层私有字段，本地存储）。
    use_stable_access_token: AtomicBool,
    /// component 凭证快照（Java `getToken()`/`getAesKey()` 实时委托 open
    /// 存储，Rust 以构造快照表达，ADAPTED）。
    component_token: Option<String>,
    component_aes_key: Option<String>,
    /// 代理快照（Java 实时委托 open 存储的代理 getter，ADAPTED）。
    http_proxy_host: Option<String>,
    http_proxy_port: Option<u16>,
    /// host 配置（Java 未初始化恒 null；Rust 以 open host 映射，ADAPTED）。
    host_config: Mutex<WxMpHostConfig>,
}

impl OpenMpConfigBridge {
    /// 构建内层桥接配置（对应 Java
    /// `new WxOpenInnerConfigStorage(WxOpenConfigStorage, appId)`，含
    /// 按 appId 分桶的三个锁：`appId:accessTokenLock` /
    /// `appId:jsapiTicketLock` / `appId:cardApiTicketLock`）。
    fn new(open_config: Arc<dyn WxOpenConfigStorage>, app_id: &str) -> Self {
        let mut host_config = WxMpHostConfig::new();
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

impl wx_rust_common::config::WxConfigStorage for OpenMpConfigBridge {
    fn app_id(&self) -> &str {
        &self.app_id
    }

    /// 代运营场景无独立 appsecret（Java 恒 null → Rust `""`，ADAPTED）；
    /// 桥接的 token 刷新走三方链，不经 appid/secret 换取。
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
        // Java `getLockByKey(appId + ":accessTokenLock")`（内层专用锁，
        // 与组件服务刷新链的按 appId 锁同一锁表）
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

impl WxMpConfigStorage for OpenMpConfigBridge {
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

    /// Java `getTemplateId()` 恒 null。
    fn template_id(&self) -> Option<&str> {
        None
    }

    /// Java `getOauth2RedirectUrl()` 恒 null。
    fn oauth2_redirect_url(&self) -> Option<&str> {
        None
    }

    /// Java `getQrConnectRedirectUrl()` 恒 null。
    fn qr_connect_redirect_url(&self) -> Option<&str> {
        None
    }

    fn retry_sleep_millis(&self) -> i32 {
        self.open_config.retry_sleep_millis()
    }

    fn max_retry_times(&self) -> i32 {
        self.open_config.max_retry_times()
    }

    fn host_config(&self) -> WxMpHostConfig {
        self.host_config.lock().unwrap().clone()
    }

    fn set_host_config(&self, host_config: WxMpHostConfig) {
        *self.host_config.lock().unwrap() = host_config;
    }
}
