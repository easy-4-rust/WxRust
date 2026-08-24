//! 企业微信服务门面。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpService`（继承 common
//! `WxService`）与 `BaseWxCpServiceImpl`/`WxCpServiceImpl` 暴露的全部方法。
//! Java 三层继承链（Impl → HttpComponentsImpl → Base）在 Rust 以
//! trait 默认实现 + 组合表达（与 mp/miniapp 同一设计原则）。
//!
//! 说明：
//! - Java 的 31 个子服务 getter（`getDepartmentService()` 等）由本 trait
//!   提供默认返回 `None` 的 getter，`WxCpServiceImpl` 后续批次覆写为返回
//!   实际子服务实例（子服务 trait 目前为空占位，见 `sub_services`）；
//! - Java `<T, E> execute(RequestExecutor, String, E)` 为泛型方法，trait
//!   无法携带（破坏 dyn 兼容），以 `api::r#impl::base_wx_cp_service_impl`
//!   的泛型自由函数 `execute_with_retry`/`execute_internal`/`execute_normal`
//!   表达（与 mp/miniapp 同一语义、同一文件映射）；`get`/`post` 默认实现
//!   即调用执行引擎；
//! - Java `getRequestHttp()`（`RequestHttp<?,?>`）在 Rust 中以
//!   `http_client() -> &reqwest::Client` 表达（reqwest 统一 HTTP，
//!   ADAPTED：多后端适配由 reqwest 后端类型承担）；
//! - Java 的 `post(String, JsonObject)`/`post(String, ToJson)`/
//!   `post(String, Object)` 重载在 Rust 中以 `serde_json::to_string(...)`
//!   后调用 `post(url, &str)` 表达（调用点序列化，ADAPTED）；
//! - 门面业务方法（`js_code_2_session`/`get_provider_token`）已按 Java
//!   `BaseWxCpServiceImpl.jsCode2Session`/`getProviderToken` 语义在默认
//!   实现中启用（bean 解析逻辑已就绪）。

use std::sync::Arc;

use async_trait::async_trait;

use wx_rust_common::bean::WxJsapiSignature;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::{WxSession, WxSessionManager};
use wx_rust_common::util::RandomUtils;
use wx_rust_common::util::crypto::Sha1;
use wx_rust_common::util::http::{SimpleGetRequestExecutor, SimplePostRequestExecutor};

use crate::api::{
    WxCpAgentService, WxCpAgentWorkBenchService, WxCpChatService, WxCpCorpGroupService,
    WxCpDepartmentService, WxCpExportService, WxCpExternalContactService, WxCpGroupRobotService,
    WxCpHrService, WxCpIntelligentRobotService, WxCpKfService, WxCpLivingService, WxCpMediaService,
    WxCpMeetingService, WxCpMenuService, WxCpMessageService, WxCpMsgAuditService,
    WxCpOAuth2Service, WxCpOaAgentService, WxCpOaCalendarService, WxCpOaMeetingRoomService,
    WxCpOaScheduleService, WxCpOaService, WxCpOaWeDocService, WxCpOaWeDriveService,
    WxCpSchoolHealthService, WxCpSchoolService, WxCpSchoolUserService, WxCpTagService,
    WxCpTaskCardService, WxCpUserService,
};
use crate::bean::{WxCpAgentJsapiSignature, WxCpMaJsCode2SessionResult, WxCpProviderToken};
use crate::config::WxCpConfigStorage;
use crate::enums::{url_core, url_tp};

/// 企业微信服务门面。
#[async_trait]
pub trait WxCpService: Send + Sync {
    /// 获取配置存储（对应 Java `getWxCpConfigStorage()`）。
    fn wx_cp_config_storage(&self) -> Arc<dyn WxCpConfigStorage>;

    /// 注入配置存储（对应 Java `setWxCpConfigStorage(WxCpConfigStorage)`；
    /// Java 中注入后会重新初始化 HTTP 客户端，Rust 中 reqwest 客户端
    /// 构造于服务构建时，默认空实现）。
    fn set_wx_cp_config_storage(&self, _config: Arc<dyn WxCpConfigStorage>) {}

