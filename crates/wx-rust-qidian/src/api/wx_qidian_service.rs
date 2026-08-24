//! 腾讯企点服务门面。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api.WxQidianService` +
//! `BaseWxQidianServiceImpl`。Java 三层继承链（Impl → HttpComponentsImpl →
//! Base）在 Rust 以 trait 默认实现 + 组合表达（与 mp/miniapp 模块同一
//! 设计原则）：本 trait 携带 Base 的全部默认实现（access_token 双检锁、
//! ticket/jsapi 签名、GET/POST 执行引擎、多配置切换入口），具体实现仅需
//! 提供配置存储、HTTP 客户端与重试参数。
//!
//! 说明：
//! - Java 泛型 `execute(RequestExecutor, String, E)` 以泛型自由函数
//!   `crate::api::impl::base_wx_qidian_service_impl::execute_with_retry`
//!   承载（trait 无法携带泛型方法，与 mp/miniapp 同一约定）。
//! - Java 重载（`get(String,...)`/`get(WxQidianApiUrl,...)` 等）以
//!   `_by_url` 后缀区分（ADAPTED：Rust 无重载）。
//! - `getRequestHttp()`（多 HTTP 后端抽象）以 reqwest 客户端承载
//!   （`PLATFORM_NA`，见台账）。

use std::sync::Arc;

use async_trait::async_trait;

use serde_json::Value;

use wx_rust_common::bean::{WxJsapiSignature, WxNetCheckResult};
use wx_rust_common::enums::TicketType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::RandomUtils;
use wx_rust_common::util::crypto::Sha1;

use crate::api::{WxQidianCallDataService, WxQidianDialService};
use crate::config::WxQidianConfigStorage;
use crate::enums::ApiUrl;
use crate::enums::wx_qidian_api_url::other;

/// 获取 access_token 超时（毫秒，对应 mp/miniapp 同一 3 秒约定）。
const ACCESS_TOKEN_TIMEOUT_MILLIS: u64 = 3000;

/// 腾讯企点服务门面。
#[async_trait]
pub trait WxQidianService: Send + Sync {
    /// 当前企点配置存储（对应 Java `getWxMpConfigStorage()`）。
    fn config_storage(&self) -> Arc<dyn WxQidianConfigStorage>;

    /// 设置企点配置存储（对应 Java `setWxMpConfigStorage`，兼容老版本）。
    fn set_config_storage(&self, config: Arc<dyn WxQidianConfigStorage>);

    /// 动态添加企点配置（对应 Java `addConfigStorage`）。
    fn add_config_storage(&self, mp_id: &str, config_storage: Arc<dyn WxQidianConfigStorage>);

    /// 动态移除企点配置（对应 Java `removeConfigStorage`）。
    fn remove_config_storage(&self, mp_id: &str);

    /// 注入多个企点配置（对应 Java `setMultiConfigStorages(Map)`）。
    fn set_multi_config_storages(
        &self,
        config_storages: Vec<(String, Arc<dyn WxQidianConfigStorage>)>,
    );

    /// 注入多个企点配置并指定默认（对应 Java
    /// `setMultiConfigStorages(Map, String defaultMpId)`）。
    fn set_multi_config_storages_with_default(
        &self,
        config_storages: Vec<(String, Arc<dyn WxQidianConfigStorage>)>,
        default_mp_id: &str,
    );

    /// 进行相应的企点切换（对应 Java `switchover`）。
    fn switchover(&self, mp_id: &str) -> bool;

    /// 进行相应的企点切换（对应 Java `switchoverTo`）；不存在时返回错误
    /// （对应 Java 抛 `WxRuntimeException`）。
    fn switchover_to(&self, mp_id: &str) -> Result<(), String>;

    /// HTTP 客户端（reqwest，克隆廉价；对应 Java `getRequestHttp()` 的
    /// HTTP 能力）。
    fn http_client(&self) -> reqwest::Client;

    /// 重试间隔（毫秒，对应 Java `retrySleepMillis` 字段）。
    fn retry_sleep_millis(&self) -> i32;

