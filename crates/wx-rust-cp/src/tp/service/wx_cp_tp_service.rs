//! 企业微信第三方应用（服务商）服务门面。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpService` 与
//! `BaseWxCpTpServiceImpl` 暴露的全部方法。Java 的继承链
//! （`WxCpTpServiceImpl` → `WxCpTpServiceApacheHttpClientImpl` →
//! `BaseWxCpTpServiceImpl`）在 Rust 以 trait 默认实现 + 组合表达（与
//! `WxCpService` 门面同一设计原则）：
//! - 「Base」的业务方法（suite token/授权/代企业 token/签名/预授权码等）
//!   为本 trait 的默认实现（对应 Java `BaseWxCpTpServiceImpl`）；
//! - `get_suite_access_token_with_force`（对应 Java
//!   `WxCpTpServiceOkHttpImpl.getSuiteAccessToken(boolean)` 的 suite token
//!   拉取与双检锁刷新）与存储/HTTP/会话/12 个子服务 getter 由
//!   `WxCpTpServiceImpl` 覆写；
//! - 执行引擎（`execute`/`executeInternal` 的指数退避重试 + suite token
//!   自动单次刷新）抽为 `api` 同级模式：`tp::service::r#impl::
//!   base_wx_cp_tp_service_impl` 的泛型自由函数（trait 无法携带泛型方法，
//!   破坏 dyn 兼容），`get`/`post` 默认实现即调用执行引擎；
//! - Java `getRequestHttp()`（`RequestHttp<?,?>`）在 Rust 中以
//!   `http_client() -> &reqwest::Client` 表达（reqwest 统一 HTTP，
//!   ADAPTED）。

use std::sync::Arc;

use async_trait::async_trait;

use wx_rust_common::bean::{WxAccessToken, WxJsapiSignature};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_common::util::RandomUtils;
use wx_rust_common::util::crypto::Sha1;
use wx_rust_common::util::http::{SimpleGetRequestExecutor, SimplePostRequestExecutor};

use crate::bean::message::WxCpTpXmlMessage;
use crate::bean::{
    WxCpMaJsCode2SessionResult, WxCpProviderToken, WxCpTpAdmin, WxCpTpAppQrcode, WxCpTpAuthInfo,
    WxCpTpCorp, WxCpTpCorpId2OpenCorpId, WxCpTpPermanentCodeInfo, WxCpTpPreauthCode,
    WxCpTpUserDetail, WxCpTpUserInfo, WxTpCustomizedAuthUrl, WxTpLoginInfo,
};
use crate::config::WxCpTpConfigStorage;
use crate::enums::url_tp;
use crate::tp::service::{
    WxCpTpContactService, WxCpTpCustomizedService, WxCpTpDepartmentService, WxCpTpEditionService,
    WxCpTpIdConvertService, WxCpTpLicenseService, WxCpTpMediaService, WxCpTpMessageService,
    WxCpTpOAService, WxCpTpOAuth2Service, WxCpTpOrderService, WxCpTpTagService, WxCpTpUserService,
};

/// 企业微信第三方应用服务门面。
#[async_trait]
pub trait WxCpTpService: Send + Sync {
    // ---- 配置存储 / HTTP / 会话 / 重试参数（由具体实现覆写） ----

    /// 获取配置存储（对应 Java `getWxCpTpConfigStorage()`）。
    fn wx_cp_tp_config_storage(&self) -> Arc<dyn WxCpTpConfigStorage>;

    /// 注入配置存储（对应 Java `setWxCpTpConfigStorage`；Java 注入后会
    /// 重新初始化 HTTP 客户端，Rust 中 reqwest 客户端构造于服务构建时）。
    fn set_wx_cp_tp_config_storage(&self, _config: Arc<dyn WxCpTpConfigStorage>) {}

    /// HTTP 客户端（对应 Java `getRequestHttp()`，reqwest 统一 HTTP，
    /// ADAPTED）。
    fn http_client(&self) -> &reqwest::Client;

    /// HTTP 请求重试间隔（毫秒），对应 Java Base 字段
    /// `retrySleepMillis`（默认 1000）。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// HTTP 请求最大重试次数，对应 Java Base 字段 `maxRetryTimes`
    /// （默认 5）。
    fn max_retry_times(&self) -> i32 {
        5
    }

    /// 设置当微信系统响应系统繁忙时，要等待多少
    /// `retrySleepMillis(ms) * 2^(重试次数 - 1)` 再发起重试（对应 Java
    /// `setRetrySleepMillis(int)`）。
    fn set_retry_sleep_millis(&self, _retry_sleep_millis: i32) {}

    /// 设置当微信系统响应系统繁忙时，最大重试次数（对应 Java
    /// `setMaxRetryTimes(int)`）。
    fn set_max_retry_times(&self, _max_retry_times: i32) {}

    /// 获取会话管理器（对应 Java `getSessionManager()`；Java Base 默认
    /// 装配 `StandardSessionManager`）。
    fn session_manager(&self) -> Arc<dyn WxSessionManager> {
        Arc::new(StandardSessionManager::new())
    }

    /// 初始化 http 请求对象（对应 Java `initHttp()`；Rust 中 reqwest
    /// 客户端在服务构建时初始化，默认空实现）。
    fn init_http(&self) {}

    // ---- 消息签名校验（对应 Java checkSignature） ----

