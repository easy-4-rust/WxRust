//! 基础实现（对应 Java `service.impl.BaseWxPayServiceImpl` +
//! `WxPayServiceHttpComponentsImpl` + `WxPayServiceImpl`）。
//!
//! Java 三层继承链在 Rust 以"门面 trait 默认实现 + 本模块实现体"表达：
//! - `execute_post`：v2 XML POST 执行引擎（对应 `WxPayServiceHttpComponentsImpl.post`，
//!   无 access_token、无重试，pay 与 mp/miniapp 的本质差异）；
//! - `WxPayServiceImpl`：提供配置存储与 HTTP 客户端，并覆写配置管理
//!   （多商户 configMap + 切换，对应 `BaseWxPayServiceImpl` 的
//!   `addConfig`/`removeConfig`/`setMultiConfig`/`switchover`/`switchoverTo`）、
//!   v2 POST 通道与 `getWxApiData`；
//! - v3 通道（`postV3`/`getV3`/...，需 RSA 私钥签名 + 平台证书验签）与
//!   p12 证书通道（`useKey=true`）留待 Wave 3/Wave 2，当前返回
//!   `-99 未实现` 错误（见 `WxPayService` trait 默认实现）。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::sync::Weak;
use std::time::Duration;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::r#impl::SubServiceBundle;
use crate::api::{
    Apply4SubjectConfirmService, Applyment4SubService, BankService, BrandMerchantTransferService,
    BusinessCircleService, BusinessOperationTransferService, ComplaintService,
    CustomDeclarationService, EcommerceService, EntPayService, MarketingBusiFavorService,
    MarketingFavorService, MarketingMediaService, MerchantLimitationService, MerchantMediaService,
    MerchantTransferService, MiPayService, PartnerPayScoreService, PartnerPayScoreSignPlanService,
    PartnerTransferService, PayScoreService, PayrollService, ProfitSharingService, RealNameService,
    RedpackService, SubscriptionBillingService, TransferService, WxDepositService,
    WxEntrustPapService, WxPayService,
};
use crate::bean::WxPayApiData;
use crate::config::WxPayConfig;

/// 执行 v2 XML POST 请求（对应 Java `WxPayServiceHttpComponentsImpl.post`）。
///
/// pay 与 mp/miniapp 执行引擎的本质差异：**无 access_token**（不需要 token
/// 注入/自动刷新），也**无重试循环**（Java 支付侧直接执行，无
/// `executeWithRetry`）。
///
/// # 参数
/// - `config`：当前支付配置（决定沙箱基地址等，仅作证书通道判断）
/// - `client`：HTTP 客户端
/// - `url`：完整请求地址（由 `WxPayService::get_pay_base_url()` + 路径拼接）
/// - `request_str`：v2 XML 报文（含签名）
/// - `use_key`：是否使用证书（p12）通道
/// - `mime_type`：自定义 Content-Type（Java `post(..., mimeType)`；为空时
///   默认 `text/xml`）
pub async fn execute_post(
    config: &dyn WxPayConfig,
    client: &reqwest::Client,
    url: &str,
    request_str: &str,
    use_key: bool,
    mime_type: Option<&str>,
) -> Result<String, WxErrorException> {
    if use_key {
        // 对应 Java `initSslHttpClient()`（p12 证书 → SSLContext）；p12
        // 解析（Java `initSSLContext`/`p12ToPem`）由 Wave 2 提供
        // （`util::wx_pay_service_impl_utils::build_cert_client`）。
        let cert_client = crate::util::wx_pay_service_impl_utils::build_cert_client(config)?;
        let mut request = cert_client.post(url).body(request_str.to_string());
        match mime_type {
            Some(m) => request = request.header(reqwest::header::CONTENT_TYPE, m),
            None => request = request.header(reqwest::header::CONTENT_TYPE, "text/xml"),
        }
        let resp = request
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        let text = resp
            .text()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        return Ok(text);
    }
    let mut request = client.post(url).body(request_str.to_string());
    match mime_type {
        Some(m) => request = request.header(reqwest::header::CONTENT_TYPE, m),
        None => request = request.header(reqwest::header::CONTENT_TYPE, "text/xml"),
    }
    let resp = request
        .send()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;
    let text = resp
        .text()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;
    Ok(text)
}

