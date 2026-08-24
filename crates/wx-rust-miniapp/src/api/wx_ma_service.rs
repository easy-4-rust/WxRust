//! 小程序服务门面。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaService` + `BaseWxMaServiceImpl`
//! 及其业务子服务（user/security/qrcode/subscribe/msg/internet/link）在门面
//! 上暴露的全部方法。
//! Java 三层继承链（Impl → HttpComponentsImpl → Base）在 Rust 以
//! trait 默认实现 + 组合表达（与 mp 模块同一设计原则）：本 trait 携带 Base
//! 的全部默认实现（access_token 双检锁、GET/POST 执行引擎、签名校验、token
//! 提取、登录会话、业务子域方法），具体实现仅需提供配置存储与 HTTP 客户端。
//!
//! 说明：
//! - Java 门面中的 53 个子服务 getter（`getMsgService()` 等，对应
//!   `getXxxService`）已由 Wave 3 装配：本 trait 提供默认返回 `None` 的
//!   getter，`WxMaServiceImpl` 覆写为返回实际子服务实例。
//! - Java `postWithSignature`（API 签名通道，需 AES-GCM + RSA-PSS）依赖后续
//!   波次引入 `rsa`/GCM 能力，本波次未实现（见报告）。
//! - 本波次未引用任何 bean 之外的待生成类型；bean 路径均为
//!   `crate::bean::*`（对应 Java `cn.binarywang.wx.miniapp.bean.*`）。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use base64::Engine as _;
use hmac::{Hmac, KeyInit, Mac};
use sha1::Digest as Sha1Digest;
use sha2::Sha256;

use wx_rust_common::bean::subscribemsg::{
    CategoryData, PubTemplateKeyword, PubTemplateTitleListResult, TemplateInfo,
};
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::SignUtils;
use wx_rust_common::util::crypto::Sha1;
use wx_rust_common::util::fs::FileUtils;
use wx_rust_common::util::http::MediaUploadRequestExecutor;

use crate::api::{
    WxMaAnalysisService, WxMaCloudService, WxMaCodeService, WxMaComplaintService,
    WxMaCustomserviceWorkService, WxMaDeviceSubscribeService, WxMaEmployeeRelationService,
    WxMaExpressDeliveryReturnService, WxMaExpressService, WxMaFaceService,
    WxMaImmediateDeliveryService, WxMaInternetService, WxMaIntracityService, WxMaJsapiService,
    WxMaKefuService, WxMaLinkService, WxMaLiveGoodsService, WxMaLiveMemberService, WxMaLiveService,
    WxMaMarketingService, WxMaMediaService, WxMaMsgService, WxMaOpenApiService,
    WxMaOrderManagementService, WxMaOrderShippingService, WxMaPluginService,
    WxMaProductOrderService, WxMaProductService, WxMaPromotionService, WxMaQrcodeJumpService,
    WxMaQrcodeService, WxMaReimburseInvoiceService, WxMaRunService, WxMaSchemeService,
    WxMaSecurityService, WxMaSettingService, WxMaShareService, WxMaShopAccountService,
    WxMaShopAfterSaleService, WxMaShopAuditService, WxMaShopCatService, WxMaShopCouponService,
    WxMaShopDeliveryService, WxMaShopImgService, WxMaShopOrderService, WxMaShopPayService,
    WxMaShopRegisterService, WxMaShopSharerService, WxMaShopSpuService, WxMaSubscribeService,
    WxMaUserService, WxMaVodService, WxMaXPayService,
};

use crate::bean::internet::WxMaInternetResponse;
use crate::bean::safety::{WxMaUserSafetyRiskRankRequest, WxMaUserSafetyRiskRankResponse};
use crate::bean::security::{
    WxMaMediaSecCheckCheckRequest, WxMaMsgSecCheckCheckRequest, WxMaMsgSecCheckCheckResponse,
};
use crate::bean::shortlink::GenerateShortLinkRequest;
use crate::bean::urllink::{GenerateUrlLinkRequest, QueryUrlLinkRequest, QueryUrlLinkResponse};
use crate::bean::{
    WxMaCode2VerifyInfoResult, WxMaCodeLineColor, WxMaGetUserNotifyRequest,
    WxMaGetUserNotifyResult, WxMaJscode2SessionResult, WxMaKefuMessage, WxMaMediaAsyncCheckResult,
    WxMaPhoneNumberInfo, WxMaServiceNotifyExtRequest, WxMaServiceNotifyRequest,
    WxMaSubscribeMessage, WxMaUniformMessage, WxMaUpdatableMsg, WxMaUserInfo,
};
use crate::config::WxMaConfig;
use crate::constant::wx_ma_constants::DEFAULT_ENV_VERSION;
use crate::enums::url_business;
use crate::enums::url_core;

/// 小程序服务门面。
#[async_trait]
pub trait WxMaService: Send + Sync {
    /// 当前小程序配置存储（对应 Java `getWxMaConfig()`）。
    fn wx_ma_config(&self) -> Arc<dyn WxMaConfig>;

    /// HTTP 客户端。
    fn http_client(&self) -> &reqwest::Client;

    // ---- 子服务（对应 Java WxMaService 的 `getXxxService()`；默认返回
    // None，由 WxMaServiceImpl 覆写为装配后的实例） ----

    /// 用户服务（对应 Java `getUserService`）。
    fn user_service(&self) -> Option<Arc<dyn WxMaUserService>> {
        None
    }

    /// 消息服务（对应 Java `getMsgService`）。
    fn msg_service(&self) -> Option<Arc<dyn WxMaMsgService>> {
        None
    }

    /// 素材服务（对应 Java `getMediaService`）。
    fn media_service(&self) -> Option<Arc<dyn WxMaMediaService>> {
        None
    }

    /// 客服服务（对应 Java `getKefuService`）。
    fn kefu_service(&self) -> Option<Arc<dyn WxMaKefuService>> {
        None
    }

    /// 数据分析服务（对应 Java `getAnalysisService`）。
    fn analysis_service(&self) -> Option<Arc<dyn WxMaAnalysisService>> {
        None
    }

    /// 小程序码服务（对应 Java `getCodeService`）。
    fn code_service(&self) -> Option<Arc<dyn WxMaCodeService>> {
        None
    }

    /// 物流服务（对应 Java `getExpressService`）。
    fn express_service(&self) -> Option<Arc<dyn WxMaExpressService>> {
        None
    }

    /// 内容安全服务（对应 Java `getSecurityService`）。
    fn security_service(&self) -> Option<Arc<dyn WxMaSecurityService>> {
        None
    }

    /// 设置服务（对应 Java `getSettingService`）。
    fn setting_service(&self) -> Option<Arc<dyn WxMaSettingService>> {
        None
    }

    /// 订阅消息服务（对应 Java `getSubscribeService`）。
    fn subscribe_service(&self) -> Option<Arc<dyn WxMaSubscribeService>> {
        None
    }