    /// 最大重试次数（对应 Java `maxRetryTimes` 字段）。
    fn max_retry_times(&self) -> i32;

    /// 设置重试间隔（对应 Java `setRetrySleepMillis`）。
    fn set_retry_sleep_millis(&self, retry_sleep_millis: i32);

    /// 设置最大重试次数（对应 Java `setMaxRetryTimes`）。
    fn set_max_retry_times(&self, max_retry_times: i32);

    // ---- 子服务（对应 Java `getDialService()`/`getCallDataService()`；
    // 默认返回 None，由 WxQidianServiceImpl 覆写） ----

    /// 基础话务服务。
    fn dial_service(&self) -> Option<Arc<dyn WxQidianDialService>> {
        None
    }

    /// 通话数据服务。
    fn call_data_service(&self) -> Option<Arc<dyn WxQidianCallDataService>> {
        None
    }

    // ---- 核心能力（对应 Java WxQidianService + BaseWxQidianServiceImpl） ----

    /// 验证消息确实来自微信服务器（对应 Java `checkSignature`）。
    ///
    /// `sha1(token + timestamp + nonce 排序拼接)` 与签名比对（对应 Java
    /// `SHA1.gen`）；任何异常（含 token 未配置）返回 false。
    fn check_signature(&self, timestamp: &str, nonce: &str, signature: &str) -> bool {
        let config = self.config_storage();
        let Some(token) = config.token() else {
            return false;
        };
        match Sha1::digest(&[token, timestamp, nonce]) {
            Ok(digest) => digest == signature,
            Err(_) => false,
        }
    }

    /// 获取 access_token，不强制刷新（对应 Java `getAccessToken()`）。
    async fn get_access_token(&self) -> Result<String, WxErrorException> {
        self.get_access_token_with_force(false).await
    }

    /// 获取 access_token（可强制刷新）。
    ///
    /// 对应 Java `getAccessToken(boolean)`：双检锁 + tryLock(100ms) 轮询 +
    /// 3 秒超时；企点 token 接口为
    /// `https://api.qidian.qq.com/cgi-bin/token?grant_type=client_credential
    /// &appid=%s&secret=%s`（与 mp 同构）。
    async fn get_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        if !force_refresh && !config.is_access_token_expired() {
            return config
                .access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
        }

        let lock = config.access_token_lock();
        let timeout_at = std::time::Instant::now()
            + std::time::Duration::from_millis(ACCESS_TOKEN_TIMEOUT_MILLIS);
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

