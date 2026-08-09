//! 公众号服务门面。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpService` + `BaseWxMpServiceImpl`。
//! Java 三层继承链（Impl → HttpComponentsImpl → Base）在 Rust 以
//! trait 默认实现 + 组合表达（见 ARCHITECTURE.md 设计原则）：本 trait 携带
//! Base 的全部默认实现（token/ticket 双检锁、执行引擎、签名、通用 API），
//! 具体实现仅需提供配置存储与 HTTP 客户端。

use std::sync::Arc;

use async_trait::async_trait;

use wx_rust_common::bean::{WxJsapiSignature, WxNetCheckResult};
use wx_rust_common::enums::TicketType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::RandomUtils;
use wx_rust_common::util::crypto::Sha1;
use wx_rust_common::util::http::{SimpleGetRequestExecutor, SimplePostRequestExecutor};

use crate::api::{
    WxMpAiOpenService, WxMpCardService, WxMpCommentService, WxMpDataCubeService, WxMpDeviceService,
    WxMpDraftService, WxMpFreePublishService, WxMpGuideBuyerService, WxMpGuideMassedJobService,
    WxMpGuideMaterialService, WxMpGuideService, WxMpGuideTagService, WxMpImgProcService,
    WxMpKefuService, WxMpMarketingService, WxMpMassMessageService, WxMpMaterialService,
    WxMpMemberCardService, WxMpMenuService, WxMpMerchantInvoiceService, WxMpOcrService,
    WxMpQrcodeService, WxMpReimburseInvoiceService, WxMpShakeService, WxMpStoreService,
    WxMpSubscribeMsgService, WxMpTemplateMsgService, WxMpUserBlacklistService, WxMpUserService,
    WxMpUserTagService, WxMpWifiService,
};
use crate::bean::result::{WxMpCurrentAutoReplyInfo, WxMpShortKeyResult};
use crate::config::WxMpConfigStorage;
use crate::enums::wx_mp_api_url::other as api_url;

/// 公众号服务门面。
#[async_trait]
pub trait WxMpService: Send + Sync {
    /// 当前公众号配置存储。
    fn wx_mp_config_storage(&self) -> Arc<dyn WxMpConfigStorage>;

    /// HTTP 客户端。
    fn http_client(&self) -> &reqwest::Client;

    // ---- 子服务（未实现子域返回 None，随批次补齐） ----

    /// 菜单服务。
    fn menu_service(&self) -> Option<Arc<dyn WxMpMenuService>> {
        None
    }

    /// 模板消息服务。
    fn template_msg_service(&self) -> Option<Arc<dyn WxMpTemplateMsgService>> {
        None
    }

    /// 二维码服务。
    fn qrcode_service(&self) -> Option<Arc<dyn WxMpQrcodeService>> {
        None
    }

    /// 客服服务。
    fn kefu_service(&self) -> Option<Arc<dyn WxMpKefuService>> {
        None
    }

    /// 用户服务。
    fn user_service(&self) -> Option<Arc<dyn WxMpUserService>> {
        None
    }

    /// 用户标签服务。
    fn user_tag_service(&self) -> Option<Arc<dyn WxMpUserTagService>> {
        None
    }

    /// 用户黑名单服务。
    fn user_blacklist_service(&self) -> Option<Arc<dyn WxMpUserBlacklistService>> {
        None
    }

    /// 门店服务。
    fn store_service(&self) -> Option<Arc<dyn WxMpStoreService>> {
        None
    }

    /// 评论服务。
    fn comment_service(&self) -> Option<Arc<dyn WxMpCommentService>> {
        None
    }

    /// 数据统计服务。
    fn data_cube_service(&self) -> Option<Arc<dyn WxMpDataCubeService>> {
        None
    }

    /// Wi-Fi 服务。
    fn wifi_service(&self) -> Option<Arc<dyn WxMpWifiService>> {
        None
    }

