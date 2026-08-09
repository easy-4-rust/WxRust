//! 微信支付服务门面。
//!
//! 对应 Java `com.github.binarywang.wxpay.service.WxPayService` 接口 +
//! `service.impl.BaseWxPayServiceImpl`（执行引擎、多商户配置切换、v2/v3
//! 请求通道）。Java 三层继承链（`WxPayServiceImpl` → `WxPayServiceHttpComponentsImpl`
//! → `BaseWxPayServiceImpl`）在 Rust 以 trait 默认实现 + 组合表达（与
//! mp/miniapp 同一设计原则）：本 trait 携带 Base 的默认实现（配置管理、
//! v2 XML POST 引擎、沙箱基地址等），具体实现仅需提供配置存储与 HTTP 客户端。
//!
//! 签名冻结口径（Wave 0）：
//! - Java 接口 `WxPayService.java` 的 **139 个方法全部镜像** 为本 trait 方法
//!   （含 2 个 `default` 方法 `switchover(String)`/`switchoverTo(String)`）。
//! - Java 泛型方法（`<T> T createOrder(...)`/`createOrderV3`/`combineTransactions`/
//!   `baseParseOrderNotifyV3Result`）在 Rust 以 `serde_json::Value` 类型擦除
//!   （`ADAPTED`：泛型方法破坏 trait 对象安全，以具体 bean 类型或泛型自由
//!   函数承接）；`HttpPost`/`HttpRequestBase` 参数以 `reqwest::Request`
//!   镜像（`ADAPTED`）；`InputStream` 返回值以 `Vec<u8>` 镜像（`ADAPTED`）。
//! - pay 无 access_token；v2 报文签名（MD5/HMAC-SHA256）与 v3 RSA 签名
//!   见 `util::sign_utils` 与 `util::crypto`。
//! - 29 个子服务 getter（`getXxxService()`）默认返回 `None`，由具体实现
//!   （`WxPayServiceImpl`，Wave 5 P5 装配）覆写为 `SubServiceBundle` 实例。
//!
//! Wave 2 P2a 实现说明：
//! - v2 方法族（下单/查询/关闭/退款/账单/支付分等）在默认实现中覆写，
//!   语义逐行镜像 Java `BaseWxPayServiceImpl`：`checkAndSign`（配置回填 +
//!   签名）、`BaseWxPayResult.fromXML`、`checkResult`（验签 +
//!   return_code/result_code 成功校验）见 `util::wx_pay_service_impl_utils`；
//! - v3 方法族走 `util::wx_pay_service_impl_utils::execute_v3`（Authorization
//!   头 + Wechatpay-Serial 头 + 响应验签 + v3 错误 JSON 转换）；平台证书
//!   验签衔接 `config` 的微信支付公钥（完全公钥模式），证书自动更新为
//!   Wave 2 P2b 职责（`platform_public_key` 中留有错误分支）；
//! - 通知解析：v3 通知复用 `util::wx_pay_notify_utils`（验签 + AES-GCM
//!   解密），v2 退款通知 `req_info` 的 AES-256-ECB 解密见
//!   `util::wx_pay_service_impl_utils::decrypt_refund_req_info`；
//! - 对账单/资金账单文本解析见 `util::wx_pay_service_impl_utils`。

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use wx_rust_common::error::WxErrorException;

use crate::api::{
    Apply4SubjectConfirmService, Applyment4SubService, BankService, BrandMerchantTransferService,
    BusinessCircleService, BusinessOperationTransferService, ComplaintService,
    CustomDeclarationService, EcommerceService, EntPayService, MarketingBusiFavorService,
    MarketingFavorService, MarketingMediaService, MerchantLimitationService, MerchantMediaService,
    MerchantTransferService, MiPayService, PartnerPayScoreService, PartnerPayScoreSignPlanService,
    PartnerTransferService, PayScoreService, PayrollService, ProfitSharingService, RealNameService,
    RedpackService, SubscriptionBillingService, TransferService, WxDepositService,
    WxEntrustPapService,
};
use crate::bean::order::{
    WxPayAppOrderResult, WxPayMpOrderResult, WxPayMwebOrderResult, WxPayNativeOrderResult,
};
use crate::bean::result::wx_pay_unified_order_v3_result::{
    AppResult as V3AppResult, JsapiResult as V3JsapiResult,
};
use crate::bean::{
    AppResult, CombineCloseRequest, CombineNotifyResult, CombineQueryResult,
    CombineTransactionsRequest, CombineTransactionsResult, ComplaintNotifyResult,
    GlobalTradeTypeEnum, OriginNotifyResponse, PartnerSubscribeNotifyResult, ReqInfo,
    SignatureHeader, TradeTypeEnum, TransferBillsNotifyResult, WxPayApiData,
    WxPayApplyBillV3Result, WxPayApplyFundFlowBillV3Request, WxPayApplyTradeBillV3Request,
    WxPayAuthcode2OpenidRequest, WxPayAuthcode2OpenidResult, WxPayBillResult, WxPayCodepayRequest,
    WxPayCodepayResult, WxPayCommonResult, WxPayCouponInfoQueryRequest, WxPayCouponInfoQueryResult,
    WxPayCouponSendRequest, WxPayCouponSendResult, WxPayCouponStockQueryRequest,
    WxPayCouponStockQueryResult, WxPayDefaultRequest, WxPayDownloadBillRequest,
    WxPayDownloadFundFlowRequest, WxPayFaceAuthInfoRequest, WxPayFaceAuthInfoResult,
    WxPayFacepayRequest, WxPayFacepayResult, WxPayFundFlowResult, WxPayMicropayRequest,
    WxPayMicropayResult, WxPayNotifyV3Result, WxPayOrderCloseRequest, WxPayOrderCloseResult,
    WxPayOrderCloseV3Request, WxPayOrderNotifyResult, WxPayOrderQueryRequest,
    WxPayOrderQueryResult, WxPayOrderQueryV3Request, WxPayOrderQueryV3Result,
    WxPayOrderReverseRequest, WxPayOrderReverseResult, WxPayOrderReverseV3Request,
    WxPayOrderReverseV3Result, WxPayPartnerNotifyV3Result, WxPayPartnerOrderCloseV3Request,
    WxPayPartnerOrderQueryV3Request, WxPayPartnerOrderQueryV3Result,
    WxPayPartnerRefundNotifyV3Result, WxPayPartnerRefundV3Request,
    WxPayPartnerUnifiedOrderV3Request, WxPayQueryCommentRequest, WxPayQueryExchangeRateRequest,
    WxPayQueryExchangeRateResult, WxPayRefundNotifyResult, WxPayRefundNotifyV3Result,
    WxPayRefundQueryRequest, WxPayRefundQueryResult, WxPayRefundQueryV3Request,
    WxPayRefundQueryV3Result, WxPayRefundRequest, WxPayRefundResult, WxPayRefundV3Request,
    WxPayRefundV3Result, WxPayReportRequest, WxPaySandboxSignKeyResult, WxPayShorturlRequest,
    WxPayShorturlResult, WxPayTransferBatchesNotifyV3Result, WxPayUnifiedOrderRequest,
    WxPayUnifiedOrderResult, WxPayUnifiedOrderV3GlobalRequest, WxPayUnifiedOrderV3Request,
    WxPayUnifiedOrderV3Result, WxScanPayNotifyResult,
};
use crate::config::WxPayConfig;
use crate::constant::wx_pay_constants::{
    WxPaySpecificTradeType, result_code, sign_type as sign_type_const,
    trade_type as trade_type_const,
};
use crate::enums::pay_url;
use crate::util::crypto::wx_pay_v3_crypto_utils::{
    gen_nonce_str, gen_timestamp, sign_sha256_rsa, verify_sha256_rsa,
};
use crate::util::wx_pay_service_impl_utils as impl_utils;
use crate::util::wx_pay_service_impl_utils::V2Request;

/// 构造"未实现"错误（剩余少量无法在本波次实现的方法）。
fn not_implemented(method: &str) -> WxErrorException {
    WxErrorException::from_code(
        -99,
        format!("WxPayService::{method} 未实现（依赖二维码生成库，Wave 2 P2a 保留）"),
    )
}

/// 全局支付方式 → 接口 trade_type 字符串（对应 Java `tradeType.name()`，
/// 枚举常量名为大写）。
fn global_trade_type_str(trade_type: &GlobalTradeTypeEnum) -> &'static str {
    match trade_type {
        GlobalTradeTypeEnum::App => "APP",
        GlobalTradeTypeEnum::Jsapi => "JSAPI",
        GlobalTradeTypeEnum::Native => "NATIVE",
        GlobalTradeTypeEnum::H5 => "H5",
    }
}

/// 全局支付方式 → 境内支付方式（对应 Java `TradeTypeEnum.valueOf(tradeType.name())`）。
fn global_to_domestic(trade_type: &GlobalTradeTypeEnum) -> TradeTypeEnum {
    match trade_type {
        GlobalTradeTypeEnum::App => TradeTypeEnum::App,
        GlobalTradeTypeEnum::Jsapi => TradeTypeEnum::Jsapi,
        GlobalTradeTypeEnum::Native => TradeTypeEnum::Native,
        GlobalTradeTypeEnum::H5 => TradeTypeEnum::H5,
    }
}

/// 校验 v3 通知签名（对应 Java `BaseWxPayServiceImpl#verifyNotifySign`：
/// `WECHATPAY/SIGNTEST/` 探测流量识别 + 平台证书 SHA256withRSA 验签）。
fn verify_notify_sign_with_config(
    config: &dyn WxPayConfig,
    header: &SignatureHeader,
    data: &str,
) -> Result<bool, WxErrorException> {
    crate::util::wx_pay_notify_utils::verify_notify_signature(
        &impl_utils::platform_public_key(config)?,
        header,
        data,
    )
}

/// 通用 v3 通知解析（对应 Java `baseParseOrderNotifyV3Result` 全流程：
/// 验签 → `OriginNotifyResponse` 解析 → `AesUtils.decryptToString` 解密 →
/// 反序列化为 `T`）。
///
/// `ADAPTED`：Java 的 `SignatureHeader` 可为 null（跳过验签），Rust 引用
/// 类型不可空，验签恒执行（非空 header 语义）。
fn parse_notify_v3_typed<T: serde::de::DeserializeOwned>(
    config: &dyn WxPayConfig,
    notify_data: &str,
    header: &SignatureHeader,
) -> Result<(OriginNotifyResponse, T), WxErrorException> {
    // 探测流量识别（对应 Java verifyNotifySign，util 的 parse_notify_v3_result
    // 不含此检查，此处先行）
    if let Some(signature) = header.signature.as_deref() {
        if signature.starts_with("WECHATPAY/SIGNTEST/") {
            return Err(impl_utils::runtime("微信支付签名探测流量"));
        }
    }
    let api_v3_key = config.api_v3_key().unwrap_or_default();
    // 平台证书/公钥衔接（P2b 完成证书自动更新后可换为按 serial 路由的 verifier）
    let public_key = impl_utils::platform_public_key(config)?;
    let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
        notify_data,
        Some(header),
        api_v3_key,
        move |_serial, message, signature| {
            verify_sha256_rsa(&public_key, message, signature).unwrap_or(false)
        },
    )
    .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
    Ok((origin_to_bean(&parsed.raw_data), parsed.result))
}

/// 通知原始报文类型转换（util 结构 → bean 结构，字段一一对应）。
fn origin_to_bean(
    origin: &crate::util::wx_pay_notify_utils::OriginNotifyResponse,
) -> OriginNotifyResponse {
    OriginNotifyResponse {
        id: origin.id.clone(),
        create_time: origin.create_time.clone(),
        event_type: origin.event_type.clone(),
        summary: origin.summary.clone(),
        resource_type: origin.resource_type.clone(),
        resource: origin.resource.as_ref().map(|r| {
            crate::bean::notify::origin_notify_response::Resource {
                algorithm: r.algorithm.clone(),
                original_type: r.original_type.clone(),
                ciphertext: r.ciphertext.clone(),
                associated_data: r.associated_data.clone(),
                nonce: r.nonce.clone(),
            }
        }),
    }
}

/// v3 下单结果组装调起支付参数（对应 Java
/// `WxPayUnifiedOrderV3Result#getPayInfo` / `CombineTransactionsResult#getPayInfo`）：
/// JSAPI → `JsapiResult`（RSA paySign）；H5 → `h5_url` 字符串；APP →
/// `AppResult`；NATIVE → `code_url` 字符串。
fn build_v3_pay_info(
    config: &dyn WxPayConfig,
    trade_type: TradeTypeEnum,
    prepay_id: Option<&str>,
    h5_url: Option<&str>,
    code_url: Option<&str>,
    app_id: &str,
    mch_id: &str,
) -> Result<serde_json::Value, WxErrorException> {
    let private_key = impl_utils::load_merchant_private_key(config)?;
    let timestamp = gen_timestamp().to_string();
    let nonce_str = gen_nonce_str();
    match trade_type {
        TradeTypeEnum::Jsapi => {
            let package_value = format!("prepay_id={}", prepay_id.unwrap_or_default());
            // 对应 Java JsapiResult.getSignStr：appId\n时间戳\nnonceStr\npackage
            let sign_str = format!("{app_id}\n{timestamp}\n{nonce_str}\n{package_value}\n");
            let pay_sign = sign_sha256_rsa(&private_key, sign_str.as_bytes())
                .map_err(|e| impl_utils::runtime(format!("签名计算失败: {e}")))?;
            let result = V3JsapiResult {
                app_id: Some(app_id.to_string()),
                time_stamp: Some(timestamp),
                nonce_str: Some(nonce_str),
                package_value: Some(package_value),
                sign_type: Some("RSA".to_string()),
                pay_sign: Some(pay_sign),
                prepay_id: prepay_id.map(str::to_string),
            };
            Ok(serde_json::to_value(result).map_err(|e| impl_utils::runtime(e.to_string()))?)
        }
        TradeTypeEnum::H5 => Ok(serde_json::Value::String(
            h5_url.unwrap_or_default().to_string(),
        )),
        TradeTypeEnum::App => {
            // 对应 Java AppResult.getSignStr：appid\n时间戳\nnoncestr\nprepayid
            let sign_str = format!(
                "{app_id}\n{timestamp}\n{nonce_str}\n{}\n",
                prepay_id.unwrap_or_default()
            );
            let sign = sign_sha256_rsa(&private_key, sign_str.as_bytes())
                .map_err(|e| impl_utils::runtime(format!("签名计算失败: {e}")))?;
            let result = V3AppResult {
                appid: Some(app_id.to_string()),
                partner_id: Some(mch_id.to_string()),
                prepay_id: prepay_id.map(str::to_string),
                package_value: Some("Sign=WXPay".to_string()),
                noncestr: Some(nonce_str),
                timestamp: Some(timestamp),
                sign: Some(sign),
            };
            Ok(serde_json::to_value(result).map_err(|e| impl_utils::runtime(e.to_string()))?)
        }
        TradeTypeEnum::Native => Ok(serde_json::Value::String(
            code_url.unwrap_or_default().to_string(),
        )),
    }
}

