#[cfg(test)]
pub mod test_support {
    use std::sync::{Arc, Mutex, RwLock, Weak};

    use async_trait::async_trait;
    use wx_rust_common::config::WxConfigStorage;
    use wx_rust_common::error::WxErrorException;

    use crate::api::WxStoreService;
    use crate::config::{WxStoreConfig, WxStoreHostConfig};

    /// 测试用最小配置存储（实现 `WxStoreConfig`；access_token 预置且永不过期）。
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

    impl WxStoreConfig for MockChannelConfig {
        fn set_retry_sleep_millis(&self, _value: i32) {}
        fn set_max_retry_times(&self, _value: i32) {}
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

        fn host_config(&self) -> WxStoreHostConfig {
            WxStoreHostConfig::new()
        }

        fn set_host_config(&self, _host_config: WxStoreHostConfig) {}

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
        config: RwLock<Arc<dyn WxStoreConfig>>,
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
    impl WxStoreService for MockChannelService {
        fn address_service(&self) -> Option<Arc<dyn crate::api::WxStoreAddressService>> {
            None
        }
        fn after_sale_service(&self) -> Option<Arc<dyn crate::api::WxStoreAfterSaleService>> {
            None
        }
        fn basic_service(&self) -> Option<Arc<dyn crate::api::WxStoreBasicService>> {
            None
        }
        fn brand_service(&self) -> Option<Arc<dyn crate::api::WxStoreBrandService>> {
            None
        }
        fn category_service(&self) -> Option<Arc<dyn crate::api::WxStoreCategoryService>> {
            None
        }
        fn compass_shop_service(&self) -> Option<Arc<dyn crate::api::WxStoreCompassShopService>> {
            None
        }
        fn cooperation_service(&self) -> Option<Arc<dyn crate::api::WxStoreCooperationService>> {
            None
        }
        fn coupon_service(&self) -> Option<Arc<dyn crate::api::WxStoreCouponService>> {
            None
        }
        fn ewaybill_service(&self) -> Option<Arc<dyn crate::api::WxStoreEwaybillService>> {
            None
        }
        fn favorite_service(&self) -> Option<Arc<dyn crate::api::WxStoreFavoriteService>> {
            None
        }
        fn freight_template_service(
            &self,
        ) -> Option<Arc<dyn crate::api::WxStoreFreightTemplateService>> {
            None
        }
        fn fund_service(&self) -> Option<Arc<dyn crate::api::WxStoreFundService>> {
            None
        }
        fn gift_service(&self) -> Option<Arc<dyn crate::api::WxStoreGiftService>> {
            None
        }
        fn home_page_service(&self) -> Option<Arc<dyn crate::api::WxStoreHomePageService>> {
            None
        }
        fn kf_service(&self) -> Option<Arc<dyn crate::api::WxStoreKfService>> {
            None
        }
        fn limited_discount_service(
            &self,
        ) -> Option<Arc<dyn crate::api::WxStoreLimitedDiscountService>> {
            None
        }
        fn order_service(&self) -> Option<Arc<dyn crate::api::WxStoreOrderService>> {
            None
        }
        fn product_assistant_service(
            &self,
        ) -> Option<Arc<dyn crate::api::WxStoreProductAssistantService>> {
            None
        }
        fn product_service(&self) -> Option<Arc<dyn crate::api::WxStoreProductService>> {
            None
        }
        fn product_stock_service(&self) -> Option<Arc<dyn crate::api::WxStoreProductStockService>> {
            None
        }
        fn qic_service(&self) -> Option<Arc<dyn crate::api::WxStoreQicService>> {
            None
        }
        fn sharer_service(&self) -> Option<Arc<dyn crate::api::WxStoreSharerService>> {
            None
        }
        fn supplier_service(&self) -> Option<Arc<dyn crate::api::WxStoreSupplierService>> {
            None
        }
        fn vip_service(&self) -> Option<Arc<dyn crate::api::WxStoreVipService>> {
            None
        }
        fn warehouse_service(&self) -> Option<Arc<dyn crate::api::WxStoreWarehouseService>> {
            None
        }
        fn talent_service(&self) -> Option<Arc<dyn crate::api::WxTalentService>> {
            None
        }

        fn wx_store_config(&self) -> Arc<dyn WxStoreConfig> {
            self.config.read().unwrap().clone()
        }

        fn set_config(&self, config: Arc<dyn WxStoreConfig>) {
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
    pub fn build_service(response: &str) -> (Arc<MockChannelService>, Weak<dyn WxStoreService>) {
        let arc = Arc::new(MockChannelService::new(response));
        let weak: Weak<dyn WxStoreService> =
            Arc::downgrade(&(arc.clone() as Arc<dyn WxStoreService>));
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