    /// 草稿箱服务。
    fn draft_service(&self) -> Option<Arc<dyn WxMpDraftService>> {
        None
    }

    /// 发布能力服务。
    fn free_publish_service(&self) -> Option<Arc<dyn WxMpFreePublishService>> {
        None
    }

    /// 设备服务。
    fn device_service(&self) -> Option<Arc<dyn WxMpDeviceService>> {
        None
    }

    /// 群发消息服务。
    fn mass_message_service(&self) -> Option<Arc<dyn WxMpMassMessageService>> {
        None
    }

    /// 公众号GuideBuyer服务。
    fn guide_buyer_service(&self) -> Option<Arc<dyn WxMpGuideBuyerService>> {
        None
    }

    /// 公众号GuideTag服务。
    fn guide_tag_service(&self) -> Option<Arc<dyn WxMpGuideTagService>> {
        None
    }

    /// 公众号GuideMaterial服务。
    fn guide_material_service(&self) -> Option<Arc<dyn WxMpGuideMaterialService>> {
        None
    }

    /// 公众号GuideMassedJob服务。
    fn guide_massed_job_service(&self) -> Option<Arc<dyn WxMpGuideMassedJobService>> {
        None
    }

    /// 公众号Material服务。
    fn material_service(&self) -> Option<Arc<dyn WxMpMaterialService>> {
        None
    }

    /// 公众号Shake服务。
    fn shake_service(&self) -> Option<Arc<dyn WxMpShakeService>> {
        None
    }

    /// 公众号Card服务。
    fn card_service(&self) -> Option<Arc<dyn WxMpCardService>> {
        None
    }

    /// 公众号MemberCard服务。
    fn member_card_service(&self) -> Option<Arc<dyn WxMpMemberCardService>> {
        None
    }

    /// 公众号Guide服务。
    fn guide_service(&self) -> Option<Arc<dyn WxMpGuideService>> {
        None
    }

    /// 公众号Marketing服务。
    fn marketing_service(&self) -> Option<Arc<dyn WxMpMarketingService>> {
        None
    }

    /// 公众号SubscribeMsg服务。
    fn subscribe_msg_service(&self) -> Option<Arc<dyn WxMpSubscribeMsgService>> {
        None
    }

    /// 公众号AiOpen服务。
    fn ai_open_service(&self) -> Option<Arc<dyn WxMpAiOpenService>> {
        None
    }

    /// 公众号Ocr服务。
    fn ocr_service(&self) -> Option<Arc<dyn WxMpOcrService>> {
        None
    }

    /// 公众号ImgProc服务。
    fn img_proc_service(&self) -> Option<Arc<dyn WxMpImgProcService>> {
        None
    }

    /// 公众号ReimburseInvoice服务。
    fn reimburse_invoice_service(&self) -> Option<Arc<dyn WxMpReimburseInvoiceService>> {
        None
    }

    /// 公众号MerchantInvoice服务。
    fn merchant_invoice_service(&self) -> Option<Arc<dyn WxMpMerchantInvoiceService>> {
        None
    }

    // ---- 核心能力（对应 BaseWxMpServiceImpl） ----

    /// 获取 access_token（对应 Java `getAccessToken()`）。
    async fn get_access_token(&self) -> Result<String, WxErrorException> {
        self.get_access_token_with_force(false).await
    }

    /// 获取 access_token（可强制刷新）。
    ///
    /// 对应 Java `getAccessToken(boolean forceRefresh)`：双检锁 +
    /// tryLock(100ms) 轮询 + 3 秒超时；稳定版接口按配置切换。
    async fn get_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_mp_config_storage();
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

