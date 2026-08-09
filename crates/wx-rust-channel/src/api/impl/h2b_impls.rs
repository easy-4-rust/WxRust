//! H2b 组子服务实现注册（Wave 2，非 shop 域）。
//!
//! 本组实现 Java `me.chanjar.weixin.channel.api.impl` 中 14 个非 shop 域子服务：
//! WxChannelFundServiceImpl/WxStoreHomePageServiceImpl/WxStoreCooperationServiceImpl/
//! WxChannelCompassShopServiceImpl/WxLeagueWindowServiceImpl/WxLeagueSupplierServiceImpl/
//! WxLeaguePromoterServiceImpl/WxLeagueProductServiceImpl/WxLeadComponentServiceImpl/
//! WxFinderLiveServiceImpl/WxAssistantServiceImpl/WxChannelVipServiceImpl/
//! WxChannelCompassFinderServiceImpl/WxChannelLiveDashboardServiceImpl。
//!
//! 注册方式（与 miniapp `g4_impls.rs` 同一模式）：本文件为非 `mod.rs` 的分组
//! 注册文件，子模块以 `#[path]` 显式指回 `api/impl/` 根目录文件。
//!
//! 挂载（Wave 3 收尾）：由 `impl/mod.rs` 声明 `pub mod h2b_impls;` 挂载为
//! `crate::api::r#impl::h2b_impls`（原 `api/wx_channel_fund_service.rs` 的
//! TEMP-H2B 临时桥接已删除；各 impl 内嵌测试统一引用本路径）。

#[path = "wx_assistant_service_impl.rs"]
pub mod wx_assistant_service_impl;
#[path = "wx_channel_compass_finder_service_impl.rs"]
pub mod wx_channel_compass_finder_service_impl;
#[path = "wx_channel_compass_shop_service_impl.rs"]
pub mod wx_channel_compass_shop_service_impl;
#[path = "wx_channel_fund_service_impl.rs"]
pub mod wx_channel_fund_service_impl;
#[path = "wx_channel_live_dashboard_service_impl.rs"]
pub mod wx_channel_live_dashboard_service_impl;
#[path = "wx_channel_vip_service_impl.rs"]
pub mod wx_channel_vip_service_impl;
#[path = "wx_finder_live_service_impl.rs"]
pub mod wx_finder_live_service_impl;
#[path = "wx_lead_component_service_impl.rs"]
pub mod wx_lead_component_service_impl;
#[path = "wx_league_product_service_impl.rs"]
pub mod wx_league_product_service_impl;
#[path = "wx_league_promoter_service_impl.rs"]
pub mod wx_league_promoter_service_impl;
#[path = "wx_league_supplier_service_impl.rs"]
pub mod wx_league_supplier_service_impl;
#[path = "wx_league_window_service_impl.rs"]
pub mod wx_league_window_service_impl;
#[path = "wx_store_cooperation_service_impl.rs"]
pub mod wx_store_cooperation_service_impl;
#[path = "wx_store_home_page_service_impl.rs"]
pub mod wx_store_home_page_service_impl;

pub use wx_assistant_service_impl::WxAssistantServiceImpl;
pub use wx_channel_compass_finder_service_impl::WxChannelCompassFinderServiceImpl;
pub use wx_channel_compass_shop_service_impl::WxChannelCompassShopServiceImpl;
pub use wx_channel_fund_service_impl::WxChannelFundServiceImpl;
pub use wx_channel_live_dashboard_service_impl::WxChannelLiveDashboardServiceImpl;
pub use wx_channel_vip_service_impl::WxChannelVipServiceImpl;
pub use wx_finder_live_service_impl::WxFinderLiveServiceImpl;
pub use wx_lead_component_service_impl::WxLeadComponentServiceImpl;
pub use wx_league_product_service_impl::WxLeagueProductServiceImpl;
pub use wx_league_promoter_service_impl::WxLeaguePromoterServiceImpl;
pub use wx_league_supplier_service_impl::WxLeagueSupplierServiceImpl;
pub use wx_league_window_service_impl::WxLeagueWindowServiceImpl;
pub use wx_store_cooperation_service_impl::WxStoreCooperationServiceImpl;
pub use wx_store_home_page_service_impl::WxStoreHomePageServiceImpl;

/// 测试共享设施（仅测试构建编译；14 个 impl 的内嵌 `tests` 模块共用）。
///
/// - [`MockChannelConfig`]：最小内存配置（token 预置不过期，避免网络请求）；
/// - [`MockChannelService`]：`post` 覆盖为记录 (url, body) 并返回预设响应，
///   且模拟执行引擎的 errcode 校验（`errcode != 0` 抛 `WxErrorException`，
///   对应 Java `SimplePostRequestExecutor.handleResponse` 语义）；
/// - [`build_service`]：构造 (服务 Arc, 弱引用) 对，供各 impl `new(weak)` 注入。
#[cfg(test)]
pub mod test_support {
    use std::sync::{Arc, Mutex, RwLock, Weak};

    use async_trait::async_trait;
    use wx_rust_common::config::WxConfigStorage;
    use wx_rust_common::error::WxErrorException;

    use crate::api::WxChannelService;
    use crate::config::{WxChannelConfig, WxChannelHostConfig};

