use std::sync::Arc;

use async_trait::async_trait;
use wx_rust_common::bean::CommonUploadParam;
use wx_rust_common::bean::ToJson;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::executor::CommonUploadRequestExecutor;
use wx_rust_common::util::crypto::Sha1;
use wx_rust_common::util::http::SimpleGetRequestExecutor;

use crate::api::{
    WxStoreAddressService, WxStoreAfterSaleService, WxStoreBasicService, WxStoreBrandService,
    WxStoreCategoryService, WxStoreCompassShopService, WxStoreCooperationService,
    WxStoreCouponService, WxStoreEwaybillService, WxStoreFavoriteService,
    WxStoreFreightTemplateService, WxStoreFundService, WxStoreGiftService, WxStoreHomePageService,
    WxStoreKfService, WxStoreLimitedDiscountService, WxStoreOrderService,
    WxStoreProductAssistantService, WxStoreProductService, WxStoreProductStockService,
    WxStoreQicService, WxStoreSharerService, WxStoreSupplierService, WxStoreVipService,
    WxStoreWarehouseService, WxTalentService,
};
use crate::config::WxStoreConfig;

/// 微信小店服务门面。
/// 对应 Java: com.binarywang.wxjava.store.api.WxStoreService
#[async_trait]
pub trait WxStoreService: Send + Sync {
    // ---- 基础能力（对应 Java `WxService` + `BaseWxStoreService`）----

    /// 当前微信小店配置存储（对应 Java `getConfig()`）。
    fn wx_store_config(&self) -> Arc<dyn WxStoreConfig>;

    /// 注入配置存储（对应 Java `setConfig(WxStoreConfig)`）。
    fn set_config(&self, config: Arc<dyn WxStoreConfig>);

    /// HTTP 客户端；在构建服务时初始化。对应 Java: initHttp。
    fn http_client(&self) -> &reqwest::Client;

    /// 验证消息是否来自微信服务器（对应 Java `checkSignature(String, String, String)`）。
    fn check_signature(&self, timestamp: &str, nonce: &str, signature: &str) -> bool {
        let config = self.wx_store_config();
        let token = config.token().unwrap_or_default();
        // Java `SHA1.gen(token, timestamp, nonce)`：排序后无分隔符拼接
        match Sha1::digest(&[token, timestamp, nonce]) {
            Ok(s) => s == signature,
            Err(_) => false,
        }
    }

    /// 获取 access_token（对应 Java `getAccessToken()`，不强制刷新）。
    async fn get_access_token(&self) -> Result<String, WxErrorException> {
        self.get_access_token_with_force(false).await
    }

    /// 获取 access_token（对应 Java `getAccessToken(boolean forceRefresh)`）。
    ///
    /// 双检锁 + tryLock(100ms) 轮询 + 3 秒超时；稳定版接口按配置切换
    /// （与 mp/miniapp 同一实现）。
    async fn get_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_store_config();
        if !force_refresh && !config.is_access_token_expired() {
            return config
                .access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
        }

        let lock = config.access_token_lock();
        let timeout_at = std::time::Instant::now() + std::time::Duration::from_millis(3000);
        // 对应 Java tryLock(100ms) 轮询：guard 必须持有到刷新完成（双检锁）
        let _guard = loop {
            if !force_refresh && !config.is_access_token_expired() {
                return config
                    .access_token()
                    .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
            }
            match lock.try_lock() {
                Ok(guard) => break guard,
                Err(_) => {
                    if std::time::Instant::now() > timeout_at {
                        return Err(WxErrorException::from_code(
                            -99,
                            "获取accessToken超时：获取时间超时",
                        ));
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        };

        let response = if config.is_stable_access_token() {
            self.do_get_stable_access_token_request(force_refresh)
                .await?
        } else {
            self.do_get_access_token_request().await?
        };
        let token = self.extract_access_token(&response)?;
        Ok(token)
    }

    /// GET 请求（对应 Java `get(String url, String queryParam)`）。
    ///
    /// **保留原执行器路径**（`SimpleGetRequestExecutor` + `execute`）：
    /// 测试 `get_appends_token_and_query` 冻结了 Java 字节序
    /// 「token 在前、query 在后」（`?access_token=..&a=1`）；统一管线
    /// 在组装 URL 时注入 token（恒在末尾），无法复现该顺序——按
    /// 「宁可少接也不改语义」约定 get 不接管线（见 task-4 报告）。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_store_service_impl::execute(
            self,
            &executor,
            url,
            query_param.to_string(),
        )
        .await
    }