/// 微信支付服务门面。
#[async_trait]
pub trait WxPayService: Send + Sync {
    /// 当前微信支付配置存储（对应 Java `getConfig()` 的直接配置访问）。
    fn wx_pay_config(&self) -> Arc<dyn WxPayConfig>;

    /// HTTP 客户端。
    fn http_client(&self) -> &reqwest::Client;

    // ---- 基础能力（对应 Java getPayBaseUrl/addConfig/removeConfig/...） ----

    /// 获取微信支付请求 url 前缀，沙箱环境可能不一样（对应 Java `getPayBaseUrl()`）。
    ///
    /// 沙箱模式（`useSandboxEnv`）且已配置 apiV3Key 时 Java 抛
    /// `WxRuntimeException("微信支付V3 目前不支持沙箱模式！")`；Rust 侧
    /// 该方法无 Result（Java 无 checked 异常），沙箱校验在请求侧执行。
    fn get_pay_base_url(&self) -> String {
        let config = self.wx_pay_config();
        if config.use_sandbox_env() {
            format!(
                "{}{}",
                config.api_host_with_path_prefix(),
                pay_url::SANDBOX_BASE_URL_SUFFIX
            )
        } else {
            config.api_host_with_path_prefix()
        }
    }

    /// Map 里加入新的配置，键为 `mchId + "_" + appId`
    /// （对应 Java `addConfig(String mchId, String appId, WxPayConfig)`）。
    fn add_config(&self, _mch_id: &str, _app_id: &str, _wx_pay_config: Arc<dyn WxPayConfig>) {}

    /// Map 里加入新的配置，使用自定义配置键（如租户 ID），兼容单参数
    /// switchover 使用方式（对应 Java `addConfig(String configKey, WxPayConfig)`）。
    fn add_config_with_key(&self, _config_key: &str, _wx_pay_config: Arc<dyn WxPayConfig>) {}

    /// 从 Map 中移除 `mchId + "_" + appId` 对应的配置
    /// （对应 Java `removeConfig(String mchId, String appId)`）。
    fn remove_config(&self, _mch_id: &str, _app_id: &str) {}

    /// 从 Map 中移除指定配置键对应的配置
    /// （对应 Java `removeConfig(String configKey)`）。
    fn remove_config_with_key(&self, _config_key: &str) {}

    /// 注入多个配置，并为每个配置赋予不同的 mchId 值
    /// （对应 Java `setMultiConfig(Map<String, WxPayConfig>)`）。
    fn set_multi_config(&self, _wx_pay_configs: &HashMap<String, Arc<dyn WxPayConfig>>) {}

    /// 注入多个配置并指定默认 mchId（对应 Java
    /// `setMultiConfig(Map<String, WxPayConfig>, String defaultMchId)`）。
    fn set_multi_config_with_default(
        &self,
        _wx_pay_configs: &HashMap<String, Arc<dyn WxPayConfig>>,
        _default_mch_id: &str,
    ) {
    }

    /// 进行相应的商户切换（对应 Java `switchover(String mchId, String appId)`）。
    ///
    /// 返回切换是否成功 boolean。
    fn switchover(&self, _mch_id: &str, _app_id: &str) -> bool {
        false
    }

    /// 根据商户号或自定义配置键进行切换：先精确匹配，未找到则前缀匹配
    /// `mchId_*`（对应 Java `default switchover(String mchIdOrConfigKey)`）。
    fn switchover_with_key(&self, _mch_id_or_config_key: &str) -> bool {
        false
    }

    /// 进行相应的商户切换，成功则返回当前对象方便链式调用，否则抛异常
    /// （对应 Java `switchoverTo(String mchId, String appId)`）。
    ///
    /// `ADAPTED`：Java 返回 `WxPayService`（this）实现链式调用，Rust 以
    /// `Result<(), WxErrorException>` 表达（失败语义同 Java 抛
    /// `WxRuntimeException`）。
    async fn switchover_to(&self, _mch_id: &str, _app_id: &str) -> Result<(), WxErrorException> {
        Err(impl_utils::runtime("未找到对应配置"))
    }

    /// 根据商户号或自定义配置键进行切换，支持链式调用
    /// （对应 Java `default switchoverTo(String mchIdOrConfigKey)`）。
    ///
    /// `ADAPTED`：返回值同 `switchover_to`，Java 默认抛
    /// `WxRuntimeException("子类需要实现此方法")`。
    async fn switchover_to_with_key(
        &self,
        _mch_id_or_config_key: &str,
    ) -> Result<(), WxErrorException> {
        Err(impl_utils::runtime("未找到对应配置"))
    }

    /// 获取配置（对应 Java `getConfig()`）。
    ///
    /// 在多商户配置场景下，根据当前切换的配置键获取对应的配置。
    fn get_config(&self) -> Arc<dyn WxPayConfig> {
        self.wx_pay_config()
    }

    /// 根据商户号和 appId 直接获取配置，不依赖切换状态
    /// （对应 Java `getConfig(String mchId, String appId)`）。
    fn get_config_by_mch_app(&self, _mch_id: &str, _app_id: &str) -> Option<Arc<dyn WxPayConfig>> {
        None
    }

    /// 根据商户号直接获取配置（对应 Java `getConfig(String mchId)`）。
    ///
    /// 适用于一个商户号对应多个 appId 的场景，返回该商户号的任意一个配置。
    fn get_config_by_mch(&self, _mch_id: &str) -> Option<Arc<dyn WxPayConfig>> {
        None
    }

    /// 设置配置对象（对应 Java `setConfig(WxPayConfig config)`）。
    fn set_config(&self, _config: Arc<dyn WxPayConfig>) {}

    // ---- HTTP 执行引擎（对应 Java WxPayServiceHttpComponentsImpl） ----

    /// 发送 post 请求，得到响应字节数组（对应 Java
    /// `postForBytes(String url, String requestStr, boolean useKey)`）。
    async fn post_for_bytes(
        &self,
        url: &str,
        request_str: &str,
        use_key: bool,
    ) -> Result<Vec<u8>, WxErrorException> {
        let config = self.wx_pay_config();
        let client = if use_key {
            impl_utils::build_cert_client(config.as_ref())?
        } else {
            self.http_client().clone()
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

    /// 发送 post 请求，得到响应字符串（对应 Java
    /// `post(String url, String requestStr, boolean useKey)`，v2 XML 通道）。
    async fn post(
        &self,
        url: &str,
        request_str: &str,
        use_key: bool,
    ) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_pay_service_impl::execute_post(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            url,
            request_str,
            use_key,
            None,
        )
        .await
    }

    /// 发送 post 请求，得到响应字符串（指定 Content-Type）
    /// （对应 Java `post(String url, String requestStr, boolean useKey, String mimeType)`）。
    async fn post_with_mime_type(
        &self,
        url: &str,
        request_str: &str,
        use_key: bool,
        mime_type: &str,
    ) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_pay_service_impl::execute_post(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            url,
            request_str,
            use_key,
            Some(mime_type),
        )
        .await
    }