    /// 测试用最小配置存储（实现 `WxChannelConfig`；access_token 预置且永不过期）。
    #[derive(Debug)]
    pub struct MockChannelConfig {
        access_token: Mutex<Option<String>>,
        access_token_lock: Arc<tokio::sync::Mutex<()>>,
    }

    impl MockChannelConfig {
        /// 构建配置并预置 access_token。
        pub fn new() -> Self {
            Self {
                access_token: Mutex::new(Some("test_access_token".to_string())),
                access_token_lock: Arc::new(tokio::sync::Mutex::new(())),
            }
        }
    }

    impl Default for MockChannelConfig {
        fn default() -> Self {
            Self::new()
        }
    }

    impl WxConfigStorage for MockChannelConfig {
        fn app_id(&self) -> &str {
            "wx_test_appid"
        }

        fn secret(&self) -> &str {
            "test_secret"
        }

        fn access_token(&self) -> Option<String> {
            self.access_token.lock().unwrap().clone()
        }

        fn is_access_token_expired(&self) -> bool {
            false
        }

        fn expire_access_token(&self) {
            *self.access_token.lock().unwrap() = None;
        }

        fn update_access_token(&self, access_token: &str, _expires_in_seconds: i32) {
            *self.access_token.lock().unwrap() = Some(access_token.to_string());
        }

        fn access_token_lock(&self) -> Arc<tokio::sync::Mutex<()>> {
            self.access_token_lock.clone()
        }
    }

    impl WxChannelConfig for MockChannelConfig {
        fn token(&self) -> Option<&str> {
            Some("test_token")
        }

        fn aes_key(&self) -> Option<&str> {
            None
        }

        fn msg_data_format(&self) -> Option<&str> {
            None
        }

        fn expires_time(&self) -> i64 {
            0
        }

        fn http_proxy_username(&self) -> Option<String> {
            None
        }

        fn http_proxy_password(&self) -> Option<String> {
            None
        }

        fn host_config(&self) -> WxChannelHostConfig {
            WxChannelHostConfig::new()
        }

        fn set_host_config(&self, _host_config: WxChannelHostConfig) {}

        fn api_host_url(&self) -> Option<String> {
            None
        }

        fn set_api_host_url(&self, _api_host_url: &str) {}

        fn access_token_url(&self) -> Option<String> {
            None
        }

        fn set_access_token_url(&self, _access_token_url: &str) {}
    }

    /// 测试用 Mock 服务：`post` 记录 (url, body) 并返回预设响应；
    /// 模拟执行引擎的 errcode 校验（对应 Java 执行器语义）。
    pub struct MockChannelService {
        config: RwLock<Arc<dyn WxChannelConfig>>,
        client: reqwest::Client,
        requests: Arc<Mutex<Vec<(String, String)>>>,
        response: Mutex<String>,
    }

    impl MockChannelService {
        /// 构建 Mock 服务。
        ///
        /// # 参数
        /// - `response`：`post` 返回的响应体（errcode != 0 时模拟执行引擎抛错）
        pub fn new(response: &str) -> Self {
            Self {
                config: RwLock::new(Arc::new(MockChannelConfig::new())),
                client: reqwest::Client::new(),
                requests: Arc::new(Mutex::new(Vec::new())),
                response: Mutex::new(response.to_string()),
            }
        }

        /// 读取已记录的全部请求 (url, body)。
        pub fn requests(&self) -> Vec<(String, String)> {
            self.requests.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl WxChannelService for MockChannelService {
        fn wx_channel_config(&self) -> Arc<dyn WxChannelConfig> {
            self.config.read().unwrap().clone()
        }

        fn set_config(&self, config: Arc<dyn WxChannelConfig>) {
            *self.config.write().unwrap() = config;
        }

        fn http_client(&self) -> &reqwest::Client {
            &self.client
        }

        /// 记录请求并返回预设响应；响应 errcode != 0 时返回业务错误
        /// （对应 Java `SimplePostRequestExecutor.handleResponse` 抛
        /// `WxErrorException` 的语义）。
        async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
            self.requests
                .lock()
                .unwrap()
                .push((url.to_string(), post_data.to_string()));
            let response = self.response.lock().unwrap().clone();
            // 模拟执行引擎的 errcode 校验
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
                if let Some(code) = json.get("errcode").and_then(|v| v.as_i64()) {
                    if code != 0 {
                        let msg = json
                            .get("errmsg")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        return Err(WxErrorException::from_code(code as i32, msg));
                    }
                }
            }
            Ok(response)
        }
    }

    /// 构建 Mock 服务并返回 (服务 Arc, 门面弱引用)。
    ///
    /// 弱引用供各 `XxxServiceImpl::new(weak)` 注入；服务 Arc 供读取
    /// [`last_request`] 断言请求路径/请求体。
    pub fn build_service(response: &str) -> (Arc<MockChannelService>, Weak<dyn WxChannelService>) {
        let arc = Arc::new(MockChannelService::new(response));
        let weak: Weak<dyn WxChannelService> =
            Arc::downgrade(&(arc.clone() as Arc<dyn WxChannelService>));
        (arc, weak)
    }

    /// 取最近一次请求 (url, body)。
    pub fn last_request(svc: &MockChannelService) -> (String, String) {
        svc.requests()
            .last()
            .cloned()
            .unwrap_or_else(|| (String::new(), String::new()))
    }
}