    /// POST 请求（对应 Java `post(String url, String postData)`）。
    ///
    /// 走统一管线（经 `execute_post_via_pipeline`：POST 文本体原样透传 +
    /// -1 指数退避重试（channel 特有「原错误码 + ！」收束）+
    /// `api_host_url` 域名替换 + token 失效单次重放——原
    /// `SimplePostRequestExecutor` 路径；POST 无 query 拼接，URL 字节序
    /// 与原路径完全一致）。
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_store_service_impl::execute_post_via_pipeline(
            self, url, post_data,
        )
        .await
    }

    /// POST 请求（对应 Java `post(String url, JsonObject jsonObject)`，
    /// 以 `jsonObject.toString()` 为请求体）。
    async fn post_json(
        &self,
        url: &str,
        json_object: &serde_json::Value,
    ) -> Result<String, WxErrorException> {
        self.post(url, &json_object.to_string()).await
    }

    /// POST 请求（对应 Java `post(String url, ToJson obj)`，以 `obj.toJson()` 为请求体）。
    ///
    /// Rust 适配：约束 `ToJson + Send + Sync` 保证 async trait 方法 future 可 Send。
    async fn post_to_json(
        &self,
        url: &str,
        obj: &(dyn ToJson + Send + Sync),
    ) -> Result<String, WxErrorException> {
        // 先同步序列化再 await，避免跨 await 持有引用
        let body = obj.to_json();
        self.post(url, &body).await
    }

    /// 文件上传请求（对应 Java `upload(String url, CommonUploadParam param)`）。
    async fn upload(
        &self,
        url: &str,
        param: CommonUploadParam,
    ) -> Result<String, WxErrorException> {
        let executor = CommonUploadRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_store_service_impl::execute(self, &executor, url, param).await
    }

    /// 设置微信系统繁忙时的重试等待毫秒数（对应 Java `setRetrySleepMillis(int)`；
    /// 默认 1000ms，委托配置存储）。
    fn set_retry_sleep_millis(&self, retry_sleep_millis: i32) {
        self.wx_store_config()
            .set_retry_sleep_millis(retry_sleep_millis);
    }

    /// 设置微信系统繁忙时的最大重试次数（对应 Java `setMaxRetryTimes(int)`；默认 5 次）。
    fn set_max_retry_times(&self, max_retry_times: i32) {
        self.wx_store_config().set_max_retry_times(max_retry_times);
    }

    /// 通过网络请求获取 access_token（对应 Java 抽象方法 `doGetAccessTokenRequest`）。
    ///
    /// 配置了自定义 `accessTokenUrl`（`%s` 格式串，Java `String.format` 语义）
    /// 时优先使用，否则走标准 `/cgi-bin/token` 地址。
    async fn do_get_access_token_request(&self) -> Result<String, WxErrorException> {
        let config = self.wx_store_config();
        let url = match config.access_token_url() {
            Some(u) if !u.is_empty() => {
                // Java String.format(url, appid, secret)：按序替换 %s
                u.replacen("%s", config.app_id(), 1)
                    .replacen("%s", config.secret(), 1)
            }
            _ => crate::enums::url_core::GET_ACCESS_TOKEN_URL
                .replacen("%s", config.app_id(), 1)
                .replacen("%s", config.secret(), 1),
        };
        let client = self.http_client();
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        let body = resp
            .text()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        Ok(body)
    }

    /// 通过稳定版接口获取 access_token（对应 Java 抽象方法
    /// `doGetStableAccessTokenRequest(boolean forceRefresh)`）。
    async fn do_get_stable_access_token_request(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_store_config();
        let url = match config.access_token_url() {
            Some(u) if !u.is_empty() => u.to_string(),
            _ => crate::enums::url_core::GET_STABLE_ACCESS_TOKEN_URL.to_string(),
        };
        let body = serde_json::json!({
            "grant_type": "client_credential",
            "appid": config.app_id(),
            "secret": config.secret(),
            "force_refresh": force_refresh,
        });
        let client = self.http_client();
        let resp = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        let text = resp
            .text()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        Ok(text)
    }

    /// 提取 access token（对应 Java `extractAccessToken`）。
    ///
    /// 解析响应 JSON，失败时抛业务错误；成功时更新配置缓存。
    fn extract_access_token(&self, result_content: &str) -> Result<String, WxErrorException> {
        let config = self.wx_store_config();
        let error = wx_rust_common::error::WxError::from_json_with_type(
            result_content,
            Some(wx_rust_common::enums::WxType::Channel),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        let json: serde_json::Value = serde_json::from_str(result_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let access_token = json
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WxErrorException::from_code(-99, "access_token 字段缺失"))?
            .to_string();
        let expires_in = json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        config.update_access_token(&access_token, expires_in);
        Ok(config.access_token().unwrap_or(access_token))
    }

    /// 获取address经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getAddressService
    fn address_service(&self) -> Option<Arc<dyn WxStoreAddressService>>;
    /// 获取after_sale经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getAfterSaleService
    fn after_sale_service(&self) -> Option<Arc<dyn WxStoreAfterSaleService>>;
    /// 获取basic经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getBasicService
    fn basic_service(&self) -> Option<Arc<dyn WxStoreBasicService>>;
    /// 获取brand经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getBrandService
    fn brand_service(&self) -> Option<Arc<dyn WxStoreBrandService>>;
    /// 获取category经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getCategoryService
    fn category_service(&self) -> Option<Arc<dyn WxStoreCategoryService>>;
    /// 获取compass_shop经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getCompassShopService
    fn compass_shop_service(&self) -> Option<Arc<dyn WxStoreCompassShopService>>;
    /// 获取cooperation经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getCooperationService
    fn cooperation_service(&self) -> Option<Arc<dyn WxStoreCooperationService>>;
    /// 获取coupon经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getCouponService
    fn coupon_service(&self) -> Option<Arc<dyn WxStoreCouponService>>;
    /// 获取ewaybill经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getEwaybillService
    fn ewaybill_service(&self) -> Option<Arc<dyn WxStoreEwaybillService>>;
    /// 获取favorite经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getFavoriteService
    fn favorite_service(&self) -> Option<Arc<dyn WxStoreFavoriteService>>;
    /// 获取freight_template经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getFreightTemplateService
    fn freight_template_service(&self) -> Option<Arc<dyn WxStoreFreightTemplateService>>;
    /// 获取fund经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getFundService
    fn fund_service(&self) -> Option<Arc<dyn WxStoreFundService>>;
    /// 获取gift经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getGiftService
    fn gift_service(&self) -> Option<Arc<dyn WxStoreGiftService>>;
    /// 获取home_page经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getHomePageService
    fn home_page_service(&self) -> Option<Arc<dyn WxStoreHomePageService>>;
    /// 获取kf经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getKfService
    fn kf_service(&self) -> Option<Arc<dyn WxStoreKfService>>;
    /// 获取limited_discount经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getLimitedDiscountService
    fn limited_discount_service(&self) -> Option<Arc<dyn WxStoreLimitedDiscountService>>;
    /// 获取order经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getOrderService
    fn order_service(&self) -> Option<Arc<dyn WxStoreOrderService>>;
    /// 获取product_assistant经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getProductAssistantService
    fn product_assistant_service(&self) -> Option<Arc<dyn WxStoreProductAssistantService>>;
    /// 获取product经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getProductService
    fn product_service(&self) -> Option<Arc<dyn WxStoreProductService>>;
    /// 获取product_stock经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getProductStockService
    fn product_stock_service(&self) -> Option<Arc<dyn WxStoreProductStockService>>;
    /// 获取qic经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getQicService
    fn qic_service(&self) -> Option<Arc<dyn WxStoreQicService>>;
    /// 获取sharer经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getSharerService
    fn sharer_service(&self) -> Option<Arc<dyn WxStoreSharerService>>;
    /// 获取supplier经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getSupplierService
    fn supplier_service(&self) -> Option<Arc<dyn WxStoreSupplierService>>;
    /// 获取vip经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getVipService
    fn vip_service(&self) -> Option<Arc<dyn WxStoreVipService>>;
    /// 获取warehouse经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getWarehouseService
    fn warehouse_service(&self) -> Option<Arc<dyn WxStoreWarehouseService>>;
    /// 获取talent经营服务，返回已装配的服务实例。对应 Java: WxStoreService#getTalentService
    fn talent_service(&self) -> Option<Arc<dyn WxTalentService>>;
}