    /// 分享服务（对应 Java `getShareService`）。
    fn share_service(&self) -> Option<Arc<dyn WxMaShareService>> {
        None
    }

    /// 小程序 scheme 服务（对应 Java `getWxMaSchemeService`）。
    fn scheme_service(&self) -> Option<Arc<dyn WxMaSchemeService>> {
        None
    }

    /// URL Link 服务（对应 Java `getLinkService`）。
    fn link_service(&self) -> Option<Arc<dyn WxMaLinkService>> {
        None
    }

    /// 二维码服务（对应 Java `getQrcodeService`）。
    fn qrcode_service(&self) -> Option<Arc<dyn WxMaQrcodeService>> {
        None
    }

    /// JSAPI 服务（对应 Java `getJsapiService`）。
    fn jsapi_service(&self) -> Option<Arc<dyn WxMaJsapiService>> {
        None
    }

    /// 插件服务（对应 Java `getPluginService`）。
    fn plugin_service(&self) -> Option<Arc<dyn WxMaPluginService>> {
        None
    }

    /// 运行服务（对应 Java `getRunService`）。
    fn run_service(&self) -> Option<Arc<dyn WxMaRunService>> {
        None
    }

    /// OpenApi 服务（对应 Java `getWxMaOpenApiService`）。
    fn open_api_service(&self) -> Option<Arc<dyn WxMaOpenApiService>> {
        None
    }

    /// 互联网服务（对应 Java `getInternetService`）。
    fn internet_service(&self) -> Option<Arc<dyn WxMaInternetService>> {
        None
    }

    /// 店铺账号服务（对应 Java `getShopAccountService`）。
    fn shop_account_service(&self) -> Option<Arc<dyn WxMaShopAccountService>> {
        None
    }

    /// 售后服务（对应 Java `getShopAfterSaleService`）。
    fn shop_after_sale_service(&self) -> Option<Arc<dyn WxMaShopAfterSaleService>> {
        None
    }

    /// 审核服务（对应 Java `getShopAuditService`）。
    fn shop_audit_service(&self) -> Option<Arc<dyn WxMaShopAuditService>> {
        None
    }

    /// 类目服务（对应 Java `getShopCatService`）。
    fn shop_cat_service(&self) -> Option<Arc<dyn WxMaShopCatService>> {
        None
    }

    /// 优惠券服务（对应 Java `getWxMaShopCouponService`）。
    fn shop_coupon_service(&self) -> Option<Arc<dyn WxMaShopCouponService>> {
        None
    }

    /// 履约服务（对应 Java `getShopDeliveryService`）。
    fn shop_delivery_service(&self) -> Option<Arc<dyn WxMaShopDeliveryService>> {
        None
    }

    /// 图片服务（对应 Java `getShopImgService`）。
    fn shop_img_service(&self) -> Option<Arc<dyn WxMaShopImgService>> {
        None
    }

    /// 订单服务（对应 Java `getShopOrderService`）。
    fn shop_order_service(&self) -> Option<Arc<dyn WxMaShopOrderService>> {
        None
    }

    /// 支付服务（对应 Java `getWxMaShopPayService`）。
    fn shop_pay_service(&self) -> Option<Arc<dyn WxMaShopPayService>> {
        None
    }

    /// 商家注册服务（对应 Java `getShopRegisterService`）。
    fn shop_register_service(&self) -> Option<Arc<dyn WxMaShopRegisterService>> {
        None
    }

    /// 分享员服务（对应 Java `getShopSharerService`）。
    fn shop_sharer_service(&self) -> Option<Arc<dyn WxMaShopSharerService>> {
        None
    }

    /// 商品 SPU 服务（对应 Java `getShopSpuService`）。
    fn shop_spu_service(&self) -> Option<Arc<dyn WxMaShopSpuService>> {
        None
    }

    /// 标准版交易组件商品服务（对应 Java `getProductService`）。
    fn product_service(&self) -> Option<Arc<dyn WxMaProductService>> {
        None
    }

    /// 标准版交易组件订单服务（对应 Java `getProductOrderService`）。
    fn product_order_service(&self) -> Option<Arc<dyn WxMaProductOrderService>> {
        None
    }

    /// 订单管理服务（对应 Java `getWxMaOrderManagementService`）。
    fn order_management_service(&self) -> Option<Arc<dyn WxMaOrderManagementService>> {
        None
    }

    /// 发货信息服务（对应 Java `getWxMaOrderShippingService`）。
    fn order_shipping_service(&self) -> Option<Arc<dyn WxMaOrderShippingService>> {
        None
    }

    /// 物流退货服务（对应 Java `getWxMaExpressDeliveryReturnService`）。
    fn express_delivery_return_service(&self) -> Option<Arc<dyn WxMaExpressDeliveryReturnService>> {
        None
    }

    /// 即时配送服务（对应 Java `getWxMaImmediateDeliveryService`）。
    fn immediate_delivery_service(&self) -> Option<Arc<dyn WxMaImmediateDeliveryService>> {
        None
    }

    /// 用工关系服务（对应 Java `getEmployeeRelationService`）。
    fn employee_relation_service(&self) -> Option<Arc<dyn WxMaEmployeeRelationService>> {
        None
    }

    /// 微信客服服务（对应 Java `getCustomserviceWorkService`）。
    fn customservice_work_service(&self) -> Option<Arc<dyn WxMaCustomserviceWorkService>> {
        None
    }

    /// 直播服务（对应 Java `getLiveService`）。
    fn live_service(&self) -> Option<Arc<dyn WxMaLiveService>> {
        None
    }

    /// 直播商品服务（对应 Java `getLiveGoodsService`）。
    fn live_goods_service(&self) -> Option<Arc<dyn WxMaLiveGoodsService>> {
        None
    }

    /// 直播成员服务（对应 Java `getLiveMemberService`）。
    fn live_member_service(&self) -> Option<Arc<dyn WxMaLiveMemberService>> {
        None
    }

    /// 云开发服务（对应 Java `getCloudService`）。
    fn cloud_service(&self) -> Option<Arc<dyn WxMaCloudService>> {
        None
    }

    /// 视频点播服务（对应 Java `getWxMaVodService`）。
    fn vod_service(&self) -> Option<Arc<dyn WxMaVodService>> {
        None
    }

    /// XPay 服务（对应 Java `getWxMaXPayService`）。
    fn xpay_service(&self) -> Option<Arc<dyn WxMaXPayService>> {
        None
    }

    /// 营销服务（对应 Java `getMarketingService`）。
    fn marketing_service(&self) -> Option<Arc<dyn WxMaMarketingService>> {
        None
    }

    /// 推广服务（对应 Java `getPromotionService`）。
    fn promotion_service(&self) -> Option<Arc<dyn WxMaPromotionService>> {
        None
    }

    /// 同城服务（对应 Java `getIntracityService`）。
    fn intracity_service(&self) -> Option<Arc<dyn WxMaIntracityService>> {
        None
    }