    /// 验证推送过来的消息的正确性（对应 Java `checkSignature(String,
    /// String, String, String)`）。
    ///
    /// 签名算法：`SHA1.gen(token, timestamp, nonce, data)`——四参数排序后
    /// 无分隔符拼接再 SHA1，与消息签名比较。
    fn check_signature(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        data: &str,
    ) -> bool {
        let config = self.wx_cp_tp_config_storage();
        let token = config.token().unwrap_or_default();
        // Java `SHA1.gen(token, timestamp, nonce, data)`：排序后无分隔符拼接
        match Sha1::digest(&[token.as_str(), timestamp, nonce, data]) {
            Ok(s) => s == msg_signature,
            Err(_) => false,
        }
    }

    // ---- suite access token（对应 Java getSuiteAccessToken 等） ----

    /// 获取 suite_access_token，不强制刷新（对应 Java
    /// `getSuiteAccessToken()`）。
    async fn get_suite_access_token(&self) -> Result<String, WxErrorException> {
        self.get_suite_access_token_with_force(false).await
    }

    /// 获取 suite_access_token（可强制刷新，线程安全，对应 Java
    /// `getSuiteAccessToken(boolean)`）。
    ///
    /// 双检锁（`WxCpTpConfigStorage::suite_access_token_lock`）保证多线程
    /// 同时刷新时只刷新一次；请求体 `{suite_id, suite_secret, suite_ticket}`
    /// POST `/cgi-bin/service/get_suite_token`。默认返回 -99 未实现，由
    /// `WxCpTpServiceImpl` 覆写（对应 Java OkHttp 实现）。
    async fn get_suite_access_token_with_force(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getSuiteAccessToken 未实现（由 WxCpTpServiceImpl 覆写）",
        ))
    }

    /// 获取 suite_access_token 和剩余过期时间，不强制刷新（对应 Java
    /// `getSuiteAccessTokenEntity()`）。
    async fn get_suite_access_token_entity(&self) -> Result<WxAccessToken, WxErrorException> {
        self.get_suite_access_token_entity_with_force(false).await
    }

    /// 获取 suite_access_token 和剩余过期时间，支持强制刷新（对应 Java
    /// `getSuiteAccessTokenEntity(boolean)`）。
    async fn get_suite_access_token_entity_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<WxAccessToken, WxErrorException> {
        self.get_suite_access_token_with_force(force_refresh)
            .await?;
        Ok(self.wx_cp_tp_config_storage().suite_access_token_entity())
    }

    // ---- suite ticket（对应 Java getSuiteTicket 等） ----

    /// 获得 suite_ticket，不强制刷新（对应 Java `getSuiteTicket()`）。
    ///
    /// suite_ticket 由微信服务器定时推送（每 10 分钟），不能主动获取；
    /// 本地不存在或过期时抛 40085 `invalid suite ticket`（对应 Java
    /// `WxError.fromJson("{\"errcode\":40085, ...}")`）。
    async fn get_suite_ticket(&self) -> Result<String, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        if config.is_suite_ticket_expired() {
            // 本地 suite ticket 不存在或者过期
            return Err(WxErrorException::from_code(40085, "invalid suite ticket"));
        }
        config
            .suite_ticket()
            .ok_or_else(|| WxErrorException::from_code(40085, "invalid suite ticket"))
    }

    /// 获得 suite_ticket（对应 Java `getSuiteTicket(boolean)`，
    /// @Deprecated：suite ticket 由微信服务器推送，不能强制刷新，忽略
    /// forceRefresh 参数）。
    async fn get_suite_ticket_with_force(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        self.get_suite_ticket().await
    }

    /// 保存企业微信定时推送的 suite_ticket（对应 Java
    /// `setSuiteTicket(String)`：默认有效期 28 分钟）。
    fn set_suite_ticket(&self, suite_ticket: &str) {
        self.set_suite_ticket_with_expires(suite_ticket, 28 * 60);
    }

    /// 保存企业微信定时推送的 suite_ticket（对应 Java
    /// `setSuiteTicket(String, int)`：带有效期，内部加锁更新）。
    fn set_suite_ticket_with_expires(&self, suite_ticket: &str, expires_in_seconds: i32) {
        // 对应 Java synchronized (globalSuiteTicketRefreshLock)
        self.wx_cp_tp_config_storage()
            .update_suite_ticket(suite_ticket, expires_in_seconds);
    }

    // ---- 授权企业的 jsapi ticket（对应 Java getAuthCorpJsApiTicket 等） ----

    /// 获取授权企业的 jsapi ticket（对应 Java
    /// `getAuthCorpJsApiTicket(String)`）。
    ///
    /// 本地过期时 GET `/cgi-bin/get_jsapi_ticket`（不带 suite token），
    /// 响应 `ticket`/`expires_in` 更新到配置缓存。
    async fn get_auth_corp_js_api_ticket(
        &self,
        auth_corp_id: &str,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        if config.is_auth_corp_js_api_ticket_expired(auth_corp_id) {
            let url = format!(
                "{}?access_token={}",
                config.api_url(url_tp::GET_AUTH_CORP_JSAPI_TICKET),
                config.access_token(auth_corp_id).unwrap_or_default()
            );
            let resp = self.get_without_suite_token(&url, "", true).await?;
            let json: serde_json::Value =
                serde_json::from_str(&resp).map_err(|e| WxErrorException::Serde(e.to_string()))?;
            let errcode = json.get("errcode").and_then(|v| v.as_i64()).unwrap_or(0);
            if errcode == 0 {
                let js_api_ticket = json
                    .get("ticket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| WxErrorException::from_code(-99, "ticket 字段缺失"))?;
                let expired_in_seconds =
                    json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                // 对应 Java synchronized (globalAuthCorpJsApiTicketRefreshLock)
                config.update_auth_corp_js_api_ticket(
                    auth_corp_id,
                    js_api_ticket,
                    expired_in_seconds,
                );
            } else {
                let error = wx_rust_common::error::WxError::from_json_with_type(
                    &resp,
                    Some(wx_rust_common::enums::WxType::Cp),
                );
                return Err(WxErrorException::from_code(
                    error.error_code,
                    error.error_msg.unwrap_or_default(),
                ));
            }
        }
        config
            .auth_corp_js_api_ticket(auth_corp_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "auth corp jsapi ticket 为空"))
    }

    /// 获取授权企业的 jsapi ticket，支持强制刷新（对应 Java
    /// `getAuthCorpJsApiTicket(String, boolean)`）。
    async fn get_auth_corp_js_api_ticket_with_force(
        &self,
        auth_corp_id: &str,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        if force_refresh {
            self.wx_cp_tp_config_storage()
                .expire_auth_corp_js_api_ticket(auth_corp_id);
        }
        self.get_auth_corp_js_api_ticket(auth_corp_id).await
    }

    /// 获取第三方应用的 suite jsapi ticket（对应 Java
    /// `getSuiteJsApiTicket(String)`）。
    ///
    /// 本地过期时 GET `/cgi-bin/ticket/get`（`type=agent_config`，不带
    /// suite token），响应 `ticket`/`expires_in` 更新到配置缓存。
    async fn get_suite_js_api_ticket(
        &self,
        auth_corp_id: &str,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        if config.is_auth_suite_js_api_ticket_expired(auth_corp_id) {
            let url = format!(
                "{}?type=agent_config&access_token={}",
                config.api_url(url_tp::GET_SUITE_JSAPI_TICKET),
                config.access_token(auth_corp_id).unwrap_or_default()
            );
            let resp = self.get_without_suite_token(&url, "", true).await?;
            let json: serde_json::Value =
                serde_json::from_str(&resp).map_err(|e| WxErrorException::Serde(e.to_string()))?;
            let errcode = json.get("errcode").and_then(|v| v.as_i64()).unwrap_or(0);
            if errcode == 0 {
                let js_api_ticket = json
                    .get("ticket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| WxErrorException::from_code(-99, "ticket 字段缺失"))?;
                let expired_in_seconds =
                    json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                // 对应 Java synchronized (globalJsApiTicketRefreshLock)
                config.update_auth_suite_js_api_ticket(
                    auth_corp_id,
                    js_api_ticket,
                    expired_in_seconds,
                );
            } else {
                let error = wx_rust_common::error::WxError::from_json_with_type(
                    &resp,
                    Some(wx_rust_common::enums::WxType::Cp),
                );
                return Err(WxErrorException::from_code(
                    error.error_code,
                    error.error_msg.unwrap_or_default(),
                ));
            }
        }
        config
            .auth_suite_js_api_ticket(auth_corp_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "suite jsapi ticket 为空"))
    }

    /// 获取第三方应用的 suite jsapi ticket，支持强制刷新（对应 Java
    /// `getSuiteJsApiTicket(String, boolean)`）。
    async fn get_suite_js_api_ticket_with_force(
        &self,
        auth_corp_id: &str,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        if force_refresh {
            self.wx_cp_tp_config_storage()
                .expire_auth_suite_js_api_ticket(auth_corp_id);
        }
        self.get_suite_js_api_ticket(auth_corp_id).await
    }

    // ---- 企业小程序登录 / 企业凭证 / 永久授权码（对应 Java 各方法） ----

    /// 小程序登录凭证校验（对应 Java `jsCode2Session(String)`）。
    ///
    /// GET `{base}/cgi-bin/service/miniprogram/jscode2session`，query
    /// `js_code=..&grant_type=authorization_code`（带 suite token）。
    async fn js_code_2_session(
        &self,
        js_code: &str,
    ) -> Result<WxCpMaJsCode2SessionResult, WxErrorException> {
        let query = format!("js_code={js_code}&grant_type=authorization_code");
        let config = self.wx_cp_tp_config_storage();
        let response = self
            .get(&config.api_url(url_tp::JSCODE_TO_SESSION), &query)
            .await?;
        WxCpMaJsCode2SessionResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取企业凭证（对应 Java `getCorpToken(String, String)`）。
    ///
    /// POST `/cgi-bin/service/get_corp_token`，请求体
    /// `{"auth_corpid":.., "permanent_code":..}`（带 suite token），响应
    /// 解析为 `WxAccessToken`。
    async fn get_corp_token(
        &self,
        auth_corp_id: &str,
        permanent_code: &str,
    ) -> Result<WxAccessToken, WxErrorException> {
        let body = serde_json::json!({
            "auth_corpid": auth_corp_id,
            "permanent_code": permanent_code,
        })
        .to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_CORP_TOKEN), &body)
            .await?;
        serde_json::from_str(&result).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取企业凭证，支持强制刷新（对应 Java
    /// `getCorpToken(String, String, boolean)`）。
    ///
    /// 本地未过期且非强制刷新时直接返回缓存实体；否则重新获取并更新缓存。
    async fn get_corp_token_with_force(
        &self,
        auth_corp_id: &str,
        permanent_code: &str,
        force_refresh: bool,
    ) -> Result<WxAccessToken, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        if config.is_access_token_expired(auth_corp_id) || force_refresh {
            let corp_token = self.get_corp_token(auth_corp_id, permanent_code).await?;
            config.update_access_token(
                auth_corp_id,
                &corp_token.access_token,
                corp_token.expires_in,
            );
        }
        Ok(config.access_token_entity(auth_corp_id))
    }

    /// 获取企业永久授权码（对应 Java `getPermanentCode(String)`，
    /// @Deprecated）。
    ///
    /// POST `/cgi-bin/service/get_permanent_code`，从响应提取
    /// `auth_corp_info` 对象解析 `WxCpTpCorp` 并回填 `permanent_code`。
    async fn get_permanent_code(&self, auth_code: &str) -> Result<WxCpTpCorp, WxErrorException> {
        let body = serde_json::json!({ "auth_code": auth_code }).to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_PERMANENT_CODE), &body)
            .await?;
        parse_permanent_code(&result)
    }

    /// 获取企业永久授权码（v2，对应 Java `getV2PermanentCode(String)`）。
    async fn get_v2_permanent_code(&self, auth_code: &str) -> Result<WxCpTpCorp, WxErrorException> {
        let body = serde_json::json!({ "auth_code": auth_code }).to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_V2_PERMANENT_CODE), &body)
            .await?;
        parse_permanent_code(&result)
    }

    /// 获取企业永久授权码信息（对应 Java `getPermanentCodeInfo(String)`）。
    async fn get_permanent_code_info(
        &self,
        auth_code: &str,
    ) -> Result<WxCpTpPermanentCodeInfo, WxErrorException> {
        let body = serde_json::json!({ "auth_code": auth_code }).to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_PERMANENT_CODE), &body)
            .await?;
        WxCpTpPermanentCodeInfo::from_json(&result).map_err(WxErrorException::Serde)
    }

    /// 获取企业永久授权码信息（v2，对应 Java `getV2PermanentCodeInfo(String)`）。
    async fn get_v2_permanent_code_info(
        &self,
        auth_code: &str,
    ) -> Result<WxCpTpPermanentCodeInfo, WxErrorException> {
        let body = serde_json::json!({ "auth_code": auth_code }).to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_V2_PERMANENT_CODE), &body)
            .await?;
        WxCpTpPermanentCodeInfo::from_json(&result).map_err(WxErrorException::Serde)
    }

    /// 获取预授权链接（对应 Java `getPreAuthUrl(String, String)`）。
    ///
    /// GET `/cgi-bin/service/get_pre_auth_code` 获取预授权码后拼接
    /// `https://open.work.weixin.qq.com/3rdapp/install?suite_id=..&
    /// pre_auth_code=..&redirect_uri=..[&state=..]`；`redirect_uri` 按
    /// Java `URLEncoder.encode(uri, "utf-8")` 编码（空格 `+`）。
    async fn get_pre_auth_url(
        &self,
        redirect_uri: &str,
        state: &str,
    ) -> Result<String, WxErrorException> {
        self.get_pre_auth_url_with_auth_type(redirect_uri, state, None)
            .await
    }

    /// 获取预授权链接（测试环境，对应 Java `getPreAuthUrl(String, String,
    /// int)`）。
    ///
    /// 额外调用 POST `https://qyapi.weixin.qq.com/cgi-bin/service/
    /// set_session_info` 设置 `session_info.auth_type`（0 正式授权，
    /// 1 测试授权）。
    async fn get_pre_auth_url_with_auth_type(
        &self,
        redirect_uri: &str,
        state: &str,
        auth_type: Option<i32>,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .get(&config.api_url(url_tp::GET_PREAUTH_CODE), "")
            .await?;
        let pre_auth_code =
            WxCpTpPreauthCode::from_json(&result).map_err(WxErrorException::Serde)?;

        if let Some(auth_type) = auth_type {
            // 对应 Java set_session_info 调用（固定域名，不带 suite token）
            let set_session_url = "https://qyapi.weixin.qq.com/cgi-bin/service/set_session_info";
            let body = serde_json::json!({
                "pre_auth_code": pre_auth_code.pre_auth_code,
                "session_info": { "auth_type": auth_type },
            })
            .to_string();
            self.post_without_suite_token(set_session_url, &body, true)
                .await?;
        }

        let mut pre_auth_url = format!(
            "https://open.work.weixin.qq.com/3rdapp/install?suite_id={}&pre_auth_code={}&redirect_uri={}",
            config.suite_id(),
            pre_auth_code.pre_auth_code,
            url_encode_form(redirect_uri)
        );
        if !state.trim().is_empty() {
            pre_auth_url.push_str(&format!("&state={state}"));
        }
        Ok(pre_auth_url)
    }

    /// 获取企业的授权信息（对应 Java `getAuthInfo(String, String)`）。
    ///
    /// POST `/cgi-bin/service/get_auth_info`，请求体
    /// `{"auth_corpid":.., "permanent_code":..}`（带 suite token）。
    async fn get_auth_info(
        &self,
        auth_corp_id: &str,
        permanent_code: &str,
    ) -> Result<WxCpTpAuthInfo, WxErrorException> {
        let body = serde_json::json!({
            "auth_corpid": auth_corp_id,
            "permanent_code": permanent_code,
        })
        .to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_AUTH_INFO), &body)
            .await?;
        WxCpTpAuthInfo::from_json(&result).map_err(WxErrorException::Serde)
    }

    // ---- 通用 GET/POST 执行通道（对应 Java get/post 各重载） ----

    /// GET 请求（对应 Java `get(String, String)`，自动带 suite token；
    /// query 为空传 ""，对应 Java null）。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        self.get_without_suite_token(url, query_param, false).await
    }

    /// GET 请求（对应 Java `get(String, String, boolean)`：`true` 时请求
    /// 忽略 suite access token）。
    async fn get_without_suite_token(
        &self,
        url: &str,
        query_param: &str,
        without_suite_access_token: bool,
    ) -> Result<String, WxErrorException> {
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        crate::tp::service::r#impl::base_wx_cp_tp_service_impl::execute_with_retry_tp(
            self,
            &executor,
            url,
            query_param.to_string(),
            without_suite_access_token,
        )
        .await
    }

    /// POST 请求（对应 Java `post(String, String)`，自动带 suite token）。
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException> {
        self.post_without_suite_token(url, post_data, false).await
    }

    /// POST 请求（对应 Java `post(String, String, boolean)`：`true` 时
    /// 请求忽略 suite access token）。
    async fn post_without_suite_token(
        &self,
        url: &str,
        post_data: &str,
        without_suite_access_token: bool,
    ) -> Result<String, WxErrorException> {
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        crate::tp::service::r#impl::base_wx_cp_tp_service_impl::execute_with_retry_tp(
            self,
            &executor,
            url,
            post_data.to_string(),
            without_suite_access_token,
        )
        .await
    }

    // ---- 用户身份/登录信息（对应 Java getUserInfo3rd 等） ----

    /// 获取登录/访问用户身份（对应 Java `getUserInfo3rd(String)`）。
    async fn get_user_info_3rd(&self, code: &str) -> Result<WxCpTpUserInfo, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        let url = format!("{}?code={code}", config.api_url(url_tp::GET_USERINFO3RD));
        let result = self.get(&url, "").await?;
        WxCpTpUserInfo::from_json(&result).map_err(WxErrorException::Serde)
    }

    /// 获取访问用户敏感信息（对应 Java `getUserDetail3rd(String)`）。
    async fn get_user_detail_3rd(
        &self,
        user_ticket: &str,
    ) -> Result<WxCpTpUserDetail, WxErrorException> {
        let body = serde_json::json!({ "user_ticket": user_ticket }).to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_USERDETAIL3RD), &body)
            .await?;
        WxCpTpUserDetail::from_json(&result).map_err(WxErrorException::Serde)
    }

    /// 获取登录用户信息（对应 Java `getLoginInfo(String)`）。
    ///
    /// POST `/cgi-bin/service/get_login_info?access_token=<providerToken>`
    /// （不带 suite token）。
    async fn get_login_info(&self, auth_code: &str) -> Result<WxTpLoginInfo, WxErrorException> {
        let body = serde_json::json!({ "auth_code": auth_code }).to_string();
        let access_token = self.get_wx_cp_provider_token().await?;
        let config = self.wx_cp_tp_config_storage();
        let url = format!(
            "{}?access_token={access_token}",
            config.api_url(url_tp::GET_LOGIN_INFO)
        );
        let response_text = self.post_without_suite_token(&url, &body, true).await?;
        WxTpLoginInfo::from_json(&response_text).map_err(WxErrorException::Serde)
    }

    /// 获取带参授权链接（对应 Java `getCustomizedAuthUrl(String, List)`）。
    ///
    /// POST `/cgi-bin/service/get_customized_auth_url?provider_access_token=
    /// <token>`（不带 suite token），请求体含 `state` 与 `templateid_list`。
    async fn get_customized_auth_url(
        &self,
        state: &str,
        template_id_list: &[String],
    ) -> Result<WxTpCustomizedAuthUrl, WxErrorException> {
        let body = serde_json::json!({
            "state": state,
            "templateid_list": template_id_list,
        })
        .to_string();
        let provider_access_token = self.get_wx_cp_provider_token().await?;
        let config = self.wx_cp_tp_config_storage();
        let url = format!(
            "{}?provider_access_token={provider_access_token}",
            config.api_url(url_tp::GET_CUSTOMIZED_AUTH_URL)
        );
        let response_text = self.post_without_suite_token(&url, &body, true).await?;
        WxTpCustomizedAuthUrl::from_json(&response_text).map_err(WxErrorException::Serde)
    }

    // ---- provider token（对应 Java getWxCpProviderToken 等） ----

    /// 获取服务商 providerToken（对应 Java `getWxCpProviderToken()`）。
    ///
    /// 过期时 POST `/cgi-bin/service/get_provider_token`（不带 suite
    /// token），请求体 `{corpid, provider_secret}`；Java 语义：更新缓存时
    /// 有效期减去 200 秒。
    async fn get_wx_cp_provider_token(&self) -> Result<String, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        if config.is_provider_token_expired() {
            let body = serde_json::json!({
                "corpid": config.corp_id(),
                "provider_secret": config.provider_secret(),
            })
            .to_string();
            // providerAccessToken 的获取不需要 suiteAccessToken
            let token = WxCpProviderToken::from_json(
                &self
                    .post_without_suite_token(
                        &config.api_url(url_tp::GET_PROVIDER_TOKEN),
                        &body,
                        true,
                    )
                    .await?,
            )
            .map_err(WxErrorException::Serde)?;
            // Java 语义：expiresIn - 200（预留 200 秒）
            config.update_provider_token(&token.provider_access_token, token.expires_in - 200);
        }
        config
            .provider_token()
            .ok_or_else(|| WxErrorException::from_code(-99, "provider token 为空"))
    }

    /// 获取服务商 providerToken 和剩余过期时间（对应 Java
    /// `getWxCpProviderTokenEntity()`）。
    async fn get_wx_cp_provider_token_entity(&self) -> Result<WxCpProviderToken, WxErrorException> {
        self.get_wx_cp_provider_token_entity_with_force(false).await
    }

    /// 获取服务商 providerToken 和剩余过期时间，支持强制刷新（对应 Java
    /// `getWxCpProviderTokenEntity(boolean)`）。
    async fn get_wx_cp_provider_token_entity_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<WxCpProviderToken, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        if force_refresh {
            config.expire_provider_token();
        }
        self.get_wx_cp_provider_token().await?;
        config
            .provider_token_entity()
            .ok_or_else(|| WxErrorException::from_code(-99, "provider token 实体为空"))
    }

    // ---- 消息加解密（对应 Java fromEncryptedXml/getVerifyDecrypt） ----

    /// 解密服务商推送的加密消息（对应 Java `fromEncryptedXml`）。
    // Java 原方法即实例方法，命名保真优先于 Rust 命名约定
    #[allow(clippy::wrong_self_convention)]
    fn from_encrypted_xml(
        &self,
        encrypted_xml: &str,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<WxCpTpXmlMessage, String> {
        WxCpTpXmlMessage::from_encrypted_xml(
            encrypted_xml,
            self.wx_cp_tp_config_storage().as_ref(),
            timestamp,
            nonce,
            msg_signature,
        )
    }

    /// 验证 URL 回调的 echoStr（对应 Java `getVerifyDecrypt(String)`：
    /// `new WxCpTpCryptUtil(configStorage).decrypt(sVerifyEchoStr)`）。
    fn get_verify_decrypt(&self, s_verify_echo_str: &str) -> Result<String, String> {
        let crypt_util =
            crate::util::crypto::WxCpTpCryptUtil::new(self.wx_cp_tp_config_storage().as_ref())?;
        crypt_util.decrypt(s_verify_echo_str)
    }

    // ---- 管理/二维码/ID 转换（对应 Java getAdminList 等） ----

    /// 获取应用的管理员列表（对应 Java `getAdminList(String, Integer)`）。
    async fn get_admin_list(
        &self,
        auth_corp_id: &str,
        agent_id: Option<i32>,
    ) -> Result<WxCpTpAdmin, WxErrorException> {
        let body = serde_json::json!({
            "auth_corpid": auth_corp_id,
            "agentid": agent_id,
        })
        .to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_ADMIN_LIST), &body)
            .await?;
        WxCpTpAdmin::from_json(&result).map_err(WxErrorException::Serde)
    }

    /// 获取应用二维码（对应 Java `getAppQrcode(String, String, String,
    /// Integer, Integer)`）。
    async fn get_app_qrcode(
        &self,
        suite_id: &str,
        app_id: &str,
        state: &str,
        style: Option<i32>,
        result_type: Option<i32>,
    ) -> Result<WxCpTpAppQrcode, WxErrorException> {
        let body = serde_json::json!({
            "suite_id": suite_id,
            "appid": app_id,
            "state": state,
            "style": style,
            "result_type": result_type,
        })
        .to_string();
        let config = self.wx_cp_tp_config_storage();
        let result = self
            .post(&config.api_url(url_tp::GET_APP_QRCODE), &body)
            .await?;
        WxCpTpAppQrcode::from_json(&result).map_err(WxErrorException::Serde)
    }

    /// 明文 corpid 转换为加密 corpid（对应 Java
    /// `corpId2OpenCorpId(String)`）。
    ///
    /// POST `/cgi-bin/service/corpid_to_opencorpid?provider_access_token=
    /// <token>`（Java 未带 withoutSuiteToken，此处镜像 Java 原样走
    /// 自动带 suite token 通道）。
    async fn corp_id_2_open_corp_id(
        &self,
        corp_id: &str,
    ) -> Result<WxCpTpCorpId2OpenCorpId, WxErrorException> {
        let body = serde_json::json!({ "corpid": corp_id }).to_string();
        let provider_access_token = self.get_wx_cp_provider_token().await?;
        let config = self.wx_cp_tp_config_storage();
        let url = format!(
            "{}?provider_access_token={provider_access_token}",
            config.api_url(url_tp::CORPID_TO_OPENCORPID)
        );
        let result = self.post(&url, &body).await?;
        WxCpTpCorpId2OpenCorpId::from_json(&result).map_err(WxErrorException::Serde)
    }

    // ---- jsapi 签名（对应 Java createAuthCorpJsApiTicketSignature 等） ----

    /// 创建机构级 jsapiTicket 签名（对应 Java
    /// `createAuthCorpJsApiTicketSignature(String, String)`）。
    async fn create_auth_corp_js_api_ticket_signature(
        &self,
        url: &str,
        auth_corp_id: &str,
    ) -> Result<WxJsapiSignature, WxErrorException> {
        let jsapi_ticket = self.get_auth_corp_js_api_ticket(auth_corp_id).await?;
        Ok(self.do_create_wx_jsapi_signature(url, auth_corp_id, &jsapi_ticket))
    }

    /// 创建应用级 jsapiTicket 签名（对应 Java
    /// `createSuiteJsApiTicketSignature(String, String)`）。
    async fn create_suite_js_api_ticket_signature(
        &self,
        url: &str,
        auth_corp_id: &str,
    ) -> Result<WxJsapiSignature, WxErrorException> {
        let jsapi_ticket = self.get_suite_js_api_ticket(auth_corp_id).await?;
        Ok(self.do_create_wx_jsapi_signature(url, auth_corp_id, &jsapi_ticket))
    }

    /// 计算 jsapi 签名（对应 Java `doCreateWxJsapiSignature`）：
    /// `SHA1(jsapi_ticket=..&noncestr=..&timestamp=..&url=..)` 按 `&`
    /// 连接（`digest_with_amp`），appid 取 authCorpId。
    fn do_create_wx_jsapi_signature(
        &self,
        url: &str,
        auth_corp_id: &str,
        jsapi_ticket: &str,
    ) -> WxJsapiSignature {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let noncestr = RandomUtils::get_random_str();
        let ticket_param = format!("jsapi_ticket={jsapi_ticket}");
        let noncestr_param = format!("noncestr={noncestr}");
        let timestamp_param = format!("timestamp={timestamp}");
        let url_param = format!("url={url}");
        let signature =
            Sha1::digest_with_amp(&[&ticket_param, &noncestr_param, &timestamp_param, &url_param])
                .unwrap_or_default();
        // Java：appId 固定取 authCorpId
        WxJsapiSignature::new(auth_corp_id, noncestr, timestamp, url, signature)
    }

    // ---- 缓存失效（对应 Java expireXxx 系列） ----

    /// 使套件 accessToken 缓存失效（对应 Java `expireSuiteAccessToken()`）。
    fn expire_suite_access_token(&self) {
        self.wx_cp_tp_config_storage().expire_suite_access_token();
    }

    /// 使机构 accessToken 缓存失效（对应 Java `expireAccessToken(String)`）。
    fn expire_access_token(&self, auth_corp_id: &str) {
        self.wx_cp_tp_config_storage()
            .expire_access_token(auth_corp_id);
    }

    /// 使机构 jsapi ticket 缓存失效（对应 Java
    /// `expireAuthCorpJsApiTicket(String)`）。
    fn expire_auth_corp_js_api_ticket(&self, auth_corp_id: &str) {
        self.wx_cp_tp_config_storage()
            .expire_auth_corp_js_api_ticket(auth_corp_id);
    }

    /// 使应用 jsapi ticket 失效（对应 Java
    /// `expireAuthSuiteJsApiTicket(String)`）。
    fn expire_auth_suite_js_api_ticket(&self, auth_corp_id: &str) {
        self.wx_cp_tp_config_storage()
            .expire_auth_suite_js_api_ticket(auth_corp_id);
    }

    /// 使服务商 accessToken 失效（对应 Java `expireProviderToken()`）。
    fn expire_provider_token(&self) {
        self.wx_cp_tp_config_storage().expire_provider_token();
    }

    // ---- 子服务 getter（对应 Java getWxCpTpXxxService()；默认 None，
    // 由 WxCpTpServiceImpl 覆写为装配后的实例） ----

    /// 通讯录服务（对应 Java `getWxCpTpContactService()`）。
    fn wx_cp_tp_contact_service(&self) -> Option<Arc<dyn WxCpTpContactService>> {
        None
    }
    /// 部门服务（对应 Java `getWxCpTpDepartmentService()`）。
    fn wx_cp_tp_department_service(&self) -> Option<Arc<dyn WxCpTpDepartmentService>> {
        None
    }
    /// 素材服务（对应 Java `getWxCpTpMediaService()`）。
    fn wx_cp_tp_media_service(&self) -> Option<Arc<dyn WxCpTpMediaService>> {
        None
    }
    /// OA 服务（对应 Java `getWxCpTpOAService()`）。
    fn wx_cp_tp_oa_service(&self) -> Option<Arc<dyn WxCpTpOAService>> {
        None
    }
    /// 成员服务（对应 Java `getWxCpTpUserService()`）。
    fn wx_cp_tp_user_service(&self) -> Option<Arc<dyn WxCpTpUserService>> {
        None
    }
    /// 接口许可服务（对应 Java `getWxCpTpLicenseService()`）。
    fn wx_cp_tp_license_service(&self) -> Option<Arc<dyn WxCpTpLicenseService>> {
        None
    }
    /// 消息服务（对应 Java `getWxCpTpMessageService()`）。
    fn wx_cp_tp_message_service(&self) -> Option<Arc<dyn WxCpTpMessageService>> {
        None
    }
    /// 应用版本付费订单服务（对应 Java `getWxCpTpOrderService()`）。
    fn wx_cp_tp_order_service(&self) -> Option<Arc<dyn WxCpTpOrderService>> {
        None
    }
    /// 应用版本付费版本服务（对应 Java `getWxCpTpEditionService()`）。
    fn wx_cp_tp_edition_service(&self) -> Option<Arc<dyn WxCpTpEditionService>> {
        None
    }
    /// ID 转换服务（对应 Java `getWxCpTpIdConverService()`）。
    fn wx_cp_tp_id_convert_service(&self) -> Option<Arc<dyn WxCpTpIdConvertService>> {
        None
    }
    /// OAuth2 服务（对应 Java `getWxCpTpOAuth2Service()`）。
    fn wx_cp_tp_o_auth2_service(&self) -> Option<Arc<dyn WxCpTpOAuth2Service>> {
        None
    }
    /// 代开发服务（对应 Java `getWxCpTpCustomizedService()`）。
    fn wx_cp_tp_customized_service(&self) -> Option<Arc<dyn WxCpTpCustomizedService>> {
        None
    }
    /// 标签服务（对应 Java 子服务 getter 之外暴露的
    /// `getWxCpTpTagService()` 能力入口；默认 None）。
    fn wx_cp_tp_tag_service(&self) -> Option<Arc<dyn WxCpTpTagService>> {
        None
    }

    // ---- 子服务 setter（对应 Java setWxCpTpXxxService()；trait 默认
    // 空实现，具体实现以装配后的固定实例承载） ----

    /// 设置通讯录服务（对应 Java `setWxCpTpContactService`）。
    fn set_wx_cp_tp_contact_service(&self, _service: Arc<dyn WxCpTpContactService>) {}
    /// 设置部门服务（对应 Java `setWxCpTpDepartmentService`）。
    fn set_wx_cp_tp_department_service(&self, _service: Arc<dyn WxCpTpDepartmentService>) {}
    /// 设置素材服务（对应 Java `setWxCpTpMediaService`）。
    fn set_wx_cp_tp_media_service(&self, _service: Arc<dyn WxCpTpMediaService>) {}
    /// 设置 OA 服务（对应 Java `setWxCpTpOAService`）。
    fn set_wx_cp_tp_oa_service(&self, _service: Arc<dyn WxCpTpOAService>) {}
    /// 设置成员服务（对应 Java `setWxCpTpUserService`）。
    fn set_wx_cp_tp_user_service(&self, _service: Arc<dyn WxCpTpUserService>) {}
    /// 设置接口许可服务（对应 Java `setWxCpTpLicenseService`）。
    fn set_wx_cp_tp_license_service(&self, _service: Arc<dyn WxCpTpLicenseService>) {}
    /// 设置消息服务（对应 Java `setWxCpTpMessageService`）。
    fn set_wx_cp_tp_message_service(&self, _service: Arc<dyn WxCpTpMessageService>) {}
    /// 设置订单服务（对应 Java `setWxCpTpOrderService`）。
    fn set_wx_cp_tp_order_service(&self, _service: Arc<dyn WxCpTpOrderService>) {}
    /// 设置版本服务（对应 Java `setWxCpTpOrderService(WxCpTpEditionService)`
    /// 重载）。
    fn set_wx_cp_tp_edition_service(&self, _service: Arc<dyn WxCpTpEditionService>) {}
    /// 设置 ID 转换服务（对应 Java `setWxCpTpIdConverService`）。
    fn set_wx_cp_tp_id_convert_service(&self, _service: Arc<dyn WxCpTpIdConvertService>) {}
    /// 设置 OAuth2 服务（对应 Java `setWxCpTpOAuth2Service`）。
    fn set_wx_cp_tp_o_auth2_service(&self, _service: Arc<dyn WxCpTpOAuth2Service>) {}
    /// 设置代开发服务（对应 Java `setWxCpTpCustomizedService`）。
    fn set_wx_cp_tp_customized_service(&self, _service: Arc<dyn WxCpTpCustomizedService>) {}
}

/// 从 get_permanent_code 响应中提取 `auth_corp_info` 并回填
/// `permanent_code`（对应 Java `getPermanentCode`/`getV2PermanentCode`
/// 内联逻辑）。
pub fn parse_permanent_code(result: &str) -> Result<WxCpTpCorp, WxErrorException> {
    let json: serde_json::Value =
        serde_json::from_str(result).map_err(|e| WxErrorException::Serde(e.to_string()))?;
    let auth_corp_info = json
        .get("auth_corp_info")
        .ok_or_else(|| WxErrorException::from_code(-99, "auth_corp_info 字段缺失"))?;
    let mut corp =
        WxCpTpCorp::from_json(&auth_corp_info.to_string()).map_err(WxErrorException::Serde)?;
    corp.permanent_code = json
        .get("permanent_code")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    Ok(corp)
}

/// Java `URLEncoder.encode(String, "utf-8")` 语义：`A-Z a-z 0-9` 与
/// `- _ . *` 保留，空格编码为 `+`，其余字符按 UTF-8 逐字节编码为
/// `%XX`（大写十六进制）。
fn url_encode_form(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for b in input.bytes() {
        let c = b as char;
        if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '*') {
            out.push(c);
        } else if b == b' ' {
            out.push('+');
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