    /// 发送 post V3 请求，得到响应字符串（对应 Java `postV3(String url, String requestStr)`）。
    async fn post_v3(&self, url: &str, request_str: &str) -> Result<String, WxErrorException> {
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            "POST",
            url,
            request_str,
        )
        .await
    }

    /// 发送 patch V3 请求，得到响应字符串（对应 Java `patchV3(String url, String requestStr)`）。
    async fn patch_v3(&self, url: &str, request_str: &str) -> Result<String, WxErrorException> {
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            "PATCH",
            url,
            request_str,
        )
        .await
    }

    /// 发送 post 请求，得到响应字符串，请求头包含 `Wechatpay-Serial`
    /// （敏感信息字段场景，对应 Java `postV3WithWechatpaySerial`）。
    ///
    /// Java 两个 postV3 变体都会附加 `Wechatpay-Serial` 头（configureRequest
    /// 统一添加），此处与 `post_v3` 行为一致。
    async fn post_v3_with_wechatpay_serial(
        &self,
        url: &str,
        request_str: &str,
    ) -> Result<String, WxErrorException> {
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            "POST",
            url,
            request_str,
        )
        .await
    }

    /// 发送 post V3 请求（预构建请求对象）
    /// （对应 Java `postV3(String url, HttpPost httpPost)`）。
    ///
    /// `ADAPTED`：Java `HttpPost` 以 `reqwest::Request` 镜像；签名所需的
    /// method/url/body 从请求对象提取后按 v3 通道重建请求（Java
    /// `requestV3` 亦会调用 `configureRequest` 附加统一请求头）。
    async fn post_v3_with_request(
        &self,
        url: &str,
        http_post: &reqwest::Request,
    ) -> Result<String, WxErrorException> {
        let method = http_post.method().as_str().to_string();
        let body = http_post
            .body()
            .and_then(|b| b.as_bytes())
            .map(|b| String::from_utf8_lossy(b).to_string())
            .unwrap_or_default();
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            &method,
            url,
            &body,
        )
        .await
    }

    /// 发送 http 请求，得到响应字符串（可以是 put/post/get/delete 等）
    /// （对应 Java `requestV3(String url, HttpRequestBase httpRequest)`）。
    ///
    /// `ADAPTED`：Java `HttpRequestBase` 以 `reqwest::Request` 镜像，实现
    /// 语义同 [`WxPayService::post_v3_with_request`]。
    async fn request_v3(
        &self,
        url: &str,
        http_request: &reqwest::Request,
    ) -> Result<String, WxErrorException> {
        let method = http_request.method().as_str().to_string();
        let body = http_request
            .body()
            .and_then(|b| b.as_bytes())
            .map(|b| String::from_utf8_lossy(b).to_string())
            .unwrap_or_default();
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            &method,
            url,
            &body,
        )
        .await
    }

    /// 发送 get V3 请求，得到响应字符串（对应 Java `getV3(String url)`）。
    async fn get_v3(&self, url: &str) -> Result<String, WxErrorException> {
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            "GET",
            url,
            "",
        )
        .await
    }

    /// 发送 get 请求，得到响应字符串，请求头包含 `Wechatpay-Serial`
    /// （对应 Java `getV3WithWechatPaySerial(String url)`）。
    ///
    /// Java 的 `requestV3` 统一附加 `Wechatpay-Serial` 头，两个 get 变体
    /// 行为一致（`strictlyNeedWechatPaySerial=false` 时 Java 亦如此）。
    async fn get_v3_with_wechat_pay_serial(&self, url: &str) -> Result<String, WxErrorException> {
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            "GET",
            url,
            "",
        )
        .await
    }

    /// 发送下载 V3 请求，得到响应流（对应 Java `downloadV3(String url)`）。
    ///
    /// `ADAPTED`：Java `InputStream` 以 `Vec<u8>` 镜像。
    async fn download_v3(&self, url: &str) -> Result<Vec<u8>, WxErrorException> {
        impl_utils::download_v3(self.wx_pay_config().as_ref(), self.http_client(), url).await
    }

    /// 发送 put V3 请求，得到响应字符串（对应 Java `putV3(String url, String requestStr)`）。
    async fn put_v3(&self, url: &str, request_str: &str) -> Result<String, WxErrorException> {
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            "PUT",
            url,
            request_str,
        )
        .await
    }

    /// 发送 delete V3 请求，得到响应字符串（对应 Java `deleteV3(String url)`）。
    async fn delete_v3(&self, url: &str) -> Result<String, WxErrorException> {
        impl_utils::execute_v3(
            self.wx_pay_config().as_ref(),
            self.http_client(),
            "DELETE",
            url,
            "",
        )
        .await
    }

    // ---- 子服务（对应 Java WxPayService 的 `getXxxService()`；默认返回
    // None，由 WxPayServiceImpl 覆写为装配后的实例） ----

    /// 微信签约代扣服务（对应 Java `getWxEntrustPapService`）。
    fn wx_entrust_pap_service(&self) -> Option<Arc<dyn WxEntrustPapService>> {
        None
    }

    /// 微信押金支付服务（对应 Java `getWxDepositService`）。
    fn wx_deposit_service(&self) -> Option<Arc<dyn WxDepositService>> {
        None
    }

    /// 批量转账到零钱服务（对应 Java `getPartnerTransferService`）。
    fn partner_transfer_service(&self) -> Option<Arc<dyn PartnerTransferService>> {
        None
    }

    /// 微工卡服务（对应 Java `getPayrollService`）。
    fn payroll_service(&self) -> Option<Arc<dyn PayrollService>> {
        None
    }

    /// 企业付款服务（对应 Java `getEntPayService`）。
    fn ent_pay_service(&self) -> Option<Arc<dyn EntPayService>> {
        None
    }

    /// 设置企业付款服务类，允许开发者自定义实现类（对应 Java `setEntPayService`）。
    fn set_ent_pay_service(&self, _ent_pay_service: Arc<dyn EntPayService>) {}

    /// 红包接口服务（对应 Java `getRedpackService`）。
    fn redpack_service(&self) -> Option<Arc<dyn RedpackService>> {
        None
    }

    /// 分账服务（对应 Java `getProfitSharingService`）。
    fn profit_sharing_service(&self) -> Option<Arc<dyn ProfitSharingService>> {
        None
    }

    /// 支付分服务（对应 Java `getPayScoreService`）。
    fn pay_score_service(&self) -> Option<Arc<dyn PayScoreService>> {
        None
    }

    /// 电商收付通服务（对应 Java `getEcommerceService`）。
    fn ecommerce_service(&self) -> Option<Arc<dyn EcommerceService>> {
        None
    }

    /// 微信支付智慧商圈服务（对应 Java `getBusinessCircleService`）。
    fn business_circle_service(&self) -> Option<Arc<dyn BusinessCircleService>> {
        None
    }

    /// 微信支付通用媒体服务（对应 Java `getMerchantMediaService`）。
    fn merchant_media_service(&self) -> Option<Arc<dyn MerchantMediaService>> {
        None
    }

    /// 微信支付营销媒体服务（对应 Java `getMarketingMediaService`）。
    fn marketing_media_service(&self) -> Option<Arc<dyn MarketingMediaService>> {
        None
    }

    /// 微信支付营销代金券服务（对应 Java `getMarketingFavorService`）。
    fn marketing_favor_service(&self) -> Option<Arc<dyn MarketingFavorService>> {
        None
    }

    /// 微信支付营销商家券服务（对应 Java `getMarketingBusiFavorService`）。
    fn marketing_busi_favor_service(&self) -> Option<Arc<dyn MarketingBusiFavorService>> {
        None
    }

    /// 商家转账到零钱服务（对应 Java `getMerchantTransferService`）。
    fn merchant_transfer_service(&self) -> Option<Arc<dyn MerchantTransferService>> {
        None
    }

    /// 品牌红包商家转账到零钱服务（对应 Java `getBrandMerchantTransferService`）。
    fn brand_merchant_transfer_service(&self) -> Option<Arc<dyn BrandMerchantTransferService>> {
        None
    }

    /// 微信支付预约扣费服务（连续包月功能，对应 Java `getSubscriptionBillingService`）。
    fn subscription_billing_service(&self) -> Option<Arc<dyn SubscriptionBillingService>> {
        None
    }

    /// 商户被管控能力及原因查询服务（对应 Java `getMerchantLimitationService`）。
    fn merchant_limitation_service(&self) -> Option<Arc<dyn MerchantLimitationService>> {
        None
    }

    /// 消费者投诉服务（对应 Java `getComplaintsService`）。
    fn complaints_service(&self) -> Option<Arc<dyn ComplaintService>> {
        None
    }

    /// 银行服务（对应 Java `getBankService`）。
    fn bank_service(&self) -> Option<Arc<dyn BankService>> {
        None
    }

    /// 商家转账服务（对应 Java `getTransferService`）。
    fn transfer_service(&self) -> Option<Arc<dyn TransferService>> {
        None
    }

    /// 商家转账运营工具服务（对应 Java `getBusinessOperationTransferService`）。
    fn business_operation_transfer_service(
        &self,
    ) -> Option<Arc<dyn BusinessOperationTransferService>> {
        None
    }

    /// 服务商支付分服务（对应 Java `getPartnerPayScoreService`）。
    fn partner_pay_score_service(&self) -> Option<Arc<dyn PartnerPayScoreService>> {
        None
    }

    /// 服务商支付分签约计划服务（对应 Java `getPartnerPayScoreSignPlanService`）。
    fn partner_pay_score_sign_plan_service(
        &self,
    ) -> Option<Arc<dyn PartnerPayScoreSignPlanService>> {
        None
    }

    /// 实名服务（对应 Java `getRealNameService`）。
    fn real_name_service(&self) -> Option<Arc<dyn RealNameService>> {
        None
    }

    /// MiPay 服务（对应 Java `getMiPayService`）。
    fn mi_pay_service(&self) -> Option<Arc<dyn MiPayService>> {
        None
    }

    /// 商户开户意愿确认服务（对应 Java `getApply4SubjectConfirmService`）。
    fn apply4_subject_confirm_service(&self) -> Option<Arc<dyn Apply4SubjectConfirmService>> {
        None
    }

    /// 特约商户进件服务（对应 Java `getApplyment4SubService`）。
    fn applyment4_sub_service(&self) -> Option<Arc<dyn Applyment4SubService>> {
        None
    }

    /// 海关报关服务（对应 Java `getCustomDeclarationService`）。
    fn custom_declaration_service(&self) -> Option<Arc<dyn CustomDeclarationService>> {
        None
    }

    // ---- 订单域（对应 Java WxPayService 的查询/关闭/下单方法） ----

    /// 查询订单（对应 Java `queryOrder(String transactionId, String outTradeNo)`，
    /// 接口地址 `/pay/orderquery`）。
    ///
    /// 微信订单号与商户订单号二选一（未提供的传 `None`）。
    async fn query_order(
        &self,
        transaction_id: Option<&str>,
        out_trade_no: Option<&str>,
    ) -> Result<WxPayOrderQueryResult, WxErrorException> {
        let mut request = WxPayOrderQueryRequest::default();
        request.out_trade_no = out_trade_no
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        request.transaction_id = transaction_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        self.query_order_with_request(&request).await
    }

    /// 查询订单（对应 Java `queryOrder(WxPayOrderQueryRequest request)`，
    /// 适合需要自定义子商户号和子商户 appid 的情形）。
    async fn query_order_with_request(
        &self,
        request: &WxPayOrderQueryRequest,
    ) -> Result<WxPayOrderQueryResult, WxErrorException> {
        // 对应 Java checkConstraints：transaction_id 与 out_trade_no 必须二选一
        let has_tid = request
            .transaction_id
            .as_deref()
            .map(str::trim)
            .map(|s| !s.is_empty())
            .unwrap_or(false);
        let has_out = request
            .out_trade_no
            .as_deref()
            .map(str::trim)
            .map(|s| !s.is_empty())
            .unwrap_or(false);
        if has_tid == has_out {
            return Err(impl_utils::runtime(
                "transaction_id 和 out_trade_no 不能同时存在或同时为空，必须二选一",
            ));
        }
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::ORDER_QUERY_URL);
        let response = self.post(&url, &xml, false).await?;
        if response.trim().is_empty() {
            // 对应 Java：throw new WxPayException("无响应结果")
            return Err(impl_utils::runtime("无响应结果"));
        }
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayOrderQueryResult::from_xml,
        )
    }

    /// 查询订单 v3（对应 Java
    /// `queryOrderV3(String transactionId, String outTradeNo)`，接口地址
    /// `/v3/pay/transactions/id/{transaction_id}` 或
    /// `/v3/pay/transactions/out-trade-no/{out_trade_no}`）。
    async fn query_order_v3(
        &self,
        transaction_id: Option<&str>,
        out_trade_no: Option<&str>,
    ) -> Result<WxPayOrderQueryV3Result, WxErrorException> {
        let mut request = WxPayOrderQueryV3Request::default();
        request.out_trade_no = out_trade_no
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        request.transaction_id = transaction_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        self.query_order_v3_with_request(&request).await
    }

    /// 查询订单 v3（对应 Java `queryOrderV3(WxPayOrderQueryV3Request request)`）。
    async fn query_order_v3_with_request(
        &self,
        request: &WxPayOrderQueryV3Request,
    ) -> Result<WxPayOrderQueryV3Result, WxErrorException> {
        let mut request = request.clone();
        if request
            .mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.mchid = self.wx_pay_config().mch_id().map(str::to_string);
        }
        let url = match request.out_trade_no.as_deref() {
            Some(out_no) => format!(
                "{}/v3/pay/transactions/out-trade-no/{}",
                self.get_pay_base_url(),
                out_no
            ),
            None => format!(
                "{}/v3/pay/transactions/id/{}",
                self.get_pay_base_url(),
                request.transaction_id.as_deref().unwrap_or_default()
            ),
        };
        let query = format!("?mchid={}", request.mchid.as_deref().unwrap_or_default());
        let response = self
            .get_v3_with_wechat_pay_serial(&format!("{url}{query}"))
            .await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 服务商模式查询订单 v3（对应 Java
    /// `queryPartnerOrderV3(String transactionId, String outTradeNo)`）。
    async fn query_partner_order_v3(
        &self,
        transaction_id: Option<&str>,
        out_trade_no: Option<&str>,
    ) -> Result<WxPayPartnerOrderQueryV3Result, WxErrorException> {
        let mut request = WxPayPartnerOrderQueryV3Request::default();
        request.out_trade_no = out_trade_no
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        request.transaction_id = transaction_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        self.query_partner_order_v3_with_request(&request).await
    }

    /// 服务商模式查询订单 v3（对应 Java
    /// `queryPartnerOrderV3(WxPayPartnerOrderQueryV3Request request)`）。
    async fn query_partner_order_v3_with_request(
        &self,
        request: &WxPayPartnerOrderQueryV3Request,
    ) -> Result<WxPayPartnerOrderQueryV3Result, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .sp_mch_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sp_mch_id = config.mch_id().map(str::to_string);
        }
        if request
            .sub_mch_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sub_mch_id = config.sub_mch_id().map(str::to_string);
        }
        let url = match request.out_trade_no.as_deref() {
            Some(out_no) => format!(
                "{}/v3/pay/partner/transactions/out-trade-no/{}",
                self.get_pay_base_url(),
                out_no
            ),
            None => format!(
                "{}/v3/pay/partner/transactions/id/{}",
                self.get_pay_base_url(),
                request.transaction_id.as_deref().unwrap_or_default()
            ),
        };
        let query = format!(
            "?sp_mchid={}&sub_mchid={}",
            request.sp_mch_id.as_deref().unwrap_or_default(),
            request.sub_mch_id.as_deref().unwrap_or_default()
        );
        let response = self
            .get_v3_with_wechat_pay_serial(&format!("{url}{query}"))
            .await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 合单查询订单（对应 Java `queryCombine(String combineOutTradeNo)`，
    /// 请求 URL `/v3/combine-transactions/out-trade-no/{combine_out_trade_no}`）。
    async fn query_combine(
        &self,
        combine_out_trade_no: &str,
    ) -> Result<CombineQueryResult, WxErrorException> {
        let url = format!(
            "{}/v3/combine-transactions/out-trade-no/{}",
            self.get_pay_base_url(),
            combine_out_trade_no
        );
        let response = self.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 关闭订单（对应 Java `closeOrder(String outTradeNo)`，接口地址
    /// `/pay/closeorder`，无需证书）。
    async fn close_order(
        &self,
        out_trade_no: &str,
    ) -> Result<WxPayOrderCloseResult, WxErrorException> {
        if out_trade_no.trim().is_empty() {
            return Err(impl_utils::runtime("out_trade_no不能为空"));
        }
        let mut request = WxPayOrderCloseRequest::default();
        request.out_trade_no = Some(out_trade_no.trim().to_string());
        self.close_order_with_request(&request).await
    }

    /// 关闭订单（对应 Java `closeOrder(WxPayOrderCloseRequest request)`，
    /// 适合需要自定义子商户号和子商户 appid 的情形）。
    async fn close_order_with_request(
        &self,
        request: &WxPayOrderCloseRequest,
    ) -> Result<WxPayOrderCloseResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::CLOSE_ORDER_URL);
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayOrderCloseResult::from_xml,
        )
    }

    /// 关闭订单 v3（对应 Java `closeOrderV3(String outTradeNo)`，接口地址
    /// `/v3/pay/transactions/out-trade-no/{out_trade_no}/close`）。
    async fn close_order_v3(&self, out_trade_no: &str) -> Result<(), WxErrorException> {
        if out_trade_no.trim().is_empty() {
            return Err(impl_utils::runtime("out_trade_no不能为空"));
        }
        let mut request = WxPayOrderCloseV3Request::default();
        if let Some(mchid) = self.wx_pay_config().mch_id() {
            request.mchid = Some(mchid.to_string());
        }
        let url = format!(
            "{}/v3/pay/transactions/out-trade-no/{}/close",
            self.get_pay_base_url(),
            out_trade_no.trim()
        );
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        self.post_v3_with_wechatpay_serial(&url, &body)
            .await
            .map(|_| ())
    }

    /// 服务商关闭订单 v3（对应 Java `closePartnerOrderV3(String outTradeNo)`）。
    async fn close_partner_order_v3(&self, out_trade_no: &str) -> Result<(), WxErrorException> {
        if out_trade_no.trim().is_empty() {
            return Err(impl_utils::runtime("out_trade_no不能为空"));
        }
        let config = self.wx_pay_config();
        let mut request = WxPayPartnerOrderCloseV3Request::default();
        if let Some(mch_id) = config.mch_id() {
            request.sp_mch_id = Some(mch_id.to_string());
        }
        if let Some(sub_mch_id) = config.sub_mch_id() {
            request.sub_mch_id = Some(sub_mch_id.to_string());
        }
        let url = format!(
            "{}/v3/pay/partner/transactions/out-trade-no/{}/close",
            self.get_pay_base_url(),
            out_trade_no.trim()
        );
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        self.post_v3_with_wechatpay_serial(&url, &body)
            .await
            .map(|_| ())
    }

    /// 关闭订单 v3（对应 Java `closeOrderV3(WxPayOrderCloseV3Request request)`，
    /// 请求 URL `/v3/pay/transactions/out-trade-no/{out_trade_no}/close`）。
    ///
    /// Wave 2 修复：`out_trade_no`（Java transient 字段，`#[serde(skip)]`）
    /// 已补回请求 bean，URL 从请求构造；请求体仅含 `mchid`（Java
    /// `GSON.toJson` 跳过 transient，线格式一致）。
    async fn close_order_v3_with_request(
        &self,
        request: &WxPayOrderCloseV3Request,
    ) -> Result<(), WxErrorException> {
        let mut request = request.clone();
        // 对应 Java `StringUtils.isBlank(mchid)` → 从配置回填
        if request
            .mchid
            .as_deref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            request.mchid = self.wx_pay_config().mch_id().map(str::to_string);
        }
        let url = format!(
            "{}/v3/pay/transactions/out-trade-no/{}/close",
            self.get_pay_base_url(),
            request.out_trade_no.as_deref().unwrap_or_default()
        );
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        self.post_v3_with_wechatpay_serial(&url, &body)
            .await
            .map(|_| ())
    }

    /// 服务商关闭订单 v3（对应 Java
    /// `closePartnerOrderV3(WxPayPartnerOrderCloseV3Request request)`，
    /// 请求 URL `/v3/pay/partner/transactions/out-trade-no/{out_trade_no}/close`）。
    ///
    /// Wave 2 修复：`out_trade_no` 已补回请求 bean；请求体仅含
    /// `sp_mchid`/`sub_mchid`（Java Gson 跳过 transient）。
    async fn close_partner_order_v3_with_request(
        &self,
        request: &WxPayPartnerOrderCloseV3Request,
    ) -> Result<(), WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java `StringUtils.isBlank(spMchId)` → 配置回填
        if request
            .sp_mch_id
            .as_deref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            request.sp_mch_id = config.mch_id().map(str::to_string);
        }
        // 对应 Java `StringUtils.isBlank(subMchId)` → 配置回填
        if request
            .sub_mch_id
            .as_deref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            request.sub_mch_id = config.sub_mch_id().map(str::to_string);
        }
        let url = format!(
            "{}/v3/pay/partner/transactions/out-trade-no/{}/close",
            self.get_pay_base_url(),
            request.out_trade_no.as_deref().unwrap_or_default()
        );
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        self.post_v3_with_wechatpay_serial(&url, &body)
            .await
            .map(|_| ())
    }

    /// 合单关闭订单（对应 Java `closeCombine(CombineCloseRequest request)`，
    /// 请求 URL `/v3/combine-transactions/out-trade-no/{combine_out_trade_no}/close`）。
    ///
    /// Wave 2 修复：`combine_out_trade_no`（Java transient 字段）已补回
    /// 请求 bean；请求体仅含 `combine_appid`/`sub_orders`（Java Gson 跳过
    /// transient，线格式一致）。
    async fn close_combine(&self, request: &CombineCloseRequest) -> Result<(), WxErrorException> {
        let url = format!(
            "{}/v3/combine-transactions/out-trade-no/{}/close",
            self.get_pay_base_url(),
            request.combine_out_trade_no.as_deref().unwrap_or_default()
        );
        let body = serde_json::to_string(request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        self.post_v3_with_wechatpay_serial(&url, &body)
            .await
            .map(|_| ())
    }

    /// 调用统一下单接口，并组装生成支付所需参数对象
    /// （对应 Java `<T> T createOrder(WxPayUnifiedOrderRequest request)`）。
    ///
    /// `ADAPTED`：Java 泛型返回值（`bean/order` 下的 `WxPayMpOrderResult` 等）
    /// 以 `serde_json::Value` 类型擦除，按交易类型分别序列化为
    /// `WxPayMwebOrderResult`/`WxPayNativeOrderResult`/`WxPayAppOrderResult`/
    /// `WxPayMpOrderResult`。
    async fn create_order(
        &self,
        request: &WxPayUnifiedOrderRequest,
    ) -> Result<serde_json::Value, WxErrorException> {
        let unified_order_result = self.unified_order(request).await?;
        let prepay_id = unified_order_result
            .prepay_id
            .as_deref()
            .unwrap_or_default();
        if prepay_id.is_empty() {
            return Err(impl_utils::runtime(format!(
                "无法获取prepay id，错误代码： '{}'，信息：{}。",
                unified_order_result.err_code.as_deref().unwrap_or_default(),
                unified_order_result
                    .err_code_des
                    .as_deref()
                    .unwrap_or_default()
            )));
        }
        let timestamp = gen_timestamp().to_string();
        let nonce_str = unified_order_result.nonce_str.clone().unwrap_or_default();
        let config = self.wx_pay_config();
        let trade_type = request.trade_type.as_deref().unwrap_or_default();
        match trade_type {
            trade_type_const::MWEB => {
                let result = WxPayMwebOrderResult {
                    mweb_url: unified_order_result.mweb_url.clone(),
                };
                Ok(serde_json::to_value(result).map_err(|e| impl_utils::runtime(e.to_string()))?)
            }
            trade_type_const::NATIVE => {
                let result = WxPayNativeOrderResult {
                    code_url: unified_order_result.code_url.clone(),
                };
                Ok(serde_json::to_value(result).map_err(|e| impl_utils::runtime(e.to_string()))?)
            }
            trade_type_const::APP => {
                // APP支付绑定的是微信开放平台上的账号，APPID为开放平台上绑定APP后发放的参数
                let mut app_id = unified_order_result.appid.clone();
                if unified_order_result
                    .sub_app_id
                    .as_deref()
                    .map(|s| !s.is_empty())
                    .unwrap_or(false)
                {
                    app_id = unified_order_result.sub_app_id.clone();
                }
                let mut partner_id = unified_order_result.mch_id.clone();
                if unified_order_result
                    .sub_mch_id
                    .as_deref()
                    .map(|s| !s.is_empty())
                    .unwrap_or(false)
                {
                    partner_id = unified_order_result.sub_mch_id.clone();
                }
                // 此map用于参与调起sdk支付的二次签名,格式全小写，timestamp只能是10位
                let mut config_map = HashMap::new();
                config_map.insert("prepayid".to_string(), prepay_id.to_string());
                config_map.insert(
                    "partnerid".to_string(),
                    partner_id.clone().unwrap_or_default(),
                );
                let package_value = "Sign=WXPay";
                config_map.insert("package".to_string(), package_value.to_string());
                config_map.insert("timestamp".to_string(), timestamp.clone());
                config_map.insert("noncestr".to_string(), nonce_str.clone());
                config_map.insert("appid".to_string(), app_id.clone().unwrap_or_default());
                let sign =
                    sign_utils_create(config_map, request.sign_type.as_deref(), config.as_ref())?;
                let result = WxPayAppOrderResult {
                    sign: Some(sign),
                    prepay_id: Some(prepay_id.to_string()),
                    partner_id,
                    app_id,
                    package_value: Some(package_value.to_string()),
                    time_stamp: Some(timestamp),
                    nonce_str: Some(nonce_str),
                };
                Ok(serde_json::to_value(result).map_err(|e| impl_utils::runtime(e.to_string()))?)
            }
            trade_type_const::JSAPI => {
                let sign_type = match request.sign_type.as_deref() {
                    Some(st) => st.to_string(),
                    None => sign_type_const::MD5.to_string(),
                };
                let mut appid = unified_order_result.appid.clone();
                if unified_order_result
                    .sub_app_id
                    .as_deref()
                    .map(|s| !s.is_empty())
                    .unwrap_or(false)
                {
                    appid = unified_order_result.sub_app_id.clone();
                }
                // 对应 Java：WxPayMpOrderResult 各字段（appId/timeStamp/nonceStr/
                // package/signType）参与二次签名
                let mut sign_map = HashMap::new();
                sign_map.insert("appId".to_string(), appid.clone().unwrap_or_default());
                sign_map.insert("timeStamp".to_string(), timestamp.clone());
                sign_map.insert("nonceStr".to_string(), nonce_str.clone());
                sign_map.insert("package".to_string(), format!("prepay_id={prepay_id}"));
                sign_map.insert("signType".to_string(), sign_type.clone());
                let pay_sign = sign_utils_create(sign_map, Some(&sign_type), config.as_ref())?;
                let result = WxPayMpOrderResult {
                    app_id: appid,
                    time_stamp: Some(timestamp),
                    nonce_str: Some(nonce_str),
                    package_value: Some(format!("prepay_id={prepay_id}")),
                    sign_type: Some(sign_type),
                    pay_sign: Some(pay_sign),
                };
                Ok(serde_json::to_value(result).map_err(|e| impl_utils::runtime(e.to_string()))?)
            }
            _ => Err(impl_utils::runtime("该交易类型暂不支持")),
        }
    }

    /// 调用统一下单接口，按指定交易方式组装生成支付所需参数对象
    /// （对应 Java
    /// `<T> T createOrder(WxPayConstants.TradeType.Specific<T> specificTradeType,
    /// WxPayUnifiedOrderRequest request)`；request 的 tradeType 与配置的
    /// tradeType 将被忽略，转而使用 specificTradeType）。
    async fn create_order_with_specific(
        &self,
        specific_trade_type: WxPaySpecificTradeType,
        request: &WxPayUnifiedOrderRequest,
    ) -> Result<serde_json::Value, WxErrorException> {
        let mut request = request.clone();
        request.trade_type = Some(specific_trade_type.type_str().to_string());
        self.create_order(&request).await
    }

    /// 统一下单（对应 Java `unifiedOrder(WxPayUnifiedOrderRequest request)`，
    /// 接口地址 `/pay/unifiedorder`；appid、mchid 等参数自动从配置获取）。
    async fn unified_order(
        &self,
        request: &WxPayUnifiedOrderRequest,
    ) -> Result<WxPayUnifiedOrderResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java checkAndSign 覆写：notify_url/trade_type 从配置补齐
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.notify_url().map(str::to_string);
        }
        if request
            .trade_type
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.trade_type = config.trade_type().map(str::to_string);
        }
        // 对应 Java checkConstraints：NATIVE 必须指定 product_id
        if request.trade_type.as_deref() == Some(trade_type_const::NATIVE)
            && request
                .product_id
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
        {
            return Err(impl_utils::runtime(
                "当trade_type是'NATIVE'时，需指定非空的product_id值",
            ));
        }
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::UNIFIED_ORDER_URL);
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayUnifiedOrderResult::from_xml,
        )
    }

    /// v3 统一下单并组装生成支付所需参数对象（对应 Java
    /// `<T> T createOrderV3(TradeTypeEnum tradeType, WxPayUnifiedOrderV3Request request)`）。
    ///
    /// `ADAPTED`：泛型返回值以 `serde_json::Value` 类型擦除。
    async fn create_order_v3(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayUnifiedOrderV3Request,
    ) -> Result<serde_json::Value, WxErrorException> {
        // 对应 Java：unifiedOrderV3 会回填 appid/mchid，createOrderV3 用回填后的值
        let mut filled = request.clone();
        let config = self.wx_pay_config();
        if filled
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            filled.appid = config.app_id().map(str::to_string);
        }
        if filled
            .mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            filled.mchid = config.mch_id().map(str::to_string);
        }
        let result = self.unified_order_v3(trade_type, &filled).await?;
        build_v3_pay_info(
            config.as_ref(),
            trade_type,
            result.prepay_id.as_deref(),
            result.h5_url.as_deref(),
            result.code_url.as_deref(),
            filled.appid.as_deref().unwrap_or_default(),
            filled.mchid.as_deref().unwrap_or_default(),
        )
    }

    /// 服务商模式 v3 统一下单并组装生成支付所需参数对象（对应 Java
    /// `<T> T createPartnerOrderV3(TradeTypeEnum tradeType,
    /// WxPayPartnerUnifiedOrderV3Request request)`）。
    ///
    /// `ADAPTED`：泛型返回值以 `serde_json::Value` 类型擦除。
    async fn create_partner_order_v3(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayPartnerUnifiedOrderV3Request,
    ) -> Result<serde_json::Value, WxErrorException> {
        let config = self.wx_pay_config();
        let mut filled = request.clone();
        if filled
            .sp_appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            filled.sp_appid = config.app_id().map(str::to_string);
        }
        if filled
            .sp_mch_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            filled.sp_mch_id = config.mch_id().map(str::to_string);
        }
        if filled
            .sub_mch_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            filled.sub_mch_id = config.sub_mch_id().map(str::to_string);
        }
        let result = self.unified_partner_order_v3(trade_type, &filled).await?;
        // 获取应用ID（对应 Java：sub_appid 为空时用 sp_appid）
        let app_id = match filled.sub_appid.as_deref() {
            Some(s) if !s.trim().is_empty() => s.trim().to_string(),
            _ => filled.sp_appid.as_deref().unwrap_or_default().to_string(),
        };
        build_v3_pay_info(
            config.as_ref(),
            trade_type,
            result.prepay_id.as_deref(),
            result.h5_url.as_deref(),
            result.code_url.as_deref(),
            &app_id,
            filled.sub_mch_id.as_deref().unwrap_or_default(),
        )
    }

    /// 境外微信支付 v3 统一下单并组装生成支付所需参数对象（对应 Java
    /// `<T> T createOrderV3Global(GlobalTradeTypeEnum tradeType,
    /// WxPayUnifiedOrderV3GlobalRequest request)`）。
    ///
    /// `ADAPTED`：泛型返回值以 `serde_json::Value` 类型擦除。
    async fn create_order_v3_global(
        &self,
        trade_type: GlobalTradeTypeEnum,
        request: &WxPayUnifiedOrderV3GlobalRequest,
    ) -> Result<serde_json::Value, WxErrorException> {
        let config = self.wx_pay_config();
        let mut filled = request.clone();
        if filled
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            filled.appid = config.app_id().map(str::to_string);
        }
        if filled
            .mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            filled.mchid = config.mch_id().map(str::to_string);
        }
        let result = self.unified_order_v3_global(trade_type, &filled).await?;
        build_v3_pay_info(
            config.as_ref(),
            global_to_domestic(&trade_type),
            result.prepay_id.as_deref(),
            result.h5_url.as_deref(),
            result.code_url.as_deref(),
            filled.appid.as_deref().unwrap_or_default(),
            filled.mchid.as_deref().unwrap_or_default(),
        )
    }

    /// 服务商模式 v3 统一下单（对应 Java
    /// `unifiedPartnerOrderV3(TradeTypeEnum tradeType,
    /// WxPayPartnerUnifiedOrderV3Request request)`）。
    async fn unified_partner_order_v3(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayPartnerUnifiedOrderV3Request,
    ) -> Result<WxPayUnifiedOrderV3Result, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .sp_appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sp_appid = config.app_id().map(str::to_string);
        }
        if request
            .sp_mch_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sp_mch_id = config.mch_id().map(str::to_string);
        }
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.notify_url().map(str::to_string);
        }
        if request
            .sub_appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sub_appid = config.sub_app_id().map(str::to_string);
        }
        if request
            .sub_mch_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sub_mch_id = config.sub_mch_id().map(str::to_string);
        }
        let url = format!("{}{}", self.get_pay_base_url(), trade_type.partner_url());
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// v3 统一下单（对应 Java
    /// `unifiedOrderV3(TradeTypeEnum tradeType, WxPayUnifiedOrderV3Request request)`）。
    async fn unified_order_v3(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayUnifiedOrderV3Request,
    ) -> Result<WxPayUnifiedOrderV3Result, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        if request
            .mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.mchid = config.mch_id().map(str::to_string);
        }
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.notify_url().map(str::to_string);
        }
        let url = format!("{}{}", self.get_pay_base_url(), trade_type.merchant_url());
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 境外微信支付 v3 统一下单（对应 Java
    /// `unifiedOrderV3Global(GlobalTradeTypeEnum tradeType,
    /// WxPayUnifiedOrderV3GlobalRequest request)`）。
    async fn unified_order_v3_global(
        &self,
        trade_type: GlobalTradeTypeEnum,
        request: &WxPayUnifiedOrderV3GlobalRequest,
    ) -> Result<WxPayUnifiedOrderV3Result, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        if request
            .mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.mchid = config.mch_id().map(str::to_string);
        }
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.notify_url().map(str::to_string);
        }
        if request
            .trade_type
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.trade_type = Some(global_trade_type_str(&trade_type).to_string());
        }
        // 境外支付使用独立基地址（对应 Java globalBaseUrl）
        let url = format!("https://apihk.mch.weixin.qq.com{}", trade_type.url());
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 合单支付 API（对应 Java `combine(TradeTypeEnum tradeType,
    /// CombineTransactionsRequest request)`，覆盖 APP/JSAPI/H5/NATIVE）。
    async fn combine(
        &self,
        trade_type: TradeTypeEnum,
        request: &CombineTransactionsRequest,
    ) -> Result<CombineTransactionsResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .combine_appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.combine_appid = config.app_id().map(str::to_string);
        }
        if request
            .combine_mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.combine_mchid = config.mch_id().map(str::to_string);
        }
        let url = format!("{}{}", self.get_pay_base_url(), trade_type.combine_url());
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 合单支付并组装调起支付参数（对应 Java
    /// `<T> T combineTransactions(TradeTypeEnum tradeType,
    /// CombineTransactionsRequest request)`）。
    ///
    /// `ADAPTED`：泛型返回值以 `serde_json::Value` 类型擦除；Java
    /// `CombineTransactionsResult.getPayInfo` 中 H5 分支取 `h5_url`，Rust
    /// bean 缺失该字段（生成器缺口），此处从原始响应 JSON 提取。
    async fn combine_transactions(
        &self,
        trade_type: TradeTypeEnum,
        request: &CombineTransactionsRequest,
    ) -> Result<serde_json::Value, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .combine_appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.combine_appid = config.app_id().map(str::to_string);
        }
        if request
            .combine_mchid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.combine_mchid = config.mch_id().map(str::to_string);
        }
        let url = format!("{}{}", self.get_pay_base_url(), trade_type.combine_url());
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        let raw: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))?;
        let result: CombineTransactionsResult = serde_json::from_value(raw.clone())
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))?;
        match trade_type {
            TradeTypeEnum::H5 => {
                // bean 缺 h5_url 字段，从原始 JSON 取值（Java getPayInfo 返回 h5Url）
                Ok(raw
                    .get("h5_url")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null))
            }
            TradeTypeEnum::Jsapi => build_v3_pay_info(
                config.as_ref(),
                trade_type,
                result.prepay_id.as_deref(),
                None,
                result.code_url.as_deref(),
                request.combine_appid.as_deref().unwrap_or_default(),
                request.combine_mchid.as_deref().unwrap_or_default(),
            ),
            TradeTypeEnum::App => {
                // 对应 Java CombineTransactionsResult.getPayInfo：APP 不二次签名
                let mut app_result = AppResult::default();
                app_result.appid = request.combine_appid.clone();
                app_result.prepayid = result.prepay_id.clone();
                app_result.partnerid = request.combine_mchid.clone();
                app_result.noncestr = Some(gen_nonce_str());
                app_result.timestamp = Some(gen_timestamp().to_string());
                app_result.package_value = Some("Sign=WXPay".to_string());
                Ok(serde_json::to_value(app_result)
                    .map_err(|e| impl_utils::runtime(e.to_string()))?)
            }
            TradeTypeEnum::Native => build_v3_pay_info(
                config.as_ref(),
                trade_type,
                result.prepay_id.as_deref(),
                None,
                result.code_url.as_deref(),
                request.combine_appid.as_deref().unwrap_or_default(),
                request.combine_mchid.as_deref().unwrap_or_default(),
            ),
        }
    }

    /// 该接口调用"统一下单"接口，并拼装发起支付请求需要的参数
    /// （对应 Java `getPayInfo(WxPayUnifiedOrderRequest request)`，已废弃，
    /// 建议使用 `create_order`）。
    async fn get_pay_info(
        &self,
        request: &WxPayUnifiedOrderRequest,
    ) -> Result<HashMap<String, String>, WxErrorException> {
        let unified_order_result = self.unified_order(request).await?;
        let prepay_id = unified_order_result
            .prepay_id
            .as_deref()
            .unwrap_or_default();
        if prepay_id.is_empty() {
            return Err(impl_utils::runtime(format!(
                "无法获取prepay id，错误代码： '{}'，信息：{}。",
                unified_order_result.err_code.as_deref().unwrap_or_default(),
                unified_order_result
                    .err_code_des
                    .as_deref()
                    .unwrap_or_default()
            )));
        }
        let config = self.wx_pay_config();
        let mut pay_info = HashMap::new();
        let timestamp = gen_timestamp().to_string();
        let nonce_str = unified_order_result.nonce_str.clone().unwrap_or_default();
        let trade_type = request.trade_type.as_deref().unwrap_or_default();
        match trade_type {
            trade_type_const::NATIVE => {
                pay_info.insert(
                    "codeUrl".to_string(),
                    unified_order_result.code_url.clone().unwrap_or_default(),
                );
            }
            trade_type_const::APP => {
                // APP支付绑定的是微信开放平台上的账号，APPID为开放平台上绑定APP后发放的参数
                let app_id = config.app_id().unwrap_or_default().to_string();
                let partner_id = config.mch_id().unwrap_or_default().to_string();
                let mut config_map = HashMap::new();
                config_map.insert("prepayid".to_string(), prepay_id.to_string());
                config_map.insert("partnerid".to_string(), partner_id.clone());
                let package_value = "Sign=WXPay";
                config_map.insert("package".to_string(), package_value.to_string());
                config_map.insert("timestamp".to_string(), timestamp.clone());
                config_map.insert("noncestr".to_string(), nonce_str.clone());
                config_map.insert("appid".to_string(), app_id.clone());
                // 此map用于客户端与微信服务器交互
                pay_info.insert(
                    "sign".to_string(),
                    sign_utils_create(config_map, request.sign_type.as_deref(), config.as_ref())?,
                );
                pay_info.insert("prepayId".to_string(), prepay_id.to_string());
                pay_info.insert("partnerId".to_string(), partner_id);
                pay_info.insert("appId".to_string(), app_id);
                pay_info.insert("packageValue".to_string(), package_value.to_string());
                pay_info.insert("timeStamp".to_string(), timestamp.clone());
                pay_info.insert("nonceStr".to_string(), nonce_str);
            }
            trade_type_const::JSAPI => {
                pay_info.insert(
                    "appId".to_string(),
                    unified_order_result.appid.clone().unwrap_or_default(),
                );
                pay_info.insert("timeStamp".to_string(), timestamp);
                pay_info.insert("nonceStr".to_string(), nonce_str);
                pay_info.insert("package".to_string(), format!("prepay_id={prepay_id}"));
                if let Some(st) = request.sign_type.as_deref() {
                    pay_info.insert("signType".to_string(), st.to_string());
                }
                let sign = sign_utils_create(
                    pay_info.clone(),
                    request.sign_type.as_deref(),
                    config.as_ref(),
                )?;
                pay_info.insert("paySign".to_string(), sign);
            }
            _ => {}
        }
        Ok(pay_info)
    }

    // ---- 退款域（对应 Java WxPayService 的退款/退款查询方法） ----

    /// 申请退款（对应 Java `refund(WxPayRefundRequest request)`，接口地址
    /// `/secapi/pay/refund`，**需证书**）。
    async fn refund(
        &self,
        request: &WxPayRefundRequest,
    ) -> Result<WxPayRefundResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        refund_request_check(&request)?;
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::REFUND_URL);
        let response = self.post(&url, &xml, true).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayRefundResult::from_xml,
        )
    }

    /// 申请退款 v2（支持单品，对应 Java `refundV2(WxPayRefundRequest request)`，
    /// 接口地址 `/secapi/pay/refundv2`，**需证书**）。
    async fn refund_v2(
        &self,
        request: &WxPayRefundRequest,
    ) -> Result<WxPayRefundResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        refund_request_check(&request)?;
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::REFUND_V2_URL);
        let response = self.post(&url, &xml, true).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayRefundResult::from_xml,
        )
    }

    /// v3 申请退款（对应 Java `refundV3(WxPayRefundV3Request request)`）。
    async fn refund_v3(
        &self,
        request: &WxPayRefundV3Request,
    ) -> Result<WxPayRefundV3Result, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.refund_notify_url().map(str::to_string);
        }
        let url = format!("{}/v3/refund/domestic/refunds", self.get_pay_base_url());
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// v3 服务商申请退款（对应 Java
    /// `partnerRefundV3(WxPayPartnerRefundV3Request request)`）。
    ///
    /// `ADAPTED`：Wave 1 生成器缺陷导致 `WxPayPartnerRefundV3Request` 缺少
    /// `sub_mchid` 字段，Java 中"从配置补齐 subMchid"的分支无法下发。
    async fn partner_refund_v3(
        &self,
        request: &WxPayPartnerRefundV3Request,
    ) -> Result<WxPayRefundV3Result, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .sp_appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sp_appid = config.app_id().map(str::to_string);
        }
        if request
            .sub_appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
            && config.sub_app_id().is_some()
        {
            request.sub_appid = config.sub_app_id().map(str::to_string);
        }
        if request
            .notify_url
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.notify_url = config.refund_notify_url().map(str::to_string);
        }
        let url = format!("{}/v3/refund/domestic/refunds", self.get_pay_base_url());
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 查询退款（对应 Java
    /// `refundQuery(String transactionId, String outTradeNo, String outRefundNo, String refundId)`；
    /// 四个参数按需传，未提供的传 `None`）。
    async fn refund_query(
        &self,
        transaction_id: Option<&str>,
        out_trade_no: Option<&str>,
        out_refund_no: Option<&str>,
        refund_id: Option<&str>,
    ) -> Result<WxPayRefundQueryResult, WxErrorException> {
        let mut request = WxPayRefundQueryRequest::default();
        request.out_trade_no = out_trade_no
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        request.transaction_id = transaction_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        request.out_refund_no = out_refund_no
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        request.refund_id = refund_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        self.refund_query_with_request(&request).await
    }

    /// 查询退款（对应 Java `refundQuery(WxPayRefundQueryRequest request)`）。
    async fn refund_query_with_request(
        &self,
        request: &WxPayRefundQueryRequest,
    ) -> Result<WxPayRefundQueryResult, WxErrorException> {
        // 对应 Java checkConstraints：四参数必须四选一
        let count = [
            request.transaction_id.as_deref(),
            request.out_trade_no.as_deref(),
            request.out_refund_no.as_deref(),
            request.refund_id.as_deref(),
        ]
        .iter()
        .filter(|v| v.map(|s| !s.trim().is_empty()).unwrap_or(false))
        .count();
        if count == 0 || count == 4 {
            return Err(impl_utils::runtime(
                "transactionId，outRefundNo，transactionId，refundId 必须四选一",
            ));
        }
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::REFUND_QUERY_URL);
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayRefundQueryResult::from_xml,
        )
    }

    /// 查询退款 v2（对应 Java `refundQueryV2(WxPayRefundQueryRequest request)`）。
    async fn refund_query_v2(
        &self,
        request: &WxPayRefundQueryRequest,
    ) -> Result<WxPayRefundQueryResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!(
            "{}{}",
            self.get_pay_base_url(),
            pay_url::REFUND_QUERY_V2_URL
        );
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayRefundQueryResult::from_xml,
        )
    }

    /// v3 查询退款（对应 Java `refundQueryV3(String outRefundNo)`）。
    async fn refund_query_v3(
        &self,
        out_refund_no: &str,
    ) -> Result<WxPayRefundQueryV3Result, WxErrorException> {
        let url = format!(
            "{}/v3/refund/domestic/refunds/{}",
            self.get_pay_base_url(),
            out_refund_no
        );
        let response = self.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// v3 查询退款（对应 Java `refundQueryV3(WxPayRefundQueryV3Request request)`）。
    async fn refund_query_v3_with_request(
        &self,
        request: &WxPayRefundQueryV3Request,
    ) -> Result<WxPayRefundQueryV3Result, WxErrorException> {
        let url = format!(
            "{}/v3/refund/domestic/refunds/{}",
            self.get_pay_base_url(),
            request.out_refund_no.as_deref().unwrap_or_default()
        );
        let response = self.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// v3 服务商查询退款（对应 Java
    /// `refundPartnerQueryV3(WxPayRefundQueryV3Request request)`）。
    async fn refund_partner_query_v3(
        &self,
        request: &WxPayRefundQueryV3Request,
    ) -> Result<WxPayRefundQueryV3Result, WxErrorException> {
        let url = format!(
            "{}/v3/refund/domestic/refunds/{}?sub_mchid={}",
            self.get_pay_base_url(),
            request.out_refund_no.as_deref().unwrap_or_default(),
            request.sub_mchid.as_deref().unwrap_or_default()
        );
        let response = self.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    // ---- 通知域（对应 Java WxPayService 的 parseXxxNotifyResult 方法） ----

    /// 解析支付结果通知（XML，对应 Java
    /// `parseOrderNotifyResult(String xmlData)`，v2 签名类型取配置/报文）。
    async fn parse_order_notify_result(
        &self,
        xml_data: &str,
    ) -> Result<WxPayOrderNotifyResult, WxErrorException> {
        self.parse_order_notify_result_with_sign_type(xml_data, None)
            .await
    }

    /// 解析支付结果通知（XML，指定签名类型，对应 Java
    /// `parseOrderNotifyResult(String xmlData, String signType)`）。
    async fn parse_order_notify_result_with_sign_type(
        &self,
        xml_data: &str,
        sign_type: Option<&str>,
    ) -> Result<WxPayOrderNotifyResult, WxErrorException> {
        // 对应 Java：检测到 V3 JSON 通知数据时给出处理建议
        if xml_data.trim_start().starts_with('{') {
            return Err(impl_utils::runtime(
                "检测到V3版本的JSON格式通知数据，请使用parseOrderNotifyV3Result方法解析。 V3 API需要传入SignatureHeader参数进行签名验证。",
            ));
        }
        let result = WxPayOrderNotifyResult::from_xml(xml_data)
            .map_err(|e| impl_utils::runtime(format!("发生异常！{e}")))?;
        let mut sign_type = sign_type.map(str::to_string);
        if sign_type.is_none() {
            // 对应 Java：先按报文 mchId/appid 切换配置，再取签名类型
            self.switchover(
                result.mch_id.as_deref().unwrap_or_default(),
                result.appid.as_deref().unwrap_or_default(),
            );
            sign_type = result
                .sign_type
                .clone()
                .or_else(|| self.wx_pay_config().sign_type().map(str::to_string));
        }
        let config = self.wx_pay_config();
        impl_utils::check_result(config.as_ref(), xml_data, sign_type.as_deref(), false)?;
        Ok(result)
    }

    /// 校验 v3 通知签名（对应 Java
    /// `verifyNotifySign(SignatureHeader header, String data)`）。
    async fn verify_notify_sign(
        &self,
        header: &SignatureHeader,
        data: &str,
    ) -> Result<bool, WxErrorException> {
        verify_notify_sign_with_config(self.wx_pay_config().as_ref(), header, data)
    }

    /// 解析 v3 支付结果通知（对应 Java
    /// `parseOrderNotifyV3Result(String notifyData, SignatureHeader header)`）。
    async fn parse_order_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPayNotifyV3Result, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::wx_pay_notify_v3_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(WxPayNotifyV3Result {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 解析 v3 服务商支付结果通知（对应 Java
    /// `parsePartnerOrderNotifyV3Result(String notifyData, SignatureHeader header)`）。
    async fn parse_partner_order_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPayPartnerNotifyV3Result, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::wx_pay_partner_notify_v3_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(WxPayPartnerNotifyV3Result {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 通用 v3 通知解析（对应 Java
    /// `<T extends WxPayBaseNotifyV3Result<E>, E> T baseParseOrderNotifyV3Result(
    /// String notifyData, SignatureHeader header, Class<T> resultType, Class<E> dataType)`）。
    ///
    /// `ADAPTED`：Java 双泛型 + `Class` 参数以 `serde_json::Value` 类型擦除，
    /// 返回 `{"rawData": OriginNotifyResponse, "result": 解密结果}` 结构
    /// （与 Java `rawData`/`result` 字段一一对应）。
    async fn base_parse_order_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<serde_json::Value, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<serde_json::Value>(
            self.wx_pay_config().as_ref(),
            notify_data,
            header,
        )?;
        Ok(serde_json::json!({ "rawData": raw, "result": result }))
    }

    /// 解析合单支付通知（对应 Java
    /// `parseCombineNotifyResult(String notifyData, SignatureHeader header)`）。
    async fn parse_combine_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<CombineNotifyResult, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::combine_notify_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(CombineNotifyResult {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 解析退款结果通知（XML，对应 Java `parseRefundNotifyResult(String xmlData)`，
    /// 含 v2 退款回调 AES-256-ECB 解密）。
    async fn parse_refund_notify_result(
        &self,
        xml_data: &str,
    ) -> Result<WxPayRefundNotifyResult, WxErrorException> {
        let result = WxPayRefundNotifyResult::from_xml(xml_data)
            .map_err(|e| impl_utils::runtime(format!("发生异常，{e}")))?;
        self.switchover(
            result.mch_id.as_deref().unwrap_or_default(),
            result.appid.as_deref().unwrap_or_default(),
        );
        // 对应 Java decryptReqInfo：return_code 为 FAIL 时直接返回，不解密
        if result.return_code.as_deref() == Some(result_code::FAIL) {
            return Ok(result);
        }
        let mut result = result;
        if let Some(req_info_b64) = result.req_info_string.clone() {
            let config = self.wx_pay_config();
            let mch_key = config.mch_key().unwrap_or_default().to_string();
            let decrypted = impl_utils::decrypt_refund_req_info(&mch_key, &req_info_b64)
                .map_err(|e| impl_utils::runtime(format!("发生异常，{e}")))?;
            let req_info: ReqInfo = quick_xml::de::from_str(&decrypted)
                .map_err(|e| impl_utils::runtime(format!("发生异常，{e}")))?;
            result.req_info = Some(req_info);
        }
        Ok(result)
    }

    /// 解析 v3 退款结果通知（对应 Java
    /// `parseRefundNotifyV3Result(String notifyData, SignatureHeader header)`）。
    async fn parse_refund_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPayRefundNotifyV3Result, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::wx_pay_refund_notify_v3_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(WxPayRefundNotifyV3Result {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 解析 v3 转账批次通知（对应 Java
    /// `parseTransferBatchesNotifyV3Result(String notifyData, SignatureHeader header)`）。
    async fn parse_transfer_batches_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPayTransferBatchesNotifyV3Result, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::wx_pay_transfer_batches_notify_v3_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(WxPayTransferBatchesNotifyV3Result {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 解析商家转账通知（对应 Java
    /// `parseTransferBillsNotifyV3Result(String notifyData, SignatureHeader header)`）。
    async fn parse_transfer_bills_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<TransferBillsNotifyResult, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::transfer::transfer_bills_notify_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(TransferBillsNotifyResult {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 解析 v3 服务商退款结果通知（对应 Java
    /// `parsePartnerRefundNotifyV3Result(String notifyData, SignatureHeader header)`）。
    async fn parse_partner_refund_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPayPartnerRefundNotifyV3Result, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::wx_pay_partner_refund_notify_v3_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(WxPayPartnerRefundNotifyV3Result {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 解析服务商订阅通知（对应 Java
    /// `parsePartnerSubscribeNotify(String notifyData, SignatureHeader header)`）。
    async fn parse_partner_subscribe_notify(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<PartnerSubscribeNotifyResult, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::partner_subscribe_notify_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(PartnerSubscribeNotifyResult {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    /// 解析扫码支付通知（XML，指定签名类型，对应 Java
    /// `parseScanPayNotifyResult(String xmlData, String signType)`）。
    ///
    /// Java 中 `signType` 参数已废弃并被忽略（恒用 `config.getSignType()`），
    /// 此处语义一致。
    async fn parse_scan_pay_notify_result_with_sign_type(
        &self,
        xml_data: &str,
        _sign_type: &str,
    ) -> Result<WxScanPayNotifyResult, WxErrorException> {
        let result = WxScanPayNotifyResult::from_xml(xml_data)
            .map_err(|e| impl_utils::runtime(format!("发生异常，{e}")))?;
        self.switchover(
            result.mch_id.as_deref().unwrap_or_default(),
            result.appid.as_deref().unwrap_or_default(),
        );
        let config = self.wx_pay_config();
        impl_utils::check_result(config.as_ref(), xml_data, config.sign_type(), false)?;
        Ok(result)
    }

    /// 解析扫码支付通知（XML，对应 Java `parseScanPayNotifyResult(String xmlData)`）。
    async fn parse_scan_pay_notify_result(
        &self,
        xml_data: &str,
    ) -> Result<WxScanPayNotifyResult, WxErrorException> {
        self.parse_scan_pay_notify_result_with_sign_type(xml_data, "")
            .await
    }

    /// 解析投诉通知（对应 Java
    /// `parseComplaintNotifyResult(String notifyData, SignatureHeader header)`）。
    async fn parse_complaint_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<ComplaintNotifyResult, WxErrorException> {
        let (raw, result) = parse_notify_v3_typed::<
            crate::bean::notify::complaint_notify_result::DecryptNotifyResult,
        >(self.wx_pay_config().as_ref(), notify_data, header)?;
        Ok(ComplaintNotifyResult {
            raw_data: Some(raw),
            result: Some(result),
        })
    }

    // ---- 其他业务（扫码支付二维码/账单/刷卡/撤销/短链接/代金券/评价/人脸/汇率） ----

    /// 生成扫码支付模式一二维码（含 logo 版本，对应 Java
    /// `createScanPayQrcodeMode1(String productId, File logoFile, Integer sideLength)`）。
    ///
    /// `ADAPTED`：Java `File` 以 `&Path` 镜像，`Integer` 以 `Option<i32>` 镜像。
    /// 二维码图片编码依赖二维码生成库，本波次保留未实现（URL 文本生成见
    /// [`WxPayService::create_scan_pay_qrcode_mode1`]）。
    async fn create_scan_pay_qrcode_mode1_with_logo(
        &self,
        _product_id: &str,
        _logo_file: Option<&Path>,
        _side_length: Option<i32>,
    ) -> Result<Vec<u8>, WxErrorException> {
        Err(not_implemented("create_scan_pay_qrcode_mode1_with_logo"))
    }

    /// 生成扫码支付模式一二维码（对应 Java `createScanPayQrcodeMode1(String productId)`，
    /// 返回二维码图片链接）。
    async fn create_scan_pay_qrcode_mode1(
        &self,
        product_id: &str,
    ) -> Result<String, WxErrorException> {
        // weixin://wxpay/bizpayurl?sign=XXXXX&appid=XXXXX&mch_id=XXXXX&product_id=XXXXXX&time_stamp=XXXXXX&nonce_str=XXXXX
        let mut params = HashMap::new();
        let config = self.wx_pay_config();
        params.insert(
            "appid".to_string(),
            config.app_id().unwrap_or_default().to_string(),
        );
        params.insert(
            "mch_id".to_string(),
            config.mch_id().unwrap_or_default().to_string(),
        );
        params.insert("product_id".to_string(), product_id.to_string());
        // 这里需要秒，10位数字
        params.insert("time_stamp".to_string(), gen_timestamp().to_string());
        params.insert("nonce_str".to_string(), impl_utils::current_time_millis());
        let sign = crate::util::sign_utils::SignUtils::create_sign(
            &params,
            Some(sign_type_const::MD5),
            config.mch_key().unwrap_or_default(),
            &[],
        )
        .map_err(WxErrorException::from)?;
        params.insert("sign".to_string(), sign);
        // Java 以 HashMap 迭代序拼 URL（无固定顺序），Rust 按字典序拼接
        let mut keys: Vec<&String> = params.keys().collect();
        keys.sort();
        let mut code_url = String::from("weixin://wxpay/bizpayurl?");
        for key in keys {
            code_url.push_str(key);
            code_url.push('=');
            code_url.push_str(&params[key]);
            code_url.push('&');
        }
        code_url.pop();
        Ok(code_url)
    }

    /// 生成扫码支付模式二二维码（对应 Java
    /// `createScanPayQrcodeMode2(String codeUrl, File logoFile, Integer sideLength)`）。
    ///
    /// `ADAPTED`：Java `File` 以 `&Path` 镜像，`Integer` 以 `Option<i32>` 镜像。
    /// 二维码图片编码依赖二维码生成库，本波次保留未实现。
    async fn create_scan_pay_qrcode_mode2(
        &self,
        _code_url: &str,
        _logo_file: Option<&Path>,
        _side_length: Option<i32>,
    ) -> Result<Vec<u8>, WxErrorException> {
        Err(not_implemented("create_scan_pay_qrcode_mode2"))
    }

    /// 交易保障上报（对应 Java `report(WxPayReportRequest request)`，
    /// 接口地址 `/payitil/report`）。
    async fn report(&self, request: &WxPayReportRequest) -> Result<(), WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::REPORT_URL);
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayCommonResult::from_xml,
        )?;
        Ok(())
    }

    /// 下载原始对账单（对应 Java
    /// `downloadRawBill(String billDate, String billType, String tarType, String deviceInfo)`，
    /// 返回原始账单文本；deviceInfo 可不传）。
    async fn download_raw_bill(
        &self,
        bill_date: &str,
        bill_type: &str,
        tar_type: &str,
        device_info: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let mut request = WxPayDownloadBillRequest::default();
        request.bill_type = Some(bill_type.to_string());
        request.bill_date = Some(bill_date.to_string());
        request.tar_type = Some(tar_type.to_string());
        request.device_info = device_info.map(str::to_string);
        self.download_raw_bill_with_request(&request).await
    }

    /// 下载原始对账单（对应 Java
    /// `downloadRawBill(WxPayDownloadBillRequest request)`）。
    async fn download_raw_bill_with_request(
        &self,
        request: &WxPayDownloadBillRequest,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        download_bill_request_check(&request)?;
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::DOWNLOAD_BILL_URL);
        if request.tar_type.as_deref() == Some(crate::constant::wx_pay_constants::tar_type::GZIP) {
            let bytes = self.post_for_bytes(&url, &xml, false).await?;
            let text = impl_utils::gunzip_to_text(&bytes)?;
            if text.starts_with('<') {
                return Err(common_result_error(&text));
            }
            Ok(text)
        } else {
            let response = self.post(&url, &xml, false).await?;
            if response.starts_with('<') {
                return Err(common_result_error(&response));
            }
            Ok(response)
        }
    }

    /// 下载对账单（解析为结构化结果，对应 Java
    /// `downloadBill(String billDate, String billType, String tarType, String deviceInfo)`）。
    async fn download_bill(
        &self,
        bill_date: &str,
        bill_type: &str,
        tar_type: &str,
        device_info: Option<&str>,
    ) -> Result<WxPayBillResult, WxErrorException> {
        let mut request = WxPayDownloadBillRequest::default();
        request.bill_type = Some(bill_type.to_string());
        request.bill_date = Some(bill_date.to_string());
        request.tar_type = Some(tar_type.to_string());
        request.device_info = device_info.map(str::to_string);
        self.download_bill_with_request(&request).await
    }

    /// 下载对账单（对应 Java `downloadBill(WxPayDownloadBillRequest request)`）。
    async fn download_bill_with_request(
        &self,
        request: &WxPayDownloadBillRequest,
    ) -> Result<WxPayBillResult, WxErrorException> {
        let response = self.download_raw_bill_with_request(request).await?;
        if response.is_empty() {
            // 对应 Java：空响应返回 null（Rust 以默认空结果表达）
            return Ok(WxPayBillResult::default());
        }
        // 对应 Java WxPayBillResult.fromRawBillResultString：未知账单类型返回 null
        Ok(impl_utils::parse_bill_result(
            &response,
            request.bill_type.as_deref().unwrap_or_default(),
        )
        .unwrap_or_default())
    }

    /// 下载资金账单（对应 Java
    /// `downloadFundFlow(String billDate, String accountType, String tarType)`）。
    async fn download_fund_flow(
        &self,
        bill_date: &str,
        account_type: &str,
        tar_type: &str,
    ) -> Result<WxPayFundFlowResult, WxErrorException> {
        let mut request = impl_utils::FundFlowBillRequest::default();
        request.bill_date = Some(bill_date.to_string());
        request.account_type = Some(account_type.to_string());
        request.tar_type = Some(tar_type.to_string());
        download_fund_flow_inner(self, &request).await
    }

    /// 下载资金账单（对应 Java `downloadFundFlow(WxPayDownloadFundFlowRequest request)`）。
    ///
    /// Wave 2 修复：`bill_date`/`account_type`/`tar_type` 已补回请求 bean
    /// （此前被误生成到同文件 `AccountType` 结构）；约束检查、HMAC-SHA256
    /// 签名、GZIP/文本处理与响应解析复用 [`WxPayService::download_fund_flow`]
    /// 的公共流程（对应 Java `checkAndSign` + `handleGzipFundFlow`/`post`
    /// + `handleFundFlow`）。
    async fn download_fund_flow_with_request(
        &self,
        request: &WxPayDownloadFundFlowRequest,
    ) -> Result<WxPayFundFlowResult, WxErrorException> {
        // 请求 bean 与公共流程的本地表达字段一一对应，构造后走同一实现
        // （URL `/pay/downloadfundflow`、证书通道、GZIP 解压、错误 XML 转换）。
        let mut inner = impl_utils::FundFlowBillRequest::default();
        inner.appid = request.appid.clone();
        inner.mch_id = request.mch_id.clone();
        inner.sub_app_id = request.sub_app_id.clone();
        inner.sub_mch_id = request.sub_mch_id.clone();
        inner.nonce_str = request.nonce_str.clone();
        inner.sign_type = request.sign_type.clone();
        inner.sign = request.sign.clone();
        inner.bill_date = request.bill_date.clone();
        inner.account_type = request.account_type.clone();
        inner.tar_type = request.tar_type.clone();
        download_fund_flow_inner(self, &inner).await
    }

    /// v3 申请交易账单（对应 Java
    /// `applyTradeBill(WxPayApplyTradeBillV3Request request)`）。
    async fn apply_trade_bill(
        &self,
        request: &WxPayApplyTradeBillV3Request,
    ) -> Result<WxPayApplyBillV3Result, WxErrorException> {
        let bill_date = request.bill_date.as_deref().unwrap_or_default();
        let bill_type = request.bill_type.as_deref().unwrap_or_default();
        let tar_type = request.tar_type.as_deref().unwrap_or_default();
        let url = if tar_type.is_empty() {
            format!(
                "{}/v3/bill/tradebill?bill_date={bill_date}&bill_type={bill_type}",
                self.get_pay_base_url()
            )
        } else {
            format!(
                "{}/v3/bill/tradebill?bill_date={bill_date}&bill_type={bill_type}&tar_type={tar_type}",
                self.get_pay_base_url()
            )
        };
        let response = self.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// v3 申请资金账单（对应 Java
    /// `applyFundFlowBill(WxPayApplyFundFlowBillV3Request request)`，
    /// 请求 URL `/v3/bill/fundflowbill?bill_date=...&account_type=...&tar_type=...`）。
    ///
    /// Wave 2 修复：`bill_date`/`account_type`/`tar_type` 已补回请求 bean
    /// （此前被误生成到同文件 `AccountType` 结构），URL 从请求构造；
    /// `tar_type` 为空时不携带该查询参数（对应 Java `StringUtils.isBlank`）。
    async fn apply_fund_flow_bill(
        &self,
        request: &WxPayApplyFundFlowBillV3Request,
    ) -> Result<WxPayApplyBillV3Result, WxErrorException> {
        let bill_date = request.bill_date.as_deref().unwrap_or_default();
        let account_type = request.account_type.as_deref().unwrap_or_default();
        let tar_type = request.tar_type.as_deref().unwrap_or_default();
        let url = if tar_type.is_empty() {
            format!(
                "{}/v3/bill/fundflowbill?bill_date={bill_date}&account_type={account_type}",
                self.get_pay_base_url()
            )
        } else {
            format!(
                "{}/v3/bill/fundflowbill?bill_date={bill_date}&account_type={account_type}&tar_type={tar_type}",
                self.get_pay_base_url()
            )
        };
        let response = self.get_v3_with_wechat_pay_serial(&url).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 按 url 下载账单（对应 Java `downloadBill(String url)`）。
    ///
    /// `ADAPTED`：Java `InputStream` 以 `Vec<u8>` 镜像。
    async fn download_bill_with_url(&self, url: &str) -> Result<Vec<u8>, WxErrorException> {
        self.download_v3(url).await
    }

    /// 刷卡支付（对应 Java `micropay(WxPayMicropayRequest request)`，
    /// 接口地址 `/pay/micropay`，**需证书**）。
    async fn micropay(
        &self,
        request: &WxPayMicropayRequest,
    ) -> Result<WxPayMicropayResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::MICROPAY_URL);
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayMicropayResult::from_xml,
        )
    }

    /// 人脸支付（对应 Java `codepay(WxPayCodepayRequest request)`）。
    ///
    /// Java 中为 v3 接口（直连/服务商两套 URL），非 v2 人脸支付。
    async fn codepay(
        &self,
        request: &WxPayCodepayRequest,
    ) -> Result<WxPayCodepayResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        // 判断是否为服务商模式：设置了 sp_appid/sp_mchid/sub_mchid 任一即服务商模式
        let is_partner_mode = request
            .sp_appid
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
            || request
                .sp_mchid
                .as_deref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
            || request
                .sub_mchid
                .as_deref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);
        if is_partner_mode {
            if request
                .sp_appid
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                request.sp_appid = config.app_id().map(str::to_string);
            }
            if request
                .sp_mchid
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                request.sp_mchid = config.mch_id().map(str::to_string);
            }
            if request
                .sub_appid
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                request.sub_appid = config.sub_app_id().map(str::to_string);
            }
            if request
                .sub_mchid
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                request.sub_mchid = config.sub_mch_id().map(str::to_string);
            }
            let url = format!(
                "{}/v3/pay/partner/transactions/codepay",
                self.get_pay_base_url()
            );
            let body = serde_json::to_string(&request)
                .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
            let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
            serde_json::from_str(&response)
                .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
        } else {
            if request
                .appid
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                request.appid = config.app_id().map(str::to_string);
            }
            if request
                .mchid
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                request.mchid = config.mch_id().map(str::to_string);
            }
            let url = format!("{}/v3/pay/transactions/codepay", self.get_pay_base_url());
            let body = serde_json::to_string(&request)
                .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
            let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
            serde_json::from_str(&response)
                .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
        }
    }

    /// 撤销订单（对应 Java `reverseOrder(WxPayOrderReverseRequest request)`，
    /// 接口地址 `/secapi/pay/reverse`，**需证书**）。
    async fn reverse_order(
        &self,
        request: &WxPayOrderReverseRequest,
    ) -> Result<WxPayOrderReverseResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java checkConstraints
        let has_tid = request
            .transaction_id
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        let has_out = request
            .out_trade_no
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        if !has_tid && !has_out {
            return Err(impl_utils::runtime(
                "transaction_id 和 out_trade_no不能同时为空！",
            ));
        }
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::REVERSE_URL);
        let response = self.post(&url, &xml, true).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayOrderReverseResult::from_xml,
        )
    }

    /// v3 撤销订单（对应 Java
    /// `reverseOrderV3(WxPayOrderReverseV3Request request)`，
    /// 请求 URL `/v3/pay/transactions/out-trade-no/{out_trade_no}/reverse`）。
    ///
    /// Wave 2 修复：`out_trade_no`（Java transient 字段，`#[serde(skip)]`）
    /// 已补回请求 bean，URL 从请求构造；`appid`/`mchid` 空白时从配置回填
    /// （对应 Java `StringUtils.isBlank`）；请求体仅含 `appid`/`mchid`，
    /// 响应解析为 [`WxPayOrderReverseV3Result`]。
    async fn reverse_order_v3_with_request(
        &self,
        request: &WxPayOrderReverseV3Request,
    ) -> Result<WxPayOrderReverseV3Result, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java `StringUtils.isBlank(appid)` → 配置回填
        if request
            .appid
            .as_deref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            request.appid = config.app_id().map(str::to_string);
        }
        // 对应 Java `StringUtils.isBlank(mchid)` → 配置回填
        if request
            .mchid
            .as_deref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            request.mchid = config.mch_id().map(str::to_string);
        }
        let url = format!(
            "{}/v3/pay/transactions/out-trade-no/{}/reverse",
            self.get_pay_base_url(),
            request.out_trade_no.as_deref().unwrap_or_default()
        );
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// v3 撤销订单（对应 Java `reverseOrderV3(String outTradeNo)`）。
    async fn reverse_order_v3(
        &self,
        out_trade_no: &str,
    ) -> Result<WxPayOrderReverseV3Result, WxErrorException> {
        if out_trade_no.trim().is_empty() {
            return Err(impl_utils::runtime("out_trade_no不能为空"));
        }
        let config = self.wx_pay_config();
        let mut request = WxPayOrderReverseV3Request::default();
        if let Some(app_id) = config.app_id() {
            request.appid = Some(app_id.to_string());
        }
        if let Some(mch_id) = config.mch_id() {
            request.mchid = Some(mch_id.to_string());
        }
        let url = format!(
            "{}/v3/pay/transactions/out-trade-no/{}/reverse",
            self.get_pay_base_url(),
            out_trade_no.trim()
        );
        let body = serde_json::to_string(&request)
            .map_err(|e| impl_utils::runtime(format!("序列化失败: {e}")))?;
        let response = self.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response)
            .map_err(|e| impl_utils::runtime(format!("解析响应失败: {e}")))
    }

    /// 转换短链接（对应 Java `shorturl(WxPayShorturlRequest request)`，
    /// 接口地址 `/tools/shorturl`）。
    async fn shorturl_with_request(
        &self,
        request: &WxPayShorturlRequest,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::SHORT_URL);
        let response = self.post(&url, &xml, false).await?;
        let result: WxPayShorturlResult = impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayShorturlResult::from_xml,
        )?;
        Ok(result.short_url.unwrap_or_default())
    }

    /// 转换短链接（对应 Java `shorturl(String longUrl)`）。
    async fn shorturl(&self, long_url: &str) -> Result<String, WxErrorException> {
        let mut request = WxPayShorturlRequest::default();
        request.long_url = Some(long_url.to_string());
        self.shorturl_with_request(&request).await
    }

    /// 授权码查询 openid（对应 Java
    /// `authcode2Openid(WxPayAuthcode2OpenidRequest request)`，
    /// 接口地址 `/tools/authcodetoopenid`）。
    async fn authcode2_openid_with_request(
        &self,
        request: &WxPayAuthcode2OpenidRequest,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!(
            "{}{}",
            self.get_pay_base_url(),
            pay_url::AUTH_CODE_TO_OPENID_URL
        );
        let response = self.post(&url, &xml, false).await?;
        let result: WxPayAuthcode2OpenidResult = impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayAuthcode2OpenidResult::from_xml,
        )?;
        Ok(result.openid.unwrap_or_default())
    }

    /// 授权码查询 openid（对应 Java `authcode2Openid(String authCode)`）。
    async fn authcode2_openid(&self, auth_code: &str) -> Result<String, WxErrorException> {
        let mut request = WxPayAuthcode2OpenidRequest::default();
        request.auth_code = Some(auth_code.to_string());
        self.authcode2_openid_with_request(&request).await
    }

    /// 获取沙箱签名 key（对应 Java `getSandboxSignKey()`，接口地址
    /// `https://api.mch.weixin.qq.com/xdc/apiv2getsignkey/sign/getsignkey`）。
    async fn get_sandbox_sign_key(&self) -> Result<String, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = WxPayDefaultRequest::default();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let response = self
            .post_with_mime_type(
                pay_url::GET_SANDBOX_SIGN_KEY_URL,
                &xml,
                false,
                "application/xml",
            )
            .await?;
        let result: WxPaySandboxSignKeyResult = impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPaySandboxSignKeyResult::from_xml,
        )?;
        Ok(result.sandbox_sign_key.unwrap_or_default())
    }

    /// 发放代金券（对应 Java `sendCoupon(WxPayCouponSendRequest request)`，
    /// 接口地址 `/mmpaymkttransfers/send_coupon`）。
    async fn send_coupon(
        &self,
        request: &WxPayCouponSendRequest,
    ) -> Result<WxPayCouponSendResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::SEND_COUPON_URL);
        let response = self.post(&url, &xml, true).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayCouponSendResult::from_xml,
        )
    }

    /// 查询代金券批次（对应 Java
    /// `queryCouponStock(WxPayCouponStockQueryRequest request)`，
    /// 接口地址 `/mmpaymkttransfers/query_coupon_stock`）。
    async fn query_coupon_stock(
        &self,
        request: &WxPayCouponStockQueryRequest,
    ) -> Result<WxPayCouponStockQueryResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!(
            "{}{}",
            self.get_pay_base_url(),
            pay_url::QUERY_COUPON_STOCK_URL
        );
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayCouponStockQueryResult::from_xml,
        )
    }

    /// 查询代金券信息（对应 Java
    /// `queryCouponInfo(WxPayCouponInfoQueryRequest request)`，
    /// 接口地址 `/mmpaymkttransfers/querycouponsinfo`）。
    async fn query_coupon_info(
        &self,
        request: &WxPayCouponInfoQueryRequest,
    ) -> Result<WxPayCouponInfoQueryResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!(
            "{}{}",
            self.get_pay_base_url(),
            pay_url::QUERY_COUPON_INFO_URL
        );
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayCouponInfoQueryResult::from_xml,
        )
    }

    /// 获取最近一次接口请求的请求/响应数据（对应 Java `getWxApiData()`，
    /// 由 `ifSaveApiData` 控制是否记录；Java 为 ThreadLocal，Rust 为
    /// impl 内 RwLock）。
    fn get_wx_api_data(&self) -> Option<WxPayApiData> {
        None
    }

    /// 拉取订单评价数据（对应 Java
    /// `queryComment(Date beginDate, Date endDate, Integer offset, Integer limit)`，
    /// 日期格式 `yyyyMMddHHmmss`）。
    ///
    /// `ADAPTED`：Java `Date` 以 `chrono::DateTime<Utc>` 镜像，`Integer` 以
    /// `Option<i32>` 镜像。
    async fn query_comment(
        &self,
        begin_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<String, WxErrorException> {
        let mut request = WxPayQueryCommentRequest::default();
        request.begin_time = Some(begin_date.format("%Y%m%d%H%M%S").to_string());
        request.end_time = Some(end_date.format("%Y%m%d%H%M%S").to_string());
        request.offset = offset;
        request.limit = limit;
        self.query_comment_with_request(&request).await
    }

    /// 拉取订单评价数据（对应 Java `queryComment(WxPayQueryCommentRequest request)`）。
    async fn query_comment_with_request(
        &self,
        request: &WxPayQueryCommentRequest,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        // 签名类型目前仅支持HMAC-SHA256，默认就是HMAC-SHA256（对应 Java）
        request.sign_type = Some(sign_type_const::HMAC_SHA256.to_string());
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::QUERY_COMMENT_URL);
        let response = self.post(&url, &xml, true).await?;
        if response.starts_with('<') {
            return Err(common_result_error(&response));
        }
        Ok(response)
    }

    /// 获取微信人脸核身信息（对应 Java
    /// `getWxPayFaceAuthInfo(WxPayFaceAuthInfoRequest request)`）。
    async fn get_wx_pay_face_auth_info(
        &self,
        request: &WxPayFaceAuthInfoRequest,
    ) -> Result<WxPayFaceAuthInfoResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        if request
            .sign_type
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.sign_type = Some(sign_type_const::MD5.to_string());
        }
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = "https://payapp.weixin.qq.com/face/get_wxpayface_authinfo";
        let response = self.post(url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayFaceAuthInfoResult::from_xml,
        )
    }

    /// 人脸支付（对应 Java `facepay(WxPayFacepayRequest request)`，
    /// 接口地址 `/pay/facepay`）。
    async fn facepay(
        &self,
        request: &WxPayFacepayRequest,
    ) -> Result<WxPayFacepayResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!("{}{}", self.get_pay_base_url(), pay_url::FACE_PAY_URL);
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayFacepayResult::from_xml,
        )
    }

    /// 汇率查询（对应 Java `queryExchangeRate(String feeType, String date)`，
    /// 接口地址 `/pay/queryexchagerate`）。
    async fn query_exchange_rate(
        &self,
        fee_type: &str,
        date: &str,
    ) -> Result<WxPayQueryExchangeRateResult, WxErrorException> {
        let config = self.wx_pay_config();
        let mut request = WxPayQueryExchangeRateRequest::default();
        request.fee_type = Some(fee_type.to_string());
        request.date = Some(date.to_string());
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let xml = request
            .to_xml()
            .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
        let url = format!(
            "{}{}",
            self.get_pay_base_url(),
            pay_url::QUERY_EXCHANGE_RATE_URL
        );
        let response = self.post(&url, &xml, false).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response,
            request.sign_type().as_deref(),
            true,
            WxPayQueryExchangeRateResult::from_xml,
        )
    }
}