    /// 投诉服务（对应 Java `getComplaintService`）。
    fn complaint_service(&self) -> Option<Arc<dyn WxMaComplaintService>> {
        None
    }

    /// 设备订阅消息服务（对应 Java `getDeviceSubscribeService`）。
    fn device_subscribe_service(&self) -> Option<Arc<dyn WxMaDeviceSubscribeService>> {
        None
    }

    /// 人脸核身服务（对应 Java `getFaceService`）。
    fn face_service(&self) -> Option<Arc<dyn WxMaFaceService>> {
        None
    }

    /// 报销发票服务（对应 Java `getReimburseInvoiceService`）。
    fn reimburse_invoice_service(&self) -> Option<Arc<dyn WxMaReimburseInvoiceService>> {
        None
    }

    /// 二维码跳转服务（对应 Java `getQrcodeJumpService`）。
    fn qrcode_jump_service(&self) -> Option<Arc<dyn WxMaQrcodeJumpService>> {
        None
    }

    /// OCR 服务（对应 Java `getOcrService`）。
    ///
    /// 与 Java 一致：无独立接口文件，直接返回 common 接口
    /// `me.chanjar.weixin.common.service.WxOcrService` 的实现（Rust 侧
    /// `wx_rust_common::service::WxOcrService`，`async_trait` + `Send + Sync`
    /// 保证对象安全）。
    fn ocr_service(&self) -> Option<Arc<dyn wx_rust_common::service::WxOcrService>> {
        None
    }

    /// 图像处理服务（对应 Java `getImgProcService`）。
    ///
    /// 与 Java 一致：无独立接口文件，直接返回 common 接口
    /// `me.chanjar.weixin.common.service.WxImgProcService` 的实现（Rust 侧
    /// `wx_rust_common::service::WxImgProcService`）。
    fn img_proc_service(&self) -> Option<Arc<dyn wx_rust_common::service::WxImgProcService>> {
        None
    }

    // ---- 核心能力（对应 BaseWxMaServiceImpl） ----

    /// 获取 access_token（对应 Java `getAccessToken()`，不强制刷新）。
    async fn get_access_token(&self) -> Result<String, WxErrorException> {
        self.get_access_token_with_force(false).await
    }

    /// 获取 access_token（可强制刷新）。
    ///
    /// 对应 Java `getAccessToken(boolean forceRefresh)`：双检锁 + 3 秒超时
    /// 等待（原 tryLock(100ms) 轮询的 Rust 原生化，语义不变）；稳定版接口
    /// 按配置切换。
    async fn get_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_ma_config();
        if !force_refresh && !config.is_access_token_expired() {
            return config
                .access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
        }

        let lock = config.access_token_lock();
        // 对应 Java tryLock(100ms) 轮询 + 3s 总超时：timeout 包裹「先查—
        // 再等锁—得锁后再查」全流程；拿到 guard 后持有到刷新完成（双检锁）。
        // 等待者提前返回语义：他人刷新出未过期 token 后，等待者在等待前的
        // 双检或取到锁后的双检中直接返回，不发起自己的 token 请求。
        // lock_owned 产生 'static guard（超时 future 完成后仍可持有）。
        let outcome = tokio::time::timeout(std::time::Duration::from_millis(3000), async {
            if !force_refresh && !config.is_access_token_expired() {
                return None;
            }
            let guard = lock.lock_owned().await;
            if !force_refresh && !config.is_access_token_expired() {
                // 双检通过：他人已刷新出未过期 token，放锁并提前返回
                return None;
            }
            Some(guard)
        })
        .await;

        let _guard = match outcome {
            // 等待者提前返回：无需自行刷新
            Ok(None) => {
                return config
                    .access_token()
                    .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
            }
            Ok(Some(guard)) => guard,
            Err(_) => {
                return Err(WxErrorException::from_code(
                    -99,
                    "获取accessToken超时：获取时间超时",
                ));
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

    /// GET 请求（对应 Java `get(String, String)`）。
    ///
    /// 走统一管线 [`wx_rust_common::pipeline::execute_pipeline`]（经
    /// `execute_get_via_pipeline`：-1 指数退避重试 + token 失效单次重放；
    /// query 拼接语义内联于封装——原 `SimpleGetRequestExecutor` 路径）。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_ma_service_impl::execute_get_via_pipeline(
            self,
            url,
            query_param,
        )
        .await
    }