    /// HTTP 客户端（对应 Java `getRequestHttp()` 的 `RequestHttp<?,?>`，
    /// reqwest 统一 HTTP，ADAPTED）。
    fn http_client(&self) -> &reqwest::Client;

    // ---- 执行引擎参数（对应 Java BaseWxCpServiceImpl 的
    // retrySleepMillis/maxRetryTimes 字段） ----

    /// HTTP 请求重试间隔（毫秒），对应 Java `setRetrySleepMillis(int)`
    /// 的读取侧（Base 字段 `retrySleepMillis`，默认 1000）。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// HTTP 请求最大重试次数，对应 Java `setMaxRetryTimes(int)` 的读取侧
    /// （Base 字段 `maxRetryTimes`，默认 5）。
    fn max_retry_times(&self) -> i32 {
        5
    }

    /// 设置当微信系统响应系统繁忙时，要等待多少
    /// `retrySleepMillis(ms) * 2^(重试次数 - 1)` 再发起重试（对应 Java
    /// `setRetrySleepMillis(int)`，默认 1000ms；trait 默认空实现，
    /// 由 `WxCpServiceImpl` 覆写）。
    fn set_retry_sleep_millis(&self, _retry_sleep_millis: i32) {}

    /// 设置当微信系统响应系统繁忙时，最大重试次数（对应 Java
    /// `setMaxRetryTimes(int)`，默认 5 次；trait 默认空实现，由
    /// `WxCpServiceImpl` 覆写）。
    fn set_max_retry_times(&self, _max_retry_times: i32) {}

    // ---- 会话管理（对应 Java getSession/setSessionManager 等） ----

    /// 获取某个 sessionId 对应的 session；sessionId 没有对应的 session 时
    /// 新建一个并返回（对应 Java `getSession(String)`；`sessionManager`
    /// 未设置时返回 `None`，对应 Java null）。
    fn get_session(&self, id: &str) -> Option<Arc<dyn WxSession>> {
        self.session_manager().map(|m| m.get_session(id))
    }

    /// 获取某个 sessionId 对应的 session（对应 Java `getSession(String,
    /// boolean)`）：`create=true` 时不存在则新建，否则返回 `None`。
    fn get_session_with_create(&self, id: &str, create: bool) -> Option<Arc<dyn WxSession>> {
        self.session_manager()
            .and_then(|m| m.get_session_or_create(id, create))
    }

    /// 获取会话管理器（对应 Java `getSessionManager()`；默认 `None`，
    /// `WxCpServiceImpl` 默认装配 `StandardSessionManager`，对应 Java
    /// Base 构造器语义）。
    fn session_manager(&self) -> Option<Arc<dyn WxSessionManager>> {
        None
    }

    /// 设置会话管理器（对应 Java `setSessionManager(WxSessionManager)`）。
    fn set_session_manager(&self, _session_manager: Arc<dyn WxSessionManager>) {}