/// 微信支付服务实现。
///
/// 对应 Java `WxPayServiceImpl`（→ `WxPayServiceHttpComponentsImpl` →
/// `BaseWxPayServiceImpl`）的 Rust 组合体：持有主配置（`config`）、多商户
/// 配置表（`config_map`，键为 `mchId_appId` 或自定义配置键）、当前切换键
/// （`default_config_key`，对应 Java `WxPayConfigHolder` ThreadLocal 的
/// 简化形态）与最近一次接口数据（`wx_api_data`，对应 Java `wxApiData`
/// ThreadLocal，`ifSaveApiData` 控制记录）。
pub struct WxPayServiceImpl {
    /// 主配置（对应 Java `BaseWxPayServiceImpl.config`；trait 方法以 `&self`
    /// 访问，故以 `RwLock` 承载切换赋值）
    config: RwLock<Arc<dyn WxPayConfig>>,
    /// HTTP 客户端（对应 Java `CloseableHttpClient`，reqwest 统一实现）
    http_client: reqwest::Client,
    /// 多商户配置表（对应 Java `configMap`）
    config_map: RwLock<HashMap<String, Arc<dyn WxPayConfig>>>,
    /// 当前切换的配置键（对应 Java `WxPayConfigHolder.get()`）
    default_config_key: RwLock<Option<String>>,
    /// 最近一次接口请求/响应数据（对应 Java `wxApiData` ThreadLocal）
    wx_api_data: RwLock<Option<WxPayApiData>>,
    /// 子服务集合（对应 Java `WxPayServiceImpl` 内 29 个子服务字段；
    /// Wave 5 P5 装配，`new_arc` 中以门面弱引用构建）
    sub_services: OnceLock<SubServiceBundle>,
}

impl WxPayServiceImpl {
    /// 构建服务并以 `Arc` 返回（对应 Java `new WxPayServiceImpl()` 后
    /// `setConfig(...)` 的惯用组合）。
    ///
    /// HTTP 客户端按配置的 `httpConnectionTimeout`/`httpTimeout` 构建；
    /// 构建失败时回退默认客户端。
    pub fn new_arc(config: Arc<dyn WxPayConfig>) -> Arc<Self> {
        let http_client = reqwest::Client::builder()
            .connect_timeout(Duration::from_millis(
                config.http_connection_timeout() as u64
            ))
            .timeout(Duration::from_millis(config.http_timeout() as u64))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        let this = Arc::new(Self {
            config: RwLock::new(config),
            http_client,
            config_map: RwLock::new(HashMap::new()),
            default_config_key: RwLock::new(None),
            wx_api_data: RwLock::new(None),
            sub_services: OnceLock::new(),
        });
        // 子服务装配（对应 Java 构造器 `setXxxService(new XxxServiceImpl(this))`）：
        // 以 Weak 引用避免 Arc 环泄漏（Java 由 GC 处理同构引用环）。
        let weak: Weak<dyn WxPayService> = Arc::downgrade(&(this.clone() as Arc<dyn WxPayService>));
        let _ = this.sub_services.set(SubServiceBundle::new(weak));
        this
    }

    /// 生成配置键 `mchId + "_" + appId`（对应 Java `getConfigKey(String, String)`）。
    pub fn get_config_key(mch_id: &str, app_id: &str) -> String {
        format!("{mch_id}_{app_id}")
    }

    /// 记录最近一次接口请求/响应数据（对应 Java
    /// `wxApiData.set(new WxPayApiData(...))`，`ifSaveApiData` 控制）。
    fn record_api_data(
        &self,
        url: &str,
        request_str: &str,
        response: &Result<String, WxErrorException>,
    ) {
        if !self.wx_pay_config().if_save_api_data() {
            return;
        }
        let (response_data, error_msg) = match response {
            Ok(s) => (Some(s.clone()), None),
            Err(e) => (None, Some(e.to_string())),
        };
        *self.wx_api_data.write().unwrap() = Some(WxPayApiData::new(
            Some(url.to_string()),
            Some(request_str.to_string()),
            response_data,
            error_msg,
        ));
    }
}