        let response = self.do_get_access_token_request().await?;
        extract_access_token(config.as_ref(), &response)
    }

    /// 通过网络请求获取 access_token（对应 Java 抽象方法
    /// `doGetAccessTokenRequest`，由 HttpComponentsImpl 等实现）。
    async fn do_get_access_token_request(&self) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        // 对应 Java `String.format(GET_ACCESS_TOKEN_URL.getUrl(config),
        // appId, secret)`
        let url_template = other::GET_ACCESS_TOKEN_URL.get_url(Some(config.as_ref()));
        let url = format_url(&url_template, &[config.app_id(), config.secret()]);
        let client = self.http_client();
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        resp.text()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))
    }

    /// 获得 ticket，不强制刷新（对应 Java `getTicket(TicketType)`）。
    async fn get_ticket(&self, ticket_type: TicketType) -> Result<String, WxErrorException> {
        self.get_ticket_with_force(ticket_type, false).await
    }

    /// 获得 ticket（可强制刷新）。
    ///
    /// 对应 Java `getTicket(TicketType, boolean)`：双检锁；过期时请求
    /// `/cgi-bin/ticket/getticket?type={code}` 并缓存（预留 200 秒提前
    /// 过期由配置存储承担）。
    async fn get_ticket_with_force(
        &self,
        ticket_type: TicketType,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        if force_refresh {
            config.expire_ticket(ticket_type);
        }

        if config.is_ticket_expired(ticket_type) {
            let lock = config.ticket_lock(ticket_type);
            let _guard = lock.lock().await;
            if config.is_ticket_expired(ticket_type) {
                let url = format!(
                    "{}{}",
                    crate::enums::wx_qidian_api_url::other::GET_TICKET_URL
                        .get_url(Some(config.as_ref())),
                    ticket_type.value()
                );
                let response = self.get_by_url(&url, "").await?;
                let json: serde_json::Value = serde_json::from_str(&response)
                    .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                let ticket = json
                    .get("ticket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| WxErrorException::from_code(-99, "ticket 字段缺失"))?
                    .to_string();
                let expires_in =
                    json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                config.update_ticket(ticket_type, &ticket, expires_in);
            }
        }

        config
            .ticket(ticket_type)
            .ok_or_else(|| WxErrorException::from_code(-99, "ticket 为空"))
    }

    /// 获得 jsapi_ticket，不强制刷新（对应 Java `getJsapiTicket()`）。
    async fn get_jsapi_ticket(&self) -> Result<String, WxErrorException> {
        self.get_jsapi_ticket_with_force(false).await
    }

    /// 获得 jsapi_ticket（可强制刷新，对应 Java
    /// `getJsapiTicket(boolean)`）。
    async fn get_jsapi_ticket_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        self.get_ticket_with_force(TicketType::Jsapi, force_refresh)
            .await
    }

    /// 创建调用 jsapi 时所需要的签名（对应 Java `createJsapiSignature`）。
    ///
    /// 签名 = sha1(`jsapi_ticket=..&noncestr=..&timestamp=..&url=..` 排序
    /// 拼接，对应 Java `SHA1.genWithAmple`）。
    async fn create_jsapi_signature(
        &self,
        url: &str,
    ) -> Result<WxJsapiSignature, WxErrorException> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let random_str = RandomUtils::get_random_str();
        let jsapi_ticket = self.get_jsapi_ticket().await?;
        let signature = Sha1::digest_with_amp(&[
            &format!("jsapi_ticket={jsapi_ticket}"),
            &format!("noncestr={random_str}"),
            &format!("timestamp={timestamp}"),
            &format!("url={url}"),
        ])
        .map_err(|e| WxErrorException::from_code(-99, e))?;
        Ok(WxJsapiSignature::new(
            self.config_storage().app_id(),
            random_str,
            timestamp,
            url,
            signature,
        ))
    }

    /// 长链接转短链接（对应 Java `shortUrl`）。
    ///
    /// 网址含 `&access_token=` 时报错（对应 Java 的微信 bug 提示）。
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
        })
        .to_string();
        let response = self.post(&other::SHORTURL_API_URL, &body).await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("short_url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "short_url 字段缺失"))
    }

    /// 构造第三方使用网站应用授权登录的 url（对应 Java
    /// `buildQrConnectUrl`）。
    ///
    /// redirect_uri 在方法内做 encodeURIComponent 编码（对应 Java
    /// `URIUtil.encodeURIComponent`）；state 空串化（对应 Java
    /// `StringUtils.trimToEmpty`）。
    fn build_qr_connect_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String {
        let config = self.config_storage();
        let url_template = other::QRCONNECT_URL.get_url(Some(config.as_ref()));
        format_url(
            &url_template,
            &[
                config.app_id(),
                &wx_rust_common::util::http::UriUtil::encode_uri_component(redirect_uri),
                scope,
                state.trim(),
            ],
        )
    }

    /// 获取微信服务器 IP 地址（对应 Java `getCallbackIP`）。
    async fn get_callback_ip(&self) -> Result<Vec<String>, WxErrorException> {
        let response = self.get(&other::GET_CALLBACK_IP_URL, "").await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("ip_list")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .ok_or_else(|| WxErrorException::from_code(-99, "ip_list 字段缺失"))
    }

    /// 网络检测（对应 Java `netCheck`）。
    async fn net_check(
        &self,
        action: &str,
        operator: &str,
    ) -> Result<WxNetCheckResult, WxErrorException> {
        let body = serde_json::json!({
            "action": action,
            "check_operator": operator,
        })
        .to_string();
        let response = self.post(&other::NETCHECK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 公众号 api 调用次数清零（对应 Java `clearQuota`）。
    async fn clear_quota(&self, appid: &str) -> Result<(), WxErrorException> {
        let body = serde_json::json!({ "appid": appid }).to_string();
        self.post(&other::CLEAR_QUOTA_URL, &body).await?;
        Ok(())
    }

    // ---- GET/POST 执行引擎（对应 Java `get`/`post` 重载） ----

    /// 企点接口地址 GET（对应 Java `get(WxQidianApiUrl, String)`）。
    async fn get(&self, url: &ApiUrl, query_param: &str) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        let resolved = url.get_url(Some(config.as_ref()));
        self.get_by_url(&resolved, query_param).await
    }

    /// 原始地址 GET（对应 Java `get(String, String)`）。
    ///
    /// 走统一管线 [`wx_rust_common::pipeline::execute_pipeline`]（经
    /// `execute_get_via_pipeline`：-1 指数退避重试（Runtime 变体收束）
    /// + token 失效单次重放；query 拼接语义内联于封装——原
    /// `SimpleGetRequestExecutor` 路径）。
    async fn get_by_url(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_qidian_service_impl::execute_get_via_pipeline(
            self,
            url,
            query_param,
        )
        .await
    }

    /// 企点接口地址 POST（对应 Java `post(WxQidianApiUrl, String)`）。
    async fn post(&self, url: &ApiUrl, post_data: &str) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        let resolved = url.get_url(Some(config.as_ref()));
        self.post_by_url(&resolved, post_data).await
    }

    /// 原始地址 POST（对应 Java `post(String, String)`）。
    ///
    /// 走统一管线（经 `execute_post_via_pipeline`：POST 文本体原样透传 +
    /// -1 指数退避重试（Runtime 变体收束）+ token 失效单次重放——原
    /// `SimplePostRequestExecutor` 路径）。
    async fn post_by_url(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_qidian_service_impl::execute_post_via_pipeline(
            self, url, post_data,
        )
        .await
    }

    /// 企点接口地址 JSON 对象 POST（对应 Java
    /// `post(WxQidianApiUrl, JsonObject)`）。
    async fn post_json(
        &self,
        url: &ApiUrl,
        json_object: &Value,
    ) -> Result<String, WxErrorException> {
        self.post(url, &json_object.to_string()).await
    }

    /// 原始地址 JSON 对象 POST（对应 Java `post(String, JsonObject)`）。
    async fn post_json_by_url(
        &self,
        url: &str,
        json_object: &Value,
    ) -> Result<String, WxErrorException> {
        self.post_by_url(url, &json_object.to_string()).await
    }
}