/// v2 退款请求约束检查（对应 Java `WxPayRefundRequest#checkConstraints`）。
fn refund_request_check(request: &WxPayRefundRequest) -> Result<(), WxErrorException> {
    if let Some(account) = request.refund_account.as_deref() {
        if !account.trim().is_empty()
            && ![
                crate::constant::wx_pay_constants::refund_account_source::RECHARGE_FUNDS,
                crate::constant::wx_pay_constants::refund_account_source::UNSETTLED_FUNDS,
            ]
            .contains(&account)
        {
            return Err(impl_utils::runtime(format!(
                "refund_account目前必须为[REFUND_SOURCE_RECHARGE_FUNDS, REFUND_SOURCE_UNSETTLED_FUNDS]其中之一,实际值：{account}"
            )));
        }
    }
    let has_out = request
        .out_trade_no
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    let has_tid = request
        .transaction_id
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    if !has_out && !has_tid {
        return Err(impl_utils::runtime(
            "transaction_id 和 out_trade_no 不能同时为空，必须提供一个",
        ));
    }
    Ok(())
}

/// 下载对账单请求约束检查（对应 Java `WxPayDownloadBillRequest#checkConstraints`）。
fn download_bill_request_check(request: &WxPayDownloadBillRequest) -> Result<(), WxErrorException> {
    if let Some(tar) = request.tar_type.as_deref() {
        if !tar.trim().is_empty() && tar.trim() != crate::constant::wx_pay_constants::tar_type::GZIP
        {
            return Err(impl_utils::runtime("tar_type值如果存在，只能为GZIP"));
        }
    }
    let bill_types = [
        crate::constant::wx_pay_constants::bill_type::ALL,
        crate::constant::wx_pay_constants::bill_type::SUCCESS,
        crate::constant::wx_pay_constants::bill_type::REFUND,
        crate::constant::wx_pay_constants::bill_type::RECHARGE_REFUND,
    ];
    let bill_type = request.bill_type.as_deref().unwrap_or_default();
    if !bill_types.contains(&bill_type) {
        return Err(impl_utils::runtime(format!(
            "bill_type目前必须为[ALL, SUCCESS, REFUND, RECHARGE_REFUND]其中之一,实际值：{bill_type}"
        )));
    }
    Ok(())
}