#[async_trait]
impl WxPayService for WxPayServiceImpl {
    fn wx_pay_config(&self) -> Arc<dyn WxPayConfig> {
        self.config.read().unwrap().clone()
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    // ---- 配置管理（对应 Java BaseWxPayServiceImpl 全量镜像） ----

    fn add_config(&self, mch_id: &str, app_id: &str, wx_pay_config: Arc<dyn WxPayConfig>) {
        self.config_map
            .write()
            .unwrap()
            .insert(Self::get_config_key(mch_id, app_id), wx_pay_config);
    }

    fn add_config_with_key(&self, config_key: &str, wx_pay_config: Arc<dyn WxPayConfig>) {
        self.config_map
            .write()
            .unwrap()
            .insert(config_key.to_string(), wx_pay_config);
    }

    fn remove_config(&self, mch_id: &str, app_id: &str) {
        self.config_map
            .write()
            .unwrap()
            .remove(&Self::get_config_key(mch_id, app_id));
    }

    fn remove_config_with_key(&self, config_key: &str) {
        self.config_map.write().unwrap().remove(config_key);
    }

    fn set_multi_config(&self, wx_pay_configs: &HashMap<String, Arc<dyn WxPayConfig>>) {
        // 对应 Java：随机采用一个 mchId 进行 Http 初始化（此处取首个键）
        let default_key = wx_pay_configs.keys().next().cloned().unwrap_or_default();
        self.set_multi_config_with_default(wx_pay_configs, &default_key);
    }

    fn set_multi_config_with_default(
        &self,
        wx_pay_configs: &HashMap<String, Arc<dyn WxPayConfig>>,
        default_mch_id: &str,
    ) {
        *self.config_map.write().unwrap() = wx_pay_configs.clone();
        let default_config = wx_pay_configs
            .get(default_mch_id)
            .cloned()
            .unwrap_or_else(|| {
                self.config_map
                    .read()
                    .unwrap()
                    .values()
                    .next()
                    .cloned()
                    .unwrap_or_else(|| self.wx_pay_config())
            });
        // 对应 Java `WxRuntimeException("默认配置不存在")`：配置表为空时
        // 回退主配置（Wave 1 可改为返回错误）
        *self.config.write().unwrap() = default_config;
        *self.default_config_key.write().unwrap() = Some(default_mch_id.to_string());
    }

    fn switchover(&self, mch_id: &str, app_id: &str) -> bool {
        let key = Self::get_config_key(mch_id, app_id);
        let map = self.config_map.read().unwrap();
        if let Some(config) = map.get(&key) {
            *self.config.write().unwrap() = config.clone();
            *self.default_config_key.write().unwrap() = Some(key);
            return true;
        }
        false
    }

    fn switchover_with_key(&self, mch_id_or_config_key: &str) -> bool {
        let map = self.config_map.read().unwrap();
        // 先精确匹配（含自定义配置键），再前缀匹配 `mchId_*`
        if let Some(config) = map.get(mch_id_or_config_key) {
            *self.config.write().unwrap() = config.clone();
            *self.default_config_key.write().unwrap() = Some(mch_id_or_config_key.to_string());
            return true;
        }
        let prefix = format!("{mch_id_or_config_key}_");
        for (key, config) in map.iter() {
            if key.starts_with(&prefix) {
                *self.config.write().unwrap() = config.clone();
                *self.default_config_key.write().unwrap() = Some(key.clone());
                return true;
            }
        }
        false
    }

    async fn switchover_to(&self, mch_id: &str, app_id: &str) -> Result<(), WxErrorException> {
        if self.switchover(mch_id, app_id) {
            Ok(())
        } else {
            Err(WxErrorException::from_code(-99, "未找到对应配置"))
        }
    }

    async fn switchover_to_with_key(
        &self,
        mch_id_or_config_key: &str,
    ) -> Result<(), WxErrorException> {
        if self.switchover_with_key(mch_id_or_config_key) {
            Ok(())
        } else {
            Err(WxErrorException::from_code(-99, "未找到对应配置"))
        }
    }

    fn get_config(&self) -> Arc<dyn WxPayConfig> {
        let map = self.config_map.read().unwrap();
        if map.len() == 1 {
            // 只有一个商户号，直接返回其配置即可（对应 Java）
            return map.values().next().unwrap().clone();
        }
        if let Some(key) = self.default_config_key.read().unwrap().as_ref() {
            if let Some(config) = map.get(key) {
                return config.clone();
            }
        }
        self.wx_pay_config()
    }

    fn get_config_by_mch_app(&self, mch_id: &str, app_id: &str) -> Option<Arc<dyn WxPayConfig>> {
        self.config_map
            .read()
            .unwrap()
            .get(&Self::get_config_key(mch_id, app_id))
            .cloned()
    }

    fn get_config_by_mch(&self, mch_id: &str) -> Option<Arc<dyn WxPayConfig>> {
        let map = self.config_map.read().unwrap();
        if let Some(config) = map.get(mch_id) {
            return Some(config.clone());
        }
        let prefix = format!("{mch_id}_");
        map.iter()
            .find(|(key, _)| key.starts_with(&prefix))
            .map(|(_, config)| config.clone())
    }

    fn set_config(&self, config: Arc<dyn WxPayConfig>) {
        // 对应 Java：以 `mchId_appId` 为键注册到 configMap 并切换
        let key = Self::get_config_key(
            config.mch_id().unwrap_or_default(),
            config.app_id().unwrap_or_default(),
        );
        self.config_map
            .write()
            .unwrap()
            .insert(key.clone(), config.clone());
        *self.config.write().unwrap() = config;
        *self.default_config_key.write().unwrap() = Some(key);
    }

    // ---- HTTP 执行引擎覆写（对应 Java WxPayServiceHttpComponentsImpl） ----

    async fn post_for_bytes(
        &self,
        url: &str,
        request_str: &str,
        use_key: bool,
    ) -> Result<Vec<u8>, WxErrorException> {
        let client = if use_key {
            // 证书通道（对应 Java `initSslHttpClient`，p12 证书）
            crate::util::wx_pay_service_impl_utils::build_cert_client(
                self.wx_pay_config().as_ref(),
            )?
        } else {
            self.http_client.clone()
        };
        let resp = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "text/xml")
            .body(request_str.to_string())
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?
            .to_vec();
        Ok(bytes)
    }