    /// POST 请求（对应 Java `post(String, String)`）。
    ///
    /// 走统一管线（经 `execute_post_via_pipeline`：POST 文本体原样透传 +
    /// -1 指数退避重试 + token 失效单次重放——原
    /// `SimplePostRequestExecutor` 路径）。
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_ma_service_impl::execute_post_via_pipeline(self, url, post_data)
            .await
    }

    /// 校验消息是否来自微信服务器（对应 Java `checkSignature(String, String, String)`）。
    fn check_signature(&self, timestamp: &str, nonce: &str, signature: &str) -> bool {
        let config = self.wx_ma_config();
        let token = config.token().unwrap_or_default();
        // Java `SHA1.gen(token, timestamp, nonce)`：排序后无分隔符拼接
        match Sha1::digest(&[token, timestamp, nonce]) {
            Ok(s) => s == signature,
            Err(_) => false,
        }
    }

    /// 提取 access token（对应 Java `extractAccessToken`）。
    ///
    /// 解析响应 JSON，失败时抛业务错误；成功时更新配置缓存
    /// （Java `updateAccessTokenProcessor` 的回调在 Rust 中以直接更新表达）。
    fn extract_access_token(&self, result_content: &str) -> Result<String, WxErrorException> {
        let config = self.wx_ma_config();
        let error = wx_rust_common::error::WxError::from_json_with_type(
            result_content,
            Some(wx_rust_common::enums::WxType::MiniApp),
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

    /// 通过网络请求获取 access_token（对应 Java 抽象方法 `doGetAccessTokenRequest`）。
    ///
    /// 配置了自定义 `accessTokenUrl`（`%s` 格式串，Java `String.format` 语义）
    /// 时优先使用，否则走标准 `/cgi-bin/token` 地址。
    async fn do_get_access_token_request(&self) -> Result<String, WxErrorException> {
        let config = self.wx_ma_config();
        let url = match config.access_token_url() {
            Some(u) if !u.is_empty() => {
                // Java String.format(url, appid, secret)：按序替换 %s
                u.replacen("%s", config.app_id(), 1)
                    .replacen("%s", config.secret(), 1)
            }
            _ => url_core::get_access_token_url(config.as_ref()),
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

    /// 通过稳定版接口获取 access_token（对应 Java 抽象方法 `doGetStableAccessTokenRequest`）。
    async fn do_get_stable_access_token_request(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_ma_config();
        let url = match config.access_token_url() {
            Some(u) if !u.is_empty() => u.to_string(),
            _ => url_core::get_stable_access_token_url(config.as_ref()),
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

    // ---- 门面核心业务（对应 Java WxMaService / BaseWxMaServiceImpl） ----

    /// 获取登录后的 session 信息（对应 Java `jsCode2SessionInfo(String)`）。
    ///
    /// GET `/sns/jscode2session?appid=&secret=&js_code=&grant_type=authorization_code`；
    /// `errcode != 0` 由执行引擎抛错（`SimpleGetRequestExecutor` 统一校验）。
    async fn js_code_to_session(
        &self,
        js_code: &str,
    ) -> Result<WxMaJscode2SessionResult, WxErrorException> {
        let config = self.wx_ma_config();
        let query = format!(
            "appid={}&secret={}&js_code={}&grant_type=authorization_code",
            config.app_id(),
            config.secret(),
            js_code
        );
        let response = self
            .get(&url_core::js_code_to_session_url(config.as_ref()), &query)
            .await?;
        serde_json::from_str::<WxMaJscode2SessionResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 用户支付完成后获取 UnionId（对应 Java `getPaidUnionId`）。
    ///
    /// GET `/wxa/getpaidunionid?openid=...&transaction_id=...&mch_id=...&out_trade_no=...`，
    /// 后三个参数可选（Java `StringUtils.isNotEmpty` 才拼入）。
    async fn get_paid_union_id(
        &self,
        openid: &str,
        transaction_id: Option<&str>,
        mch_id: Option<&str>,
        out_trade_no: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let mut params = vec![format!("openid={openid}")];
        if let Some(t) = transaction_id {
            if !t.is_empty() {
                params.push(format!("transaction_id={t}"));
            }
        }
        if let Some(m) = mch_id {
            if !m.is_empty() {
                params.push(format!("mch_id={m}"));
            }
        }
        if let Some(o) = out_trade_no {
            if !o.is_empty() {
                params.push(format!("out_trade_no={o}"));
            }
        }
        let config = self.wx_ma_config();
        let response = self
            .get(
                &url_core::get_paid_union_id_url(config.as_ref()),
                &params.join("&"),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("unionid")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "unionid 字段缺失"))
    }

    /// 导入抽样数据到微信后台，用于流量分配（对应 Java `setDynamicData`）。
    ///
    /// POST `/wxa/setdynamicdata`，请求体：`lifespan`/`query`（`{"type":...}`
    /// 的 JSON 字符串）/`data`/`scene`。每个数据包不超过 5K，数据量过大时应
    /// 多线程并发导入。
    async fn set_dynamic_data(
        &self,
        lifespan: i32,
        r#type: &str,
        scene: i32,
        data: &str,
    ) -> Result<(), WxErrorException> {
        let body = serde_json::json!({
            "lifespan": lifespan,
            // Java：query 为 "{\"type\":\"...\"}" 的 JSON 字符串（非嵌套对象）
            "query": serde_json::json!({"type": r#type}).to_string(),
            "data": data,
            "scene": scene,
        });
        let config = self.wx_ma_config();
        self.post(
            &url_core::set_dynamic_data_url(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    // ---- 用户域（对应 Java WxMaUserService） ----

    /// 获取登录会话信息（对应 Java `getSessionInfo(String)`，委托
    /// `jsCode2SessionInfo`）。
    async fn get_session_info(
        &self,
        js_code: &str,
    ) -> Result<WxMaJscode2SessionResult, WxErrorException> {
        self.js_code_to_session(js_code).await
    }

    /// 解密并解析用户信息（对应 Java `getUserInfo(String, String, String)`）。
    ///
    /// 以 session_key 对 encryptedData 做 AES-128-CBC 解密后解析为
    /// `WxMaUserInfo`（解密实现见 `WxMaCryptUtils::decrypt`）。
    async fn get_user_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaUserInfo, WxErrorException> {
        let decrypted =
            crate::util::crypto::wx_ma_crypt_utils::decrypt(session_key, encrypted_data, iv_str)
                .map_err(WxErrorException::Io)?;
        serde_json::from_str::<WxMaUserInfo>(&decrypted)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 解密开放数据原始 JSON（对应 Java `WxMaCryptUtils.decrypt`）。
    ///
    /// Java 门面无此方法，为任务要求的 decrypt_session_info 语义提供门面入口；
    /// 返回解密后的原始 JSON 字符串。
    async fn decrypt_session_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<String, WxErrorException> {
        crate::util::crypto::wx_ma_crypt_utils::decrypt(session_key, encrypted_data, iv_str)
            .map_err(WxErrorException::Io)
    }

    /// 上报用户数据（对应 Java `setUserStorage(Map, String, String)`）。
    ///
    /// POST `/wxa/set_user_storage?appid=&signature=&openid=&sig_method=hmac_sha256`，
    /// signature 为请求体 JSON 的 HmacSHA256 签名（十六进制大写）。
    async fn set_user_storage(
        &self,
        kv_map: &HashMap<String, String>,
        session_key: &str,
        openid: &str,
    ) -> Result<(), WxErrorException> {
        let kv_list: Vec<serde_json::Value> = kv_map
            .iter()
            .map(|(k, v)| serde_json::json!({ "key": k, "value": v }))
            .collect();
        let params = serde_json::json!({ "kv_list": kv_list }).to_string();
        let signature = SignUtils::create_hmac_sha256_sign(&params, session_key);
        let config = self.wx_ma_config();
        let url = url_business::user::set_user_storage_url(
            config.as_ref(),
            config.app_id(),
            &signature,
            openid,
        );
        self.post(&url, &params).await?;
        Ok(())
    }

    /// 解密并解析手机号信息（对应 Java `getPhoneNoInfo(String, String, String)`）。
    async fn get_phone_no_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaPhoneNumberInfo, WxErrorException> {
        let decrypted =
            crate::util::crypto::wx_ma_crypt_utils::decrypt(session_key, encrypted_data, iv_str)
                .map_err(WxErrorException::Io)?;
        serde_json::from_str::<WxMaPhoneNumberInfo>(&decrypted)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 通过 code 获取手机号（对应 Java `getPhoneNumber(String)`）。
    ///
    /// POST `/wxa/business/getuserphonenumber`，响应含 `phone_info` 时解析并
    /// 返回，否则返回 `None`（Java 返回 null）。
    async fn get_phone_number(
        &self,
        code: &str,
    ) -> Result<Option<WxMaPhoneNumberInfo>, WxErrorException> {
        let body = serde_json::json!({ "code": code });
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::user::get_phone_number_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        match json.get("phone_info") {
            Some(info) => serde_json::from_value(info.clone())
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string())),
            None => Ok(None),
        }
    }

    /// 通过 code 获取手机号（对应 Java `getPhoneNoInfo(String)`，委托
    /// `getPhoneNumber`）。
    async fn get_phone_no_info_with_code(
        &self,
        code: &str,
    ) -> Result<Option<WxMaPhoneNumberInfo>, WxErrorException> {
        self.get_phone_number(code).await
    }

    /// 校验用户数据签名（对应 Java `checkUserInfo`）。
    ///
    /// SHA1（rawData + sessionKey 直接拼接，无排序）十六进制小写与
    /// signature 比较。
    fn check_user_info(&self, session_key: &str, raw_data: &str, signature: &str) -> bool {
        let mut hasher = sha1::Sha1::new();
        hasher.update(raw_data.as_bytes());
        hasher.update(session_key.as_bytes());
        let generated = hex::encode(hasher.finalize());
        generated == signature
    }

    /// 多端登录验证（对应 Java `getCode2VerifyInfo(String, String)`）。
    async fn get_code2_verify_info(
        &self,
        code: &str,
        checkcode: &str,
    ) -> Result<WxMaCode2VerifyInfoResult, WxErrorException> {
        let body = serde_json::json!({ "code": code, "checkcode": checkcode });
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::user::code2_verify_info_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str::<WxMaCode2VerifyInfoResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 检查登录态（对应 Java `checkSessionKey(String, String)`）。
    ///
    /// signature 为 openid 的 HmacSHA256 签名（key 为 session_key，十六进制
    /// 大写）；请求成功即返回 true（Java 恒返回 true）。
    async fn check_session_key(
        &self,
        openid: &str,
        session_key: &str,
    ) -> Result<bool, WxErrorException> {
        let signature = SignUtils::create_hmac_sha256_sign(openid, session_key);
        let config = self.wx_ma_config();
        let url = url_business::user::check_session_key_url(config.as_ref(), openid, &signature);
        // Java `service.get(url, null)`：query 为空
        self.get(&url, "").await?;
        Ok(true)
    }

    // ---- 内容安全域（对应 Java WxMaSecurityService） ----

    /// 图片安全检测（对应 Java `checkImage(File)`）。
    ///
    /// 以 multipart（字段名 `media`）上传图片文件到 `/wxa/img_sec_check`；
    /// 上传成功即返回 true（Java `result != null`）。
    async fn check_image_file(&self, file_path: &str) -> Result<bool, WxErrorException> {
        let executor = MediaUploadRequestExecutor::new(self.http_client().clone());
        let content = std::fs::read(file_path).map_err(|e| WxErrorException::Io(e.to_string()))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let param = CommonUploadParam::new("media", CommonUploadData::new(file_name, content));
        let config = self.wx_ma_config();
        crate::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            self,
            &executor,
            &url_business::sec_check::img_sec_check_url(config.as_ref()),
            param,
        )
        .await?;
        Ok(true)
    }

    /// 图片安全检测（对应 Java `checkImage(String fileUrl)`）。
    ///
    /// 先将 fileUrl 下载到系统临时目录（`<毫秒时间戳>.tmp`），再委托
    /// `check_image_file`。下载/写入失败时抛 `-1 文件地址读取异常`
    /// （Java `WxError.builder().errorCode(-1).errorMsg("文件地址读取异常")`）。
    async fn check_image_url(&self, file_url: &str) -> Result<bool, WxErrorException> {
        let resp = self
            .http_client()
            .get(file_url)
            .send()
            .await
            .map_err(|_| WxErrorException::from_code(-1, "文件地址读取异常"))?;
        let bytes = resp
            .bytes()
            .await
            .map_err(|_| WxErrorException::from_code(-1, "文件地址读取异常"))?;
        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let tmp_path = std::env::temp_dir().join(format!("{now_millis}.tmp"));
        std::fs::write(&tmp_path, &bytes)
            .map_err(|_| WxErrorException::from_code(-1, "文件地址读取异常"))?;
        self.check_image_file(
            tmp_path
                .to_str()
                .ok_or_else(|| WxErrorException::from_code(-1, "文件地址读取异常"))?,
        )
        .await
    }

    /// 文本安全检测（对应 Java `checkMessage(String)`）。
    ///
    /// POST `/wxa/msg_sec_check`，请求体 `{"content": ...}`；成功即返回 true。
    async fn check_message(&self, msg_string: &str) -> Result<bool, WxErrorException> {
        let body = serde_json::json!({ "content": msg_string });
        let config = self.wx_ma_config();
        self.post(
            &url_business::sec_check::msg_sec_check_url(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(true)
    }

    /// 文本安全检测（v2 请求体，对应 Java `checkMessage(WxMaMsgSecCheckCheckRequest)`）。
    ///
    /// 请求字段：`version`/`openid`/`scene`/`content`/`nickname`/`title`/`signature`；
    /// 响应解析为 `WxMaMsgSecCheckCheckResponse`。
    async fn check_message_with_request(
        &self,
        request: &WxMaMsgSecCheckCheckRequest,
    ) -> Result<WxMaMsgSecCheckCheckResponse, WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::sec_check::msg_sec_check_url(config.as_ref()),
                &body,
            )
            .await?;
        // Java `parseErrorResponse` 的 errcode 校验已被执行引擎覆盖（同一语义）
        serde_json::from_str::<WxMaMsgSecCheckCheckResponse>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 媒体安全异步检测（对应 Java `mediaCheckAsync(String, int)`）。
    ///
    /// POST `/wxa/media_check_async`，请求体 `{"media_url":..., "media_type":...}`。
    async fn media_check_async(
        &self,
        media_url: &str,
        media_type: i32,
    ) -> Result<WxMaMediaAsyncCheckResult, WxErrorException> {
        let body = serde_json::json!({
            "media_url": media_url,
            "media_type": media_type,
        });
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::sec_check::media_check_async_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str::<WxMaMediaAsyncCheckResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 媒体安全异步检测（v2 请求体，对应 Java `mediaCheckAsync(WxMaMediaSecCheckCheckRequest)`）。
    ///
    /// 请求字段：`media_url`/`media_type`/`version`/`openid`/`scene`。
    async fn media_check_async_with_request(
        &self,
        request: &WxMaMediaSecCheckCheckRequest,
    ) -> Result<WxMaMediaAsyncCheckResult, WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::sec_check::media_check_async_url(config.as_ref()),
                &body,
            )
            .await?;
        // Java `parseErrorResponse` 的 errcode 校验已被执行引擎覆盖（同一语义）
        serde_json::from_str::<WxMaMediaAsyncCheckResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取用户安全等级（对应 Java `getUserRiskRank(WxMaUserSafetyRiskRankRequest)`）。
    async fn get_user_risk_rank(
        &self,
        request: &WxMaUserSafetyRiskRankRequest,
    ) -> Result<WxMaUserSafetyRiskRankResponse, WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::sec_check::get_user_risk_rank_url(config.as_ref()),
                &body,
            )
            .await?;
        // Java 显式 errcode 校验已被执行引擎覆盖（同一语义）
        serde_json::from_str::<WxMaUserSafetyRiskRankResponse>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    // ---- 二维码/小程序码域（对应 Java WxMaQrcodeService） ----

    /// 获取小程序二维码字节（对应 Java `createQrcodeBytes(String, int)`）。
    ///
    /// POST `/cgi-bin/wxaapp/createwxaqrcode`，请求体 `{"path":..., "width":...}`；
    /// 返回图片字节。响应为 JSON（错误）时抛错（对应 Java
    /// `QrcodeBytesRequestExecutor` 按 Content-Type 判断）。
    async fn create_qrcode_bytes(
        &self,
        path: &str,
        width: i32,
    ) -> Result<Vec<u8>, WxErrorException> {
        let body = serde_json::json!({ "path": path, "width": width });
        let executor = crate::api::r#impl::base_wx_ma_service_impl::QrcodeBytesRequestExecutor::new(
            self.http_client().clone(),
        );
        let config = self.wx_ma_config();
        crate::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            self,
            &executor,
            &url_business::qrcode::create_qrcode_url(config.as_ref()),
            body.to_string(),
        )
        .await
    }

    /// 获取小程序二维码并保存为临时文件（对应 Java `createQrcode(String, int)`）。
    ///
    /// 返回文件路径（Java `FileUtils.createTmpFile`：系统临时目录
    /// `wxjava-temp` 下随机名 `.jpg` 文件）。
    async fn create_qrcode(&self, path: &str, width: i32) -> Result<String, WxErrorException> {
        let bytes = self.create_qrcode_bytes(path, width).await?;
        self.save_qrcode_file(&bytes, None).await
    }

    /// 获取小程序二维码并保存到指定目录（对应 Java `createQrcode(String, int, String)`）。
    ///
    /// Java 语义：filePath 为**目录**，文件名随机（`File.createTempFile(name,
    /// ".jpg", dir)`），返回实际写入的文件路径。
    async fn create_qrcode_to_path(
        &self,
        path: &str,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let bytes = self.create_qrcode_bytes(path, width).await?;
        self.save_qrcode_file(&bytes, Some(file_path)).await
    }

    /// 获取小程序二维码（对应 Java `createQrcode(String)`，width 默认 430）。
    async fn create_qrcode_default(&self, path: &str) -> Result<String, WxErrorException> {
        self.create_qrcode(path, 430).await
    }

    /// 获取小程序二维码并保存到指定目录（对应 Java `createQrcode(String, String)`，
    /// width 默认 430）。
    async fn create_qrcode_default_to_path(
        &self,
        path: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        self.create_qrcode_to_path(path, 430, file_path).await
    }

    /// 获取小程序码字节（对应 Java `createWxaCodeBytes` 全参版本）。
    ///
    /// POST `/wxa/getwxacode`，请求体：`path`/`env_version`（空默认
    /// `release`）/`width`/`auto_color`/`is_hyaline`/`line_color`（空默认
    /// `{"r":"0","g":"0","b":"0"}`）。
    async fn create_wxa_code_bytes(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<Vec<u8>, WxErrorException> {
        // Java `StringUtils.defaultIfEmpty(envVersion, DEFAULT_ENV_VERSION)`
        let env_version = match env_version {
            Some(v) if !v.is_empty() => v,
            _ => DEFAULT_ENV_VERSION,
        };
        let mut body = serde_json::json!({
            "path": path,
            "env_version": env_version,
            "width": width,
            "auto_color": auto_color,
            "is_hyaline": is_hyaline,
        });
        // Java `WxaCode.builder().lineColor(null)` 经 Gson 省略 `line_color` 字段
        if let Some(lc) = line_color {
            body["line_color"] =
                serde_json::to_value(lc).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        }
        let executor = crate::api::r#impl::base_wx_ma_service_impl::QrcodeBytesRequestExecutor::new(
            self.http_client().clone(),
        );
        let config = self.wx_ma_config();
        crate::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            self,
            &executor,
            &url_business::qrcode::get_wxacode_url(config.as_ref()),
            body.to_string(),
        )
        .await
    }

    /// 获取小程序码并保存为临时文件（对应 Java `createWxaCode` 全参版本）。
    async fn create_wxa_code(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let bytes = self
            .create_wxa_code_bytes(path, env_version, width, auto_color, line_color, is_hyaline)
            .await?;
        self.save_qrcode_file(&bytes, None).await
    }

    /// 获取小程序码并保存到指定目录（对应 Java
    /// `createWxaCode(String, String, int, String, boolean, WxMaCodeLineColor, boolean)`）。
    async fn create_wxa_code_to_path(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        file_path: &str,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let bytes = self
            .create_wxa_code_bytes(path, env_version, width, auto_color, line_color, is_hyaline)
            .await?;
        self.save_qrcode_file(&bytes, Some(file_path)).await
    }

    /// 获取小程序码（对应 Java `createWxaCode(String, int)`，env_version 默认
    /// release、autoColor 默认 true、isHyaline 默认 false、lineColor 默认黑色）。
    async fn create_wxa_code_default(
        &self,
        path: &str,
        width: i32,
    ) -> Result<String, WxErrorException> {
        self.create_wxa_code(path, None, width, true, None, false)
            .await
    }

    /// 获取小程序码（对应 Java `createWxaCode(String)`，width 默认 430）。
    async fn create_wxa_code_default_simple(&self, path: &str) -> Result<String, WxErrorException> {
        self.create_wxa_code(path, None, 430, true, None, false)
            .await
    }

    /// 获取小程序码并保存到指定目录（对应 Java `createWxaCode(String, int, String)`）。
    async fn create_wxa_code_width_to_path(
        &self,
        path: &str,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        self.create_wxa_code_to_path(path, None, width, file_path, true, None, false)
            .await
    }

    /// 获取小程序码并保存到指定目录（对应 Java `createWxaCode(String, String)`，
    /// width 默认 430）。
    async fn create_wxa_code_simple_to_path(
        &self,
        path: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        self.create_wxa_code_to_path(path, None, 430, file_path, true, None, false)
            .await
    }

    /// 获取不限制数量的小程序码字节（对应 Java `createWxaCodeUnlimitBytes` 全参版本）。
    ///
    /// POST `/wxa/getwxacodeunlimit`，请求体：`scene`/`page`/`check_path`/
    /// `env_version`（可空，Java 传 null 时 gson 省略该字段）/`width`/
    /// `auto_color`/`is_hyaline`/`line_color`（可空时省略）。
    async fn create_wxa_code_unlimit_bytes(
        &self,
        scene: &str,
        page: &str,
        check_path: bool,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<Vec<u8>, WxErrorException> {
        let mut body = serde_json::Map::new();
        body.insert("scene".to_string(), serde_json::json!(scene));
        body.insert("page".to_string(), serde_json::json!(page));
        body.insert("check_path".to_string(), serde_json::json!(check_path));
        // Java：envVersion 可空（gson 省略 null 字段）
        if let Some(ev) = env_version {
            body.insert("env_version".to_string(), serde_json::json!(ev));
        }
        body.insert("width".to_string(), serde_json::json!(width));
        body.insert("auto_color".to_string(), serde_json::json!(auto_color));
        body.insert("is_hyaline".to_string(), serde_json::json!(is_hyaline));
        if let Some(lc) = line_color {
            body.insert(
                "line_color".to_string(),
                serde_json::to_value(lc).map_err(|e| WxErrorException::Serde(e.to_string()))?,
            );
        }
        let executor = crate::api::r#impl::base_wx_ma_service_impl::QrcodeBytesRequestExecutor::new(
            self.http_client().clone(),
        );
        let config = self.wx_ma_config();
        crate::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            self,
            &executor,
            &url_business::qrcode::get_wxacode_unlimit_url(config.as_ref()),
            serde_json::Value::Object(body).to_string(),
        )
        .await
    }

    /// 获取不限制数量的小程序码并保存为临时文件（对应 Java
    /// `createWxaCodeUnlimit` 全参版本）。
    async fn create_wxa_code_unlimit(
        &self,
        scene: &str,
        page: &str,
        check_path: bool,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let bytes = self
            .create_wxa_code_unlimit_bytes(
                scene,
                page,
                check_path,
                env_version,
                width,
                auto_color,
                line_color,
                is_hyaline,
            )
            .await?;
        self.save_qrcode_file(&bytes, None).await
    }

    /// 获取不限制数量的小程序码并保存到指定目录（对应 Java
    /// `createWxaCodeUnlimit(String, String, String, boolean, String, int, boolean,
    /// WxMaCodeLineColor, boolean)`）。
    async fn create_wxa_code_unlimit_to_path(
        &self,
        scene: &str,
        page: &str,
        file_path: &str,
        check_path: bool,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let bytes = self
            .create_wxa_code_unlimit_bytes(
                scene,
                page,
                check_path,
                env_version,
                width,
                auto_color,
                line_color,
                is_hyaline,
            )
            .await?;
        self.save_qrcode_file(&bytes, Some(file_path)).await
    }

    /// 获取不限制数量的小程序码（对应 Java `createWxaCodeUnlimit(String, String)`，
    /// checkPath 默认 true、envVersion 默认 release、width 默认 430、
    /// autoColor 默认 true、isHyaline 默认 false）。
    async fn create_wxa_code_unlimit_default(
        &self,
        scene: &str,
        page: &str,
    ) -> Result<String, WxErrorException> {
        self.create_wxa_code_unlimit(scene, page, true, None, 430, true, None, false)
            .await
    }

    /// 获取不限制数量的小程序码并保存到指定目录（对应 Java
    /// `createWxaCodeUnlimit(String, String, String)`）。
    async fn create_wxa_code_unlimit_default_to_path(
        &self,
        scene: &str,
        page: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        self.create_wxa_code_unlimit_to_path(
            scene, page, file_path, true, None, 430, true, None, false,
        )
        .await
    }

    // ---- 订阅消息域（对应 Java WxMaSubscribeService） ----

    /// 获取订阅消息公共模板标题列表（对应 Java `getPubTemplateTitleList`）。
    ///
    /// GET `/wxaapi/newtmpl/getpubtemplatetitles?ids=&start=&limit=`（ids 以
    /// `,` 连接）。
    async fn get_pub_template_title_list(
        &self,
        ids: &[&str],
        start: i32,
        limit: i32,
    ) -> Result<PubTemplateTitleListResult, WxErrorException> {
        let query = format!("ids={}&start={}&limit={}", ids.join(","), start, limit);
        let config = self.wx_ma_config();
        let response = self
            .get(
                &url_business::subscribe::get_pub_template_title_list_url(config.as_ref()),
                &query,
            )
            .await?;
        serde_json::from_str::<PubTemplateTitleListResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取模板标题下的关键词列表（对应 Java `getPubTemplateKeyWordsById(String)`）。
    async fn get_pub_template_keywords_by_id(
        &self,
        id: &str,
    ) -> Result<Vec<PubTemplateKeyword>, WxErrorException> {
        let config = self.wx_ma_config();
        let response = self
            .get(
                &url_business::subscribe::get_pub_template_keywords_by_id_url(config.as_ref()),
                &format!("tid={id}"),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        serde_json::from_value(json.get("data").cloned().unwrap_or_default())
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 组合模板并添加至账号下的个人模板库（对应 Java `addTemplate(String,
    /// List<Integer>, String)`）。
    ///
    /// POST `/wxaapi/newtmpl/addtemplate`，返回 `priTmplId`。
    async fn add_template(
        &self,
        id: &str,
        keyword_id_list: &[i32],
        scene_desc: &str,
    ) -> Result<String, WxErrorException> {
        let body = serde_json::json!({
            "tid": id,
            "kidList": keyword_id_list,
            "sceneDesc": scene_desc,
        });
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::subscribe::add_template_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("priTmplId")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "priTmplId 字段缺失"))
    }

    /// 获取当前账号下的个人模板列表（对应 Java `getTemplateList()`）。
    async fn get_template_list(&self) -> Result<Vec<TemplateInfo>, WxErrorException> {
        let config = self.wx_ma_config();
        let response = self
            .get(
                &url_business::subscribe::template_list_url(config.as_ref()),
                "",
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        serde_json::from_value(json.get("data").cloned().unwrap_or_default())
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 删除账号下的某个模板（对应 Java `delTemplate(String)`，成功返回 true）。
    async fn del_template(&self, template_id: &str) -> Result<bool, WxErrorException> {
        let body = serde_json::json!({ "priTmplId": template_id });
        let config = self.wx_ma_config();
        self.post(
            &url_business::subscribe::del_template_url(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(true)
    }

    /// 获取小程序账号的类目（对应 Java `getCategory()`）。
    async fn get_category(&self) -> Result<Vec<CategoryData>, WxErrorException> {
        let config = self.wx_ma_config();
        let response = self
            .get(
                &url_business::subscribe::get_category_url(config.as_ref()),
                "",
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        serde_json::from_value(json.get("data").cloned().unwrap_or_default())
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 发送订阅消息（对应 Java `sendSubscribeMsg(WxMaSubscribeMessage)`）。
    ///
    /// POST `/cgi-bin/message/subscribe/send`，请求体
    /// `touser`/`template_id`/`page`/`data`/`miniprogram_state`/`lang`；
    /// Java 的显式 errcode 校验已被执行引擎覆盖（同一语义）。
    async fn send_subscribe_msg(
        &self,
        message: &WxMaSubscribeMessage,
    ) -> Result<(), WxErrorException> {
        let body =
            serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        self.post(
            &url_business::subscribe::subscribe_msg_send_url(config.as_ref()),
            &body,
        )
        .await?;
        Ok(())
    }

    /// 激活与更新服务卡片（对应 Java `setUserNotify(WxMaServiceNotifyRequest)`）。
    async fn set_user_notify(
        &self,
        request: &WxMaServiceNotifyRequest,
    ) -> Result<(), WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        self.post(
            &url_business::subscribe::set_user_notify_url(config.as_ref()),
            &body,
        )
        .await?;
        Ok(())
    }

    /// 更新服务卡片扩展信息（对应 Java `setUserNotifyExt(WxMaServiceNotifyExtRequest)`）。
    async fn set_user_notify_ext(
        &self,
        request: &WxMaServiceNotifyExtRequest,
    ) -> Result<(), WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        self.post(
            &url_business::subscribe::set_user_notify_ext_url(config.as_ref()),
            &body,
        )
        .await?;
        Ok(())
    }

    /// 查询服务卡片状态（对应 Java `getUserNotify(WxMaGetUserNotifyRequest)`）。
    async fn get_user_notify(
        &self,
        request: &WxMaGetUserNotifyRequest,
    ) -> Result<WxMaGetUserNotifyResult, WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::subscribe::get_user_notify_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str::<WxMaGetUserNotifyResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    // ---- 消息域（对应 Java WxMaMsgService） ----

    /// 发送客服消息（对应 Java `sendKefuMsg(WxMaKefuMessage)`）。
    ///
    /// 发送成功即返回 true（Java `responseContent != null`）。
    async fn send_kefu_msg(&self, message: &WxMaKefuMessage) -> Result<bool, WxErrorException> {
        let body =
            serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        self.post(
            &url_business::msg::kefu_message_send_url(config.as_ref()),
            &body,
        )
        .await?;
        Ok(true)
    }

    /// 下发模板消息（对应 Java `sendUniformMsg(WxMaUniformMessage)`）。
    async fn send_uniform_msg(&self, message: &WxMaUniformMessage) -> Result<(), WxErrorException> {
        let body =
            serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        self.post(
            &url_business::msg::uniform_msg_send_url(config.as_ref()),
            &body,
        )
        .await?;
        Ok(())
    }

    /// 创建被分享动态消息的 activity_id（对应 Java
    /// `createUpdatableMessageActivityId()`，返回完整响应 JSON）。
    async fn create_updatable_message_activity_id(
        &self,
    ) -> Result<serde_json::Value, WxErrorException> {
        let config = self.wx_ma_config();
        let response = self
            .get(
                &url_business::msg::activity_id_create_url(config.as_ref()),
                "",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 修改被分享的动态消息（对应 Java `setUpdatableMsg(WxMaUpdatableMsg)`）。
    async fn set_updatable_msg(&self, msg: &WxMaUpdatableMsg) -> Result<(), WxErrorException> {
        let body =
            serde_json::to_string(msg).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        self.post(
            &url_business::msg::updatable_msg_send_url(config.as_ref()),
            &body,
        )
        .await?;
        Ok(())
    }

    // ---- 服务端网络域（对应 Java WxMaInternetService） ----

    /// 获取用户加密 key（指定签名，对应 Java
    /// `getUserEncryptKey(String, String, String)`）。
    ///
    /// POST `/wxa/business/getuserencryptkey?openid=&signature=&sig_method=`，
    /// 请求体为空字符串（Java `post(url, "")`）。
    async fn get_user_encrypt_key_with_signature(
        &self,
        openid: &str,
        signature: &str,
        sig_method: &str,
    ) -> Result<WxMaInternetResponse, WxErrorException> {
        let config = self.wx_ma_config();
        let url = format!(
            "{}?openid={openid}&signature={signature}&sig_method={sig_method}",
            url_business::internet::get_user_encrypt_key_url(config.as_ref())
        );
        let response = self.post(&url, "").await?;
        serde_json::from_str::<WxMaInternetResponse>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取用户加密 key（对应 Java `getUserEncryptKey(String, String)`）。
    ///
    /// signature 为以 **Base64 解码后的 sessionKey** 为密钥对空串做
    /// HmacSHA256 的十六进制大写结果（Java `sha256("", sessionKey)`；
    /// 与 `SignUtils` 的原始 key 字节语义不同）。
    async fn get_user_encrypt_key(
        &self,
        openid: &str,
        session_key: &str,
    ) -> Result<WxMaInternetResponse, WxErrorException> {
        let key = base64::engine::general_purpose::STANDARD
            .decode(session_key)
            .map_err(|_| WxErrorException::from_code(-99, "签名错误"))?;
        let mut mac = Hmac::<Sha256>::new_from_slice(&key)
            .map_err(|_| WxErrorException::from_code(-99, "签名错误"))?;
        mac.update(b"");
        let signature = hex::encode_upper(mac.finalize().into_bytes());
        let config = self.wx_ma_config();
        let url = format!(
            "{}?sig_method=hmac_sha256&openid={openid}&signature={signature}",
            url_business::internet::get_user_encrypt_key_url(config.as_ref())
        );
        let response = self.post(&url, "").await?;
        serde_json::from_str::<WxMaInternetResponse>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    // ---- 链接域（对应 Java WxMaLinkService） ----

    /// 生成 URL Link（对应 Java `generateUrlLink(GenerateUrlLinkRequest)`）。
    ///
    /// 响应含 `url_link` 字段时返回其值，否则抛 `无url_link`（Java
    /// `WxErrorException("无url_link")`）。
    async fn generate_url_link(
        &self,
        request: &GenerateUrlLinkRequest,
    ) -> Result<String, WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::link::generate_url_link_url(config.as_ref()),
                &body,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("url_link")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "无url_link"))
    }

    /// 生成短链接（对应 Java `generateShortLink(GenerateShortLinkRequest)`）。
    ///
    /// POST `/wxa/genwxashortlink`；响应含 `link` 字段时返回其值，否则抛
    /// `无link`（Java `WxErrorException("无link")`）。
    async fn generate_short_link(
        &self,
        request: &GenerateShortLinkRequest,
    ) -> Result<String, WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::link::generate_short_link_url(config.as_ref()),
                &body,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("link")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "无link"))
    }

    /// 查询 URL Link 信息（对应 Java `queryUrlLink(QueryUrlLinkRequest)`）。
    async fn query_url_link(
        &self,
        request: &QueryUrlLinkRequest,
    ) -> Result<QueryUrlLinkResponse, WxErrorException> {
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = self.wx_ma_config();
        let response = self
            .post(
                &url_business::link::query_url_link_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str::<QueryUrlLinkResponse>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 保存二维码/小程序码字节到临时文件（对应 Java
    /// `FileUtils.createTmpFile(inputStream, UUID, "jpg", dir)`）。
    ///
    /// - `file_path = None`：系统临时目录 `wxjava-temp` 下随机名 `.jpg`
    /// - `file_path = Some(dir)`：写入该目录（Java 目录语义），返回实际路径
    ///
    /// 文件名随机后缀由 `FileUtils::create_tmp_file` 保证唯一
    /// （ADAPTED：Java 以 UUID 命名，语义等价）。
    async fn save_qrcode_file(
        &self,
        bytes: &[u8],
        file_path: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let dir = file_path.map(std::path::Path::new);
        FileUtils::create_tmp_file(bytes, "qrcode", "jpg", dir)
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| WxErrorException::Io(e.to_string()))
    }
}