    /// 获取指定类型 ticket（对应 Java `getTicket(TicketType, boolean)`）。
    async fn get_ticket(
        &self,
        ticket_type: TicketType,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_mp_config_storage();
        if force_refresh {
            config.expire_ticket(ticket_type);
        }

        if config.is_ticket_expired(ticket_type) {
            let lock = config.ticket_lock(ticket_type);
            let _guard = lock.lock().await;
            if config.is_ticket_expired(ticket_type) {
                let url = format!(
                    "{}{}",
                    api_url::get_ticket_url(config.as_ref()),
                    ticket_type.value()
                );
                let response = self.get(&url, "").await?;
                let json: serde_json::Value = serde_json::from_str(&response)
                    .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                let jsapi_ticket = json
                    .get("ticket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| WxErrorException::from_code(-99, "ticket 字段缺失"))?
                    .to_string();
                let expires_in =
                    json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                config.update_ticket(ticket_type, &jsapi_ticket, expires_in);
            }
        }

        config
            .ticket(ticket_type)
            .ok_or_else(|| WxErrorException::from_code(-99, "ticket 为空"))
    }

    /// 获取 jsapi ticket（对应 Java `getJsapiTicket(boolean)`）。
    async fn get_jsapi_ticket(&self, force_refresh: bool) -> Result<String, WxErrorException> {
        self.get_ticket(TicketType::Jsapi, force_refresh).await
    }

    /// 创建 jsapi 签名（对应 Java `createJsapiSignature(String)`）。
    async fn create_jsapi_signature(
        &self,
        url: &str,
    ) -> Result<WxJsapiSignature, WxErrorException> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let random_str = RandomUtils::get_random_str();
        let jsapi_ticket = self.get_jsapi_ticket(false).await?;
        let signature = Sha1::digest_with_amp(&[
            &format!("jsapi_ticket={jsapi_ticket}"),
            &format!("noncestr={random_str}"),
            &format!("timestamp={timestamp}"),
            &format!("url={url}"),
        ])
        .map_err(|e| WxErrorException::Runtime(wx_rust_common::error::WxRuntimeError::new(e)))?;

        Ok(WxJsapiSignature {
            app_id: self.wx_mp_config_storage().app_id().to_string(),
            nonce_str: random_str,
            timestamp,
            url: url.to_string(),
            signature,
        })
    }

    /// 校验签名（对应 Java `checkSignature(String, String, String)`）。
    fn check_signature(&self, timestamp: &str, nonce: &str, signature: &str) -> bool {
        let config = self.wx_mp_config_storage();
        let token = config.token().unwrap_or_default();
        match Sha1::digest(&[token, timestamp.to_string().as_str(), nonce]) {
            Ok(s) => s == signature,
            Err(_) => false,
        }
    }