    async fn post(
        &self,
        url: &str,
        request_str: &str,
        use_key: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_pay_config();
        let result = execute_post(
            config.as_ref(),
            &self.http_client,
            url,
            request_str,
            use_key,
            None,
        )
        .await;
        self.record_api_data(url, request_str, &result);
        result
    }

    async fn post_with_mime_type(
        &self,
        url: &str,
        request_str: &str,
        use_key: bool,
        mime_type: &str,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_pay_config();
        let result = execute_post(
            config.as_ref(),
            &self.http_client,
            url,
            request_str,
            use_key,
            Some(mime_type),
        )
        .await;
        self.record_api_data(url, request_str, &result);
        result
    }

    fn get_wx_api_data(&self) -> Option<WxPayApiData> {
        self.wx_api_data.read().unwrap().clone()
    }

    // ---- 子服务 getter 覆写（Wave 5 P5：装配后的实例，对应 Java
    // `getXxxService()` 返回非空实例） ----

    fn wx_entrust_pap_service(&self) -> Option<Arc<dyn WxEntrustPapService>> {
        self.sub_services
            .get()
            .and_then(|b| b.wx_entrust_pap.clone())
    }

    fn wx_deposit_service(&self) -> Option<Arc<dyn WxDepositService>> {
        self.sub_services.get().and_then(|b| b.wx_deposit.clone())
    }

    fn partner_transfer_service(&self) -> Option<Arc<dyn PartnerTransferService>> {
        self.sub_services
            .get()
            .and_then(|b| b.partner_transfer.clone())
    }

    fn payroll_service(&self) -> Option<Arc<dyn PayrollService>> {
        self.sub_services.get().and_then(|b| b.payroll.clone())
    }

    fn ent_pay_service(&self) -> Option<Arc<dyn EntPayService>> {
        self.sub_services.get().and_then(|b| b.ent_pay.clone())
    }

    fn redpack_service(&self) -> Option<Arc<dyn RedpackService>> {
        self.sub_services.get().and_then(|b| b.redpack.clone())
    }

    fn profit_sharing_service(&self) -> Option<Arc<dyn ProfitSharingService>> {
        self.sub_services
            .get()
            .and_then(|b| b.profit_sharing.clone())
    }