/// 解析 XML 错误报文并构造错误（对应 Java
/// `WxPayException.from(BaseWxPayResult.fromXML(responseContent, WxPayCommonResult.class))`）。
fn common_result_error(xml: &str) -> WxErrorException {
    let map = crate::bean::xml::root_children_map(xml).unwrap_or_default();
    let mut parts: Vec<String> = Vec::new();
    if let Some(v) = map.get("return_code") {
        parts.push(format!("返回代码：[{v}]"));
    }
    if let Some(v) = map.get("return_msg") {
        parts.push(format!("返回信息：[{v}]"));
    }
    if let Some(v) = map.get("result_code") {
        parts.push(format!("结果代码：[{v}]"));
    }
    if let Some(v) = map.get("err_code") {
        parts.push(format!("错误代码：[{v}]"));
    }
    if let Some(v) = map.get("err_code_des") {
        parts.push(format!("错误详情：[{v}]"));
    }
    parts.push(format!("微信返回的原始报文：\n{xml}"));
    impl_utils::runtime(parts.join("，"))
}

/// 资金账单下载公共流程（对应 Java `downloadFundFlow(WxPayDownloadFundFlowRequest)`：
/// 约束检查 + HMAC-SHA256 + 证书通道 + GZIP/文本处理）。
async fn download_fund_flow_inner<S: WxPayService + ?Sized>(
    service: &S,
    request: &impl_utils::FundFlowBillRequest,
) -> Result<WxPayFundFlowResult, WxErrorException> {
    let config = service.wx_pay_config();
    let mut request = request.clone();
    // 对应 Java checkConstraints：tar_type 只能为 GZIP；account_type 限三值；
    // 强制 HMAC-SHA256 签名
    if let Some(tar) = request.tar_type.as_deref() {
        if !tar.trim().is_empty() && tar.trim() != crate::constant::wx_pay_constants::tar_type::GZIP
        {
            return Err(impl_utils::runtime("tar_type值如果存在，只能为GZIP"));
        }
    }
    let account_types = ["Basic", "Operation", "Fees"];
    let account_type = request.account_type.as_deref().unwrap_or_default();
    if !account_types.contains(&account_type) {
        return Err(impl_utils::runtime(format!(
            "account_type必须为[Basic, Operation, Fees]其中之一,实际值：{account_type}"
        )));
    }
    request.sign_type = Some(sign_type_const::HMAC_SHA256.to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut request)?;
    let xml = request
        .to_xml()
        .map_err(|e| impl_utils::runtime(format!("生成XML失败: {e}")))?;
    let url = format!(
        "{}{}",
        service.get_pay_base_url(),
        pay_url::DOWNLOAD_FUND_FLOW_URL
    );
    let response =
        if request.tar_type.as_deref() == Some(crate::constant::wx_pay_constants::tar_type::GZIP) {
            let bytes = service.post_for_bytes(&url, &xml, true).await?;
            let text = impl_utils::gunzip_to_text(&bytes)?;
            if text.starts_with('<') {
                return Err(common_result_error(&text));
            }
            text
        } else {
            let response = service.post(&url, &xml, true).await?;
            if response.starts_with('<') {
                return Err(common_result_error(&response));
            }
            response
        };
    Ok(impl_utils::parse_fund_flow_result(&response))
}

/// v2 二次签名便捷封装（对应 Java `SignUtils.createSign(...)`）。
fn sign_utils_create(
    params: HashMap<String, String>,
    sign_type: Option<&str>,
    config: &dyn WxPayConfig,
) -> Result<String, WxErrorException> {
    crate::util::sign_utils::SignUtils::create_sign(
        &params,
        sign_type,
        config.mch_key().unwrap_or_default(),
        &[],
    )
    .map_err(WxErrorException::from)
}