/// 提取 access token（对应 Java `extractAccessToken`）。
///
/// 先按 `WxError` 校验（`WxType::Mp` 错误表），errcode 非 0 抛错；否则
/// 解析 `WxAccessToken` 并更新配置存储。
fn extract_access_token(
    config: &dyn WxQidianConfigStorage,
    result_content: &str,
) -> Result<String, WxErrorException> {
    let error = wx_rust_common::error::WxError::from_json_with_type(
        result_content,
        Some(wx_rust_common::enums::WxType::Mp),
    );
    if error.error_code != 0 {
        return Err(WxErrorException::Wx(
            wx_rust_common::error::WxErrorError::new(error),
        ));
    }
    let access_token = wx_rust_common::bean::WxAccessToken::from_json(result_content)
        .map_err(|e| WxErrorException::Serde(e.to_string()))?;
    config.update_access_token(&access_token.access_token, access_token.expires_in);
    Ok(config.access_token().unwrap_or(access_token.access_token))
}

/// 顺序替换 `%s` 占位符（对应 Java `String.format` 在 URL 模板上的应用）。
fn format_url(template: &str, values: &[&str]) -> String {
    let mut result = template.to_string();
    for value in values {
        if let Some(idx) = result.find("%s") {
            result.replace_range(idx..idx + 2, value);
        }
    }
    result
}