    fn pay_score_service(&self) -> Option<Arc<dyn PayScoreService>> {
        self.sub_services.get().and_then(|b| b.pay_score.clone())
    }

    fn ecommerce_service(&self) -> Option<Arc<dyn EcommerceService>> {
        self.sub_services.get().and_then(|b| b.ecommerce.clone())
    }

    fn business_circle_service(&self) -> Option<Arc<dyn BusinessCircleService>> {
        self.sub_services
            .get()
            .and_then(|b| b.business_circle.clone())
    }

    fn merchant_media_service(&self) -> Option<Arc<dyn MerchantMediaService>> {
        self.sub_services
            .get()
            .and_then(|b| b.merchant_media.clone())
    }

    fn marketing_media_service(&self) -> Option<Arc<dyn MarketingMediaService>> {
        self.sub_services
            .get()
            .and_then(|b| b.marketing_media.clone())
    }

    fn marketing_favor_service(&self) -> Option<Arc<dyn MarketingFavorService>> {
        self.sub_services
            .get()
            .and_then(|b| b.marketing_favor.clone())
    }

    fn marketing_busi_favor_service(&self) -> Option<Arc<dyn MarketingBusiFavorService>> {
        self.sub_services
            .get()
            .and_then(|b| b.marketing_busi_favor.clone())
    }

    fn merchant_transfer_service(&self) -> Option<Arc<dyn MerchantTransferService>> {
        self.sub_services
            .get()
            .and_then(|b| b.merchant_transfer.clone())
    }

    fn brand_merchant_transfer_service(&self) -> Option<Arc<dyn BrandMerchantTransferService>> {
        self.sub_services
            .get()
            .and_then(|b| b.brand_merchant_transfer.clone())
    }

    fn subscription_billing_service(&self) -> Option<Arc<dyn SubscriptionBillingService>> {
        self.sub_services
            .get()
            .and_then(|b| b.subscription_billing.clone())
    }

    fn merchant_limitation_service(&self) -> Option<Arc<dyn MerchantLimitationService>> {
        self.sub_services
            .get()
            .and_then(|b| b.merchant_limitation.clone())
    }

    fn complaints_service(&self) -> Option<Arc<dyn ComplaintService>> {
        self.sub_services.get().and_then(|b| b.complaint.clone())
    }

    fn bank_service(&self) -> Option<Arc<dyn BankService>> {
        self.sub_services.get().and_then(|b| b.bank.clone())
    }

    fn transfer_service(&self) -> Option<Arc<dyn TransferService>> {
        self.sub_services.get().and_then(|b| b.transfer.clone())
    }

    fn business_operation_transfer_service(
        &self,
    ) -> Option<Arc<dyn BusinessOperationTransferService>> {
        self.sub_services
            .get()
            .and_then(|b| b.business_operation_transfer.clone())
    }

    fn partner_pay_score_service(&self) -> Option<Arc<dyn PartnerPayScoreService>> {
        self.sub_services
            .get()
            .and_then(|b| b.partner_pay_score.clone())
    }

    fn partner_pay_score_sign_plan_service(
        &self,
    ) -> Option<Arc<dyn PartnerPayScoreSignPlanService>> {
        self.sub_services
            .get()
            .and_then(|b| b.partner_pay_score_sign_plan.clone())
    }

    fn real_name_service(&self) -> Option<Arc<dyn RealNameService>> {
        self.sub_services.get().and_then(|b| b.real_name.clone())
    }

    fn mi_pay_service(&self) -> Option<Arc<dyn MiPayService>> {
        self.sub_services.get().and_then(|b| b.mi_pay.clone())
    }

    fn apply4_subject_confirm_service(&self) -> Option<Arc<dyn Apply4SubjectConfirmService>> {
        self.sub_services
            .get()
            .and_then(|b| b.apply4_subject_confirm.clone())
    }

    fn applyment4_sub_service(&self) -> Option<Arc<dyn Applyment4SubService>> {
        self.sub_services
            .get()
            .and_then(|b| b.applyment4_sub.clone())
    }

    fn custom_declaration_service(&self) -> Option<Arc<dyn CustomDeclarationService>> {
        self.sub_services
            .get()
            .and_then(|b| b.custom_declaration.clone())
    }
}