    /// 长链接转短链接（对应 Java `shortUrl`）。
    async fn short_url(&self, long_url: &str) -> Result<String, WxErrorException> {
        if long_url.contains("&access_token=") {
            return Err(WxErrorException::from_code(
                -99,
                "要转换的网址中存在非法字符｛&access_token=｝，会导致微信接口报错，属于微信bug，请调整地址，否则不建议使用此方法！",
            ));
        }
        let body = serde_json::json!({
            "action": "long2short",
            "long_url": long_url,
        });
        let response = self
            .post(
                &api_url::shorturl_api_url(self.wx_mp_config_storage().as_ref()),
                &body.to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("short_url")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "short_url 字段缺失"))
    }

    /// 获取微信服务器 IP 地址（对应 Java `getCallbackIP`）。
    async fn get_callback_ip(&self) -> Result<Vec<String>, WxErrorException> {
        let response = self
            .get(
                &api_url::get_callback_ip_url(self.wx_mp_config_storage().as_ref()),
                "",
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(json
            .get("ip_list")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default())
    }

    /// 网络检测（对应 Java `netCheck(String, String)`）。
    async fn net_check(
        &self,
        action: &str,
        operator: &str,
    ) -> Result<WxNetCheckResult, WxErrorException> {
        let body = serde_json::json!({
            "action": action,
            "check_operator": operator,
        });
        let response = self
            .post(
                &api_url::netcheck_url(self.wx_mp_config_storage().as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str::<WxNetCheckResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取公众号的自动回复规则（对应 Java `getCurrentAutoReplyInfo`）。
    async fn get_current_auto_reply_info(
        &self,
    ) -> Result<WxMpCurrentAutoReplyInfo, WxErrorException> {
        let response = self
            .get(
                &api_url::get_current_autoreply_info_url(self.wx_mp_config_storage().as_ref()),
                "",
            )
            .await?;
        WxMpCurrentAutoReplyInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 清空 api 调用次数（对应 Java `clearQuota(String)`）。
    async fn clear_quota(&self, appid: &str) -> Result<(), WxErrorException> {
        let body = serde_json::json!({ "appid": appid });
        self.post(
            &api_url::clear_quota_url(self.wx_mp_config_storage().as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    /// 生成短 key（对应 Java `genShorten`）。
    async fn gen_shorten(
        &self,
        long_data: &str,
        expire_seconds: i32,
    ) -> Result<String, WxErrorException> {
        let body = serde_json::json!({
            "long_data": long_data,
            "expire_seconds": expire_seconds,
        });
        let response = self
            .post(
                &api_url::gen_shorten_url(self.wx_mp_config_storage().as_ref()),
                &body.to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("short_key")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "short_key 字段缺失"))
    }

    /// 解析短 key（对应 Java `fetchShorten`）。
    async fn fetch_shorten(&self, short_key: &str) -> Result<WxMpShortKeyResult, WxErrorException> {
        let body = serde_json::json!({ "short_key": short_key });
        let response = self
            .post(
                &api_url::fetch_shorten_url(self.wx_mp_config_storage().as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpShortKeyResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 构建扫码连接地址（对应 Java `buildQrConnectUrl`）。
    fn build_qr_connect_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String {
        let config = self.wx_mp_config_storage();
        // RFC 3986：保留 unreserved 字符（与 Java URIUtil.encodeURIComponent 一致）
        const UNRESERVED: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
            .add(b' ')
            .add(b'"')
            .add(b'#')
            .add(b'%')
            .add(b'<')
            .add(b'>')
            .add(b'?')
            .add(b'`')
            .add(b'{')
            .add(b'}')
            .add(b'/')
            .add(b':')
            .add(b';')
            .add(b'=')
            .add(b'@')
            .add(b'[')
            .add(b'\\')
            .add(b']')
            .add(b'^')
            .add(b'|')
            .add(b'&')
            .add(b'+')
            .add(b'$')
            .add(b',');
        let encoded = percent_encoding::utf8_percent_encode(redirect_uri, UNRESERVED).to_string();
        let host = api_url::qrconnect_url(config.as_ref());
        format!(
            "{host}?appid={}&redirect_uri={}&response_type=code&scope={}&state={}#wechat_redirect",
            config.app_id(),
            encoded,
            scope,
            state.trim(),
        )
    }

    /// GET 请求（对应 Java `get(String, String)`）。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_mp_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            query_param.to_string(),
        )
        .await
    }

    /// POST 请求（对应 Java `post(String, String)`）。
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_mp_service_impl::execute_with_retry(
            self,
            &executor,
            url,
            post_data.to_string(),
        )
        .await
    }

    /// 提取 access token（对应 Java `extractAccessToken`）。
    fn extract_access_token(&self, result_content: &str) -> Result<String, WxErrorException> {
        let config = self.wx_mp_config_storage();
        let error = wx_rust_common::error::WxError::from_json_with_type(
            result_content,
            Some(wx_rust_common::enums::WxType::Mp),
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
    async fn do_get_access_token_request(&self) -> Result<String, WxErrorException> {
        let config = self.wx_mp_config_storage();
        let url = api_url::get_access_token_url(config.as_ref());
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
        let config = self.wx_mp_config_storage();
        let url = api_url::get_stable_access_token_url(config.as_ref());
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
}