    // ---- 核心能力（对应 BaseWxCpServiceImpl / WxCpServiceImpl） ----

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
        let config = self.wx_cp_config_storage();
        let token = config.token().unwrap_or_default();
        // Java `SHA1.gen(token, timestamp, nonce, data)`：排序后无分隔符拼接
        match Sha1::digest(&[token.as_str(), timestamp, nonce, data]) {
            Ok(s) => s == msg_signature,
            Err(_) => false,
        }
    }

    /// 获取 access_token，不强制刷新（对应 Java `getAccessToken()`）。
    async fn get_access_token(&self) -> Result<String, WxErrorException> {
        self.get_access_token_with_force(false).await
    }

    /// 获取 access_token（可强制刷新，线程安全，对应 Java
    /// `getAccessToken(boolean)`）。
    ///
    /// 双检锁（`WxCpConfigStorage::access_token_lock`）保证多线程同时刷新
    /// 时只刷新一次，避免超出调用次数上限。默认返回 -99 未实现，由
    /// `WxCpServiceImpl` 覆写（对应 Java `WxCpServiceImpl.getAccessToken`）。
    async fn get_access_token_with_force(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getAccessToken 未实现（由 WxCpServiceImpl 覆写）",
        ))
    }

    /// 获取通讯录同步 access_token（可强制刷新，线程安全，对应 Java
    /// `getContactAccessToken(boolean)`）。
    ///
    /// 通讯录同步相关接口仅支持通过「通讯录同步 secret」调用，需要使用
    /// 独立的 access_token（https://developer.work.weixin.qq.com/document/path/91579）。
    /// 默认返回 -99 未实现，由 `WxCpServiceImpl` 覆写。
    async fn get_contact_access_token_with_force(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getContactAccessToken 未实现（由 WxCpServiceImpl 覆写）",
        ))
    }

    /// 获取会话存档 access_token（可强制刷新，线程安全，对应 Java
    /// `getMsgAuditAccessToken(boolean)`）。
    ///
    /// 会话存档相关接口需要使用会话存档 secret 获取单独的 access_token
    /// （https://developer.work.weixin.qq.com/document/path/91782）。
    /// 默认返回 -99 未实现，由 `WxCpServiceImpl` 覆写。
    async fn get_msg_audit_access_token_with_force(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getMsgAuditAccessToken 未实现（由 WxCpServiceImpl 覆写）",
        ))
    }

    /// 获得 jsapi_ticket，不强制刷新（对应 Java `getJsapiTicket()`）。
    async fn get_jsapi_ticket(&self) -> Result<String, WxErrorException> {
        self.get_jsapi_ticket_with_force(false).await
    }

    /// 获得 jsapi_ticket（可强制刷新，对应 Java `getJsapiTicket(boolean)`）。
    ///
    /// 获得时会检查 jsapi ticket 是否过期，如果过期了就刷新，否则不做任何事
    /// （https://developer.work.weixin.qq.com/document/path/10029）。
    /// 默认返回 -99 未实现，由 `WxCpServiceImpl` 覆写。
    async fn get_jsapi_ticket_with_force(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getJsapiTicket 未实现（由 WxCpServiceImpl 覆写）",
        ))
    }

    /// 获得应用的 jsapi_ticket，不强制刷新（对应 Java `getAgentJsapiTicket()`）。
    async fn get_agent_jsapi_ticket(&self) -> Result<String, WxErrorException> {
        self.get_agent_jsapi_ticket_with_force(false).await
    }

    /// 获得应用的 jsapi_ticket（可强制刷新，对应 Java
    /// `getAgentJsapiTicket(boolean)`）。
    ///
    /// 应用的 jsapi_ticket 用于计算 agentConfig（参见「通过 agentConfig
    /// 注入应用的权限」）的签名，必须用 `wx.agentConfig` 中 agentid 对应的
    /// 应用 secret 去获取 access_token
    /// （https://open.work.weixin.qq.com/api/doc/90000/90136/94313）。
    /// 默认返回 -99 未实现，由 `WxCpServiceImpl` 覆写。
    async fn get_agent_jsapi_ticket_with_force(
        &self,
        _force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getAgentJsapiTicket 未实现（由 WxCpServiceImpl 覆写）",
        ))
    }

    /// 创建调用 jsapi 时所需要的签名（对应 Java `createJsapiSignature(String)`）。
    ///
    /// 算法：`SHA1(jsapi_ticket=..&noncestr=..&timestamp=..&url=..)` 按
    /// `&` 连接（`digest_with_amp`），appid 取配置 corpid。
    async fn create_jsapi_signature(
        &self,
        url: &str,
    ) -> Result<WxJsapiSignature, WxErrorException> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let noncestr = RandomUtils::get_random_str();
        let jsapi_ticket = self.get_jsapi_ticket_with_force(false).await?;
        let ticket_param = format!("jsapi_ticket={jsapi_ticket}");
        let noncestr_param = format!("noncestr={noncestr}");
        let timestamp_param = format!("timestamp={timestamp}");
        let url_param = format!("url={url}");
        let signature =
            Sha1::digest_with_amp(&[&ticket_param, &noncestr_param, &timestamp_param, &url_param])
                .map_err(|e| WxErrorException::from_code(-99, format!("生成签名失败: {e}")))?;
        // Java：appId 固定取 corpid
        let config = self.wx_cp_config_storage();
        Ok(WxJsapiSignature::new(
            config.app_id(),
            noncestr,
            timestamp,
            url,
            signature,
        ))
    }

    /// 创建调用 wx.agentConfig 时所需要的签名（对应 Java
    /// `createAgentJsapiSignature(String)`）。
    ///
    /// 与 `create_jsapi_signature` 算法一致，但使用应用 jsapi_ticket，
    /// 并携带 corpid/agentid
    /// （https://open.work.weixin.qq.com/api/doc/90000/90136/94313）。
    async fn create_agent_jsapi_signature(
        &self,
        url: &str,
    ) -> Result<WxCpAgentJsapiSignature, WxErrorException> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let noncestr = RandomUtils::get_random_str();
        let jsapi_ticket = self.get_agent_jsapi_ticket_with_force(false).await?;
        let ticket_param = format!("jsapi_ticket={jsapi_ticket}");
        let noncestr_param = format!("noncestr={noncestr}");
        let timestamp_param = format!("timestamp={timestamp}");
        let url_param = format!("url={url}");
        let signature =
            Sha1::digest_with_amp(&[&ticket_param, &noncestr_param, &timestamp_param, &url_param])
                .map_err(|e| WxErrorException::from_code(-99, format!("生成签名失败: {e}")))?;
        let config = self.wx_cp_config_storage();
        Ok(WxCpAgentJsapiSignature::new(
            config.app_id(),
            config.agent_id(),
            noncestr,
            timestamp,
            url,
            signature,
        ))
    }

    /// 企业微信小程序登录凭证校验（对应 Java
    /// `BaseWxCpServiceImpl.jsCode2Session(String)`）。
    ///
    /// GET `{base}/cgi-bin/miniprogram/jscode2session`，query 参数
    /// `js_code` + `grant_type=authorization_code`（镜像 Java 的
    /// `HashMap` + `Joiner.on("&").withKeyValueSeparator("=")`，经 `get`
    /// 执行引擎自动追加 access_token），响应解析为
    /// `WxCpMaJsCode2SessionResult`（session_key/userid/corpid）
    /// （https://work.weixin.qq.com/api/doc#90000/90136/90289/wx.qy.login）。
    async fn js_code_2_session(
        &self,
        js_code: &str,
    ) -> Result<WxCpMaJsCode2SessionResult, WxErrorException> {
        let query = format!("js_code={js_code}&grant_type=authorization_code");
        let config = self.wx_cp_config_storage();
        let response = self
            .get(&config.api_url(url_core::JSCODE_TO_SESSION), &query)
            .await?;
        WxCpMaJsCode2SessionResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取企业微信回调 IP 段（对应 Java `getCallbackIp()`）。
    ///
    /// GET `/cgi-bin/getcallbackip`，返回 `{ "ip_list": [...] }` 中的
    /// ip_list。
    async fn get_callback_ip(&self) -> Result<Vec<String>, WxErrorException> {
        let config = self.wx_cp_config_storage();
        let response = self
            .get(&config.api_url(url_core::GET_CALLBACK_IP), "")
            .await?;
        parse_ip_list(&response)
    }

    /// 获取企业微信接口 IP 段（对应 Java `getApiDomainIp()`）。
    ///
    /// GET `/cgi-bin/get_api_domain_ip`
    /// （https://developer.work.weixin.qq.com/document/path/92520）。
    async fn get_api_domain_ip(&self) -> Result<Vec<String>, WxErrorException> {
        let config = self.wx_cp_config_storage();
        let response = self
            .get(&config.api_url(url_core::GET_API_DOMAIN_IP), "")
            .await?;
        parse_ip_list(&response)
    }

    /// 获取服务商凭证（对应 Java
    /// `BaseWxCpServiceImpl.getProviderToken(String, String)`）。
    ///
    /// POST `/cgi-bin/service/get_provider_token`，请求体
    /// `{"corpid":..., "provider_secret":...}`（镜像 Java Gson 的
    /// JsonObject 插入序；经 `post` 执行引擎自动追加 access_token，与
    /// Java `this.post(...)` 一致），响应解析为 `WxCpProviderToken`
    /// （provider_access_token/expires_in）
    /// （https://work.weixin.qq.com/api/doc#90001/90143/91200）。
    async fn get_provider_token(
        &self,
        corp_id: &str,
        provider_secret: &str,
    ) -> Result<WxCpProviderToken, WxErrorException> {
        let body = serde_json::json!({
            "corpid": corp_id,
            "provider_secret": provider_secret,
        })
        .to_string();
        let config = self.wx_cp_config_storage();
        let response = self
            .post(&config.api_url(url_tp::GET_PROVIDER_TOKEN), &body)
            .await?;
        WxCpProviderToken::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// GET 请求（对应 Java `get(String, String)`；query 为空传 ""，
    /// 对应 Java null）。
    ///
    /// 走统一管线 [`wx_rust_common::pipeline::execute_pipeline`]（经
    /// `execute_get_via_pipeline`：-1 指数退避重试 + token 失效单次重放；
    /// query 拼接语义内联于封装——原 `SimpleGetRequestExecutor` 路径）。
    async fn get(&self, url: &str, query_param: &str) -> Result<String, WxErrorException> {
        crate::api::r#impl::base_wx_cp_service_impl::execute_get_via_pipeline(
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
        crate::api::r#impl::base_wx_cp_service_impl::execute_post_via_pipeline(self, url, post_data)
            .await
    }

    /// 当不需要自动带 accessToken 的时候，可以用这个发起 post 请求
    /// （对应 Java `postWithoutToken(String, String)`）。
    async fn post_without_token(
        &self,
        url: &str,
        post_data: &str,
    ) -> Result<String, WxErrorException> {
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_cp_service_impl::execute_normal(
            self,
            &executor,
            url,
            post_data.to_string(),
        )
        .await
    }

    /// 使用会话存档 access token 发起 post 请求（对应 Java
    /// `postForMsgAudit(String, String)`）。
    ///
    /// 会话存档相关 API 需要使用会话存档专用的 secret 获取独立的
    /// access token；以 `execute_normal` 执行（不自动添加 token）。
    async fn post_for_msg_audit(
        &self,
        url: &str,
        post_data: &str,
    ) -> Result<String, WxErrorException> {
        let msg_audit_access_token = self.get_msg_audit_access_token_with_force(false).await?;
        let url_with_token = format!(
            "{url}{}access_token={msg_audit_access_token}",
            if url.contains('?') { "&" } else { "?" }
        );
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_cp_service_impl::execute_normal(
            self,
            &executor,
            &url_with_token,
            post_data.to_string(),
        )
        .await
    }

    /// 使用通讯录同步 access token 发起 get 请求（对应 Java
    /// `getForContact(String, String)`）。
    ///
    /// 通讯录同步相关 API 需要使用通讯录同步专用的 secret 获取独立的
    /// access token；以 `execute_normal` 执行（不自动添加 token）。
    async fn get_for_contact(
        &self,
        url: &str,
        query_param: &str,
    ) -> Result<String, WxErrorException> {
        let contact_access_token = self.get_contact_access_token_with_force(false).await?;
        let mut url_with_token = format!(
            "{url}{}access_token={contact_access_token}",
            if url.contains('?') { "&" } else { "?" }
        );
        if !query_param.is_empty() {
            url_with_token = format!("{url_with_token}&{query_param}");
        }
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_cp_service_impl::execute_normal(
            self,
            &executor,
            &url_with_token,
            String::new(),
        )
        .await
    }

    /// 使用通讯录同步 access token 发起 post 请求（对应 Java
    /// `postForContact(String, String)`）。
    async fn post_for_contact(
        &self,
        url: &str,
        post_data: &str,
    ) -> Result<String, WxErrorException> {
        let contact_access_token = self.get_contact_access_token_with_force(false).await?;
        let url_with_token = format!(
            "{url}{}access_token={contact_access_token}",
            if url.contains('?') { "&" } else { "?" }
        );
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        crate::api::r#impl::base_wx_cp_service_impl::execute_normal(
            self,
            &executor,
            &url_with_token,
            post_data.to_string(),
        )
        .await
    }

    /// 上传部门列表覆盖企业号上的部门信息（对应 Java `replaceParty(String)`）。
    async fn replace_party(&self, media_id: &str) -> Result<String, WxErrorException> {
        let body = serde_json::json!({ "media_id": media_id }).to_string();
        let config = self.wx_cp_config_storage();
        self.post(&config.api_url(url_core::BATCH_REPLACE_PARTY), &body)
            .await
    }

    /// 上传用户列表，增量更新成员（对应 Java `syncUser(String)`）。
    ///
    /// 返回异步任务 id（响应 `jobid` 字段）。
    async fn sync_user(&self, media_id: &str) -> Result<String, WxErrorException> {
        let body = serde_json::json!({ "media_id": media_id }).to_string();
        let config = self.wx_cp_config_storage();
        let response = self
            .post(&config.api_url(url_core::BATCH_SYNC_USER), &body)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("jobid")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "jobid 字段缺失"))
    }

    /// 上传用户列表覆盖企业号上的用户信息（对应 Java `replaceUser(String)`）。
    async fn replace_user(&self, media_id: &str) -> Result<String, WxErrorException> {
        let body = serde_json::json!({ "media_id": media_id }).to_string();
        let config = self.wx_cp_config_storage();
        self.post(&config.api_url(url_core::BATCH_REPLACE_USER), &body)
            .await
    }

    /// 获取异步任务结果（对应 Java `getTaskResult(String)`）。
    async fn get_task_result(&self, job_id: &str) -> Result<String, WxErrorException> {
        let config = self.wx_cp_config_storage();
        let url = format!("{}{job_id}", config.api_url(url_core::BATCH_GET_RESULT));
        self.get(&url, "").await
    }

    /// 初始化 http 请求对象（对应 Java `initHttp()`）。
    ///
    /// Rust 中 reqwest 客户端在服务构建时初始化（`WxCpServiceImpl::new_arc`），
    /// 无需动态初始化，默认空实现。
    fn init_http(&self) {}

    /// 构造扫码登录链接 - 构造独立窗口登录二维码（对应 Java
    /// `buildQrConnectUrl(String, String)`）。
    ///
    /// `redirect_uri` 需 URL 编码（encodeURIComponent 语义，对应 Java
    /// `URIUtil.encodeURIComponent`：保留 `A-Z a-z 0-9 - _ . ! ~ * ' ( )`，
    /// 空格编码为 `%20`）；`state` 用于保持请求和回调的状态，防止 csrf
    /// 攻击，建议带上（Java 语义：空 state trim 后拼接）。
    fn build_qr_connect_url(&self, redirect_uri: &str, state: &str) -> String {
        let config = self.wx_cp_config_storage();
        let encoded_redirect = encode_uri_component(redirect_uri);
        // Java String.format("%s", (Integer)null) 输出 "null"，严格镜像
        let agent_id = config
            .agent_id()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "null".to_string());
        format!(
            "https://open.work.weixin.qq.com/wwopen/sso/qrConnect?appid={}&agentid={}&redirect_uri={}&state={}",
            config.app_id(),
            agent_id,
            encoded_redirect,
            state.trim()
        )
    }

    // ---- 子服务（对应 Java WxCpService 的 `getXxxService()`；默认返回
    // None，由 WxCpServiceImpl 后续批次覆写为装配后的实例） ----

    /// 部门服务（对应 Java `getDepartmentService()`）。
    fn department_service(&self) -> Option<Arc<dyn WxCpDepartmentService>> {
        None
    }
    /// 素材服务（对应 Java `getMediaService()`）。
    fn media_service(&self) -> Option<Arc<dyn WxCpMediaService>> {
        None
    }
    /// 菜单服务（对应 Java `getMenuService()`）。
    fn menu_service(&self) -> Option<Arc<dyn WxCpMenuService>> {
        None
    }
    /// OAuth2 服务（对应 Java `getOauth2Service()`）。
    fn oauth2_service(&self) -> Option<Arc<dyn WxCpOAuth2Service>> {
        None
    }
    /// 标签服务（对应 Java `getTagService()`）。
    fn tag_service(&self) -> Option<Arc<dyn WxCpTagService>> {
        None
    }
    /// 成员服务（对应 Java `getUserService()`）。
    fn user_service(&self) -> Option<Arc<dyn WxCpUserService>> {
        None
    }
    /// 外部联系人服务（对应 Java `getExternalContactService()`）。
    fn external_contact_service(&self) -> Option<Arc<dyn WxCpExternalContactService>> {
        None
    }
    /// 群聊服务（对应 Java `getChatService()`）。
    fn chat_service(&self) -> Option<Arc<dyn WxCpChatService>> {
        None
    }
    /// 任务卡片服务（对应 Java `getTaskCardService()`）。
    fn task_card_service(&self) -> Option<Arc<dyn WxCpTaskCardService>> {
        None
    }
    /// 应用服务（对应 Java `getAgentService()`）。
    fn agent_service(&self) -> Option<Arc<dyn WxCpAgentService>> {
        None
    }
    /// 消息服务（对应 Java `getMessageService()`）。
    fn message_service(&self) -> Option<Arc<dyn WxCpMessageService>> {
        None
    }
    /// OA 服务（对应 Java `getOaService()`）。
    fn oa_service(&self) -> Option<Arc<dyn WxCpOaService>> {
        None
    }
    /// 家校应用复学码服务（对应 Java `getSchoolService()`）。
    fn school_service(&self) -> Option<Arc<dyn WxCpSchoolService>> {
        None
    }
    /// 家校沟通服务（对应 Java `getSchoolUserService()`）。
    fn school_user_service(&self) -> Option<Arc<dyn WxCpSchoolUserService>> {
        None
    }
    /// 家校应用健康上报服务（对应 Java `getSchoolHealthService()`）。
    fn school_health_service(&self) -> Option<Arc<dyn WxCpSchoolHealthService>> {
        None
    }
    /// 直播服务（对应 Java `getLivingService()`）。
    fn living_service(&self) -> Option<Arc<dyn WxCpLivingService>> {
        None
    }
    /// OA 自建应用服务（对应 Java `getOaAgentService()`）。
    fn oa_agent_service(&self) -> Option<Arc<dyn WxCpOaAgentService>> {
        None
    }
    /// OA 效率工具微盘服务（对应 Java `getOaWeDriveService()`）。
    fn oa_we_drive_service(&self) -> Option<Arc<dyn WxCpOaWeDriveService>> {
        None
    }
    /// OA 效率工具文档服务（对应 Java `getOaWeDocService()`）。
    fn oa_we_doc_service(&self) -> Option<Arc<dyn WxCpOaWeDocService>> {
        None
    }
    /// 会话存档服务（对应 Java `getMsgAuditService()`）。
    fn msg_audit_service(&self) -> Option<Arc<dyn WxCpMsgAuditService>> {
        None
    }
    /// 日历服务（对应 Java `getOaCalendarService()`）。
    fn oa_calendar_service(&self) -> Option<Arc<dyn WxCpOaCalendarService>> {
        None
    }
    /// 会议室服务（对应 Java `getOaMeetingRoomService()`）。
    fn oa_meeting_room_service(&self) -> Option<Arc<dyn WxCpOaMeetingRoomService>> {
        None
    }
    /// 日程服务（对应 Java `getOaScheduleService()`）。
    fn oa_schedule_service(&self) -> Option<Arc<dyn WxCpOaScheduleService>> {
        None
    }
    /// 群机器人消息推送服务（对应 Java `getGroupRobotService()`）。
    fn group_robot_service(&self) -> Option<Arc<dyn WxCpGroupRobotService>> {
        None
    }
    /// 工作台服务（对应 Java `getWorkBenchService()`）。
    fn work_bench_service(&self) -> Option<Arc<dyn WxCpAgentWorkBenchService>> {
        None
    }
    /// 微信客服服务（对应 Java `getKfService()`）。
    fn kf_service(&self) -> Option<Arc<dyn WxCpKfService>> {
        None
    }
    /// 异步导出服务（对应 Java `getExportService()`）。
    fn export_service(&self) -> Option<Arc<dyn WxCpExportService>> {
        None
    }
    /// 会议服务（对应 Java `getMeetingService()`）。
    fn meeting_service(&self) -> Option<Arc<dyn WxCpMeetingService>> {
        None
    }
    /// 企业互联服务（对应 Java `getCorpGroupService()`）。
    fn corp_group_service(&self) -> Option<Arc<dyn WxCpCorpGroupService>> {
        None
    }
    /// 智能机器人服务（对应 Java `getIntelligentRobotService()`）。
    fn intelligent_robot_service(&self) -> Option<Arc<dyn WxCpIntelligentRobotService>> {
        None
    }
    /// 人事助手服务（对应 Java `getHrService()`）。
    fn hr_service(&self) -> Option<Arc<dyn WxCpHrService>> {
        None
    }

    // ---- 子服务注入（对应 Java WxCpService 的 `setXxxService()`；
    // trait 默认空实现，由 WxCpServiceImpl 覆写存储） ----

    /// 设置成员服务（对应 Java `setUserService(WxCpUserService)`）。
    fn set_user_service(&self, _user_service: Arc<dyn WxCpUserService>) {}
    /// 设置部门服务（对应 Java `setDepartmentService(WxCpDepartmentService)`）。
    fn set_department_service(&self, _department_service: Arc<dyn WxCpDepartmentService>) {}
    /// 设置素材服务（对应 Java `setMediaService(WxCpMediaService)`）。
    fn set_media_service(&self, _media_service: Arc<dyn WxCpMediaService>) {}
    /// 设置菜单服务（对应 Java `setMenuService(WxCpMenuService)`）。
    fn set_menu_service(&self, _menu_service: Arc<dyn WxCpMenuService>) {}
    /// 设置 OAuth2 服务（对应 Java `setOauth2Service(WxCpOAuth2Service)`）。
    fn set_oauth2_service(&self, _oauth2_service: Arc<dyn WxCpOAuth2Service>) {}
    /// 设置标签服务（对应 Java `setTagService(WxCpTagService)`）。
    fn set_tag_service(&self, _tag_service: Arc<dyn WxCpTagService>) {}
    /// 设置微信客服服务（对应 Java `setKfService(WxCpKfService)`）。
    fn set_kf_service(&self, _kf_service: Arc<dyn WxCpKfService>) {}
    /// 设置异步导出服务（对应 Java `setExportService(WxCpExportService)`）。
    fn set_export_service(&self, _export_service: Arc<dyn WxCpExportService>) {}
}

/// 解析 `{"ip_list": [...]}` 响应中的 ip 列表（对应 Java
/// `BaseWxCpServiceImpl.getIp` 的 Gson 解析，私有辅助）。
fn parse_ip_list(response: &str) -> Result<Vec<String>, WxErrorException> {
    let json: serde_json::Value =
        serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
    match json.get("ip_list").and_then(|v| v.as_array()) {
        Some(arr) => arr
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| WxErrorException::from_code(-99, "ip_list 含非字符串元素"))
            })
            .collect(),
        None => Err(WxErrorException::from_code(-99, "ip_list 字段缺失")),
    }
}

/// URL 组件编码（对应 Java
/// `me.chanjar.weixin.common.util.http.URIUtil.encodeURIComponent`，与
/// JS `encodeURIComponent` 保留集一致）。
///
/// 保留集：`A-Z a-z 0-9 - _ . ! ~ * ' ( )`；其余字符（含空格 → `%20`）
/// 按 UTF-8 逐字节编码为 `%XX`（大写十六进制）。
///
/// 注：与 `percent_encoding::NON_ALPHANUMERIC` 的差异在于 `.`/`-`/`_`/
/// `!`/`~`/`*`/`'`/`(`/`)` 不编码（NON_ALPHANUMERIC 会把 `.` 编码为
/// `%2E`，与 Java 语义不符，Wave 3 C3 修复）。
fn encode_uri_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for b in input.bytes() {
        let c = b as char;
        // 保留集（镜像 Java ALLOWED_CHARS 常量）
        if c.is_ascii_alphanumeric()
            || matches!(c, '-' | '_' | '.' | '!' | '~' | '*' | '\'' | '(' | ')')
        {
            out.push(c);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
