//! 企业微信服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpServiceImpl`（继承
//! `WxCpServiceApacheHttpClientImpl` → `BaseWxCpServiceImpl`）：组合门面
//! trait 的默认实现 + 配置存储持有（对应 Java `configStorage` 字段）与
//! 执行参数（`retrySleepMillis`/`maxRetryTimes`）/会话管理器
//! （`StandardSessionManager`）。
//!
//! access_token/通讯录同步 access_token/会话存档 access_token/
//! jsapi_ticket/agent jsapi_ticket 的获取全部为双检锁模式（先判断过期，
//! 再拿锁后二次判断避免重刷，对应 Java `WxCpServiceImpl` 各方法），锁
//! 由 `WxCpConfigStorage` 提供（`access_token_lock`/`ticket_lock`/
//! `contact_access_token_lock`/`msg_audit_access_token_lock`/
//! `agent_jsapi_ticket_lock`）。
//!
//! 子服务装配（对应 Java Base 构造器中的 31 个子服务字段）以
//! `Weak<dyn WxCpService>` 注入子服务（Java `new WxCpUserServiceImpl(this)`
//! 的循环引用，Rust 用弱引用打破），一次性装配 31 个子服务（对应 Java
//! `BaseWxCpServiceImpl` 构造器）。
//!
//! 说明：
//! - Java `getRequestHttpClient`/`ApacheBasicResponseHandler` 的 HTTP 执行
//!   在 Rust 中以 `reqwest::Client` 表达（统一 HTTP 后端）。

use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;

use wx_rust_common::enums::TicketType;
use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};

use crate::api::r#impl::{
    WxCpAgentServiceImpl, WxCpAgentWorkBenchServiceImpl, WxCpChatServiceImpl,
    WxCpCorpGroupServiceImpl, WxCpDepartmentServiceImpl, WxCpExportServiceImpl,
    WxCpExternalContactServiceImpl, WxCpGroupRobotServiceImpl, WxCpHrServiceImpl,
    WxCpIntelligentRobotServiceImpl, WxCpKfServiceImpl, WxCpLivingServiceImpl,
    WxCpMediaServiceImpl, WxCpMeetingServiceImpl, WxCpMenuServiceImpl, WxCpMessageServiceImpl,
    WxCpMsgAuditServiceImpl, WxCpOAuth2ServiceImpl, WxCpOaAgentServiceImpl,
    WxCpOaCalendarServiceImpl, WxCpOaMeetingRoomServiceImpl, WxCpOaScheduleServiceImpl,
    WxCpOaServiceImpl, WxCpOaWeDocServiceImpl, WxCpOaWeDriveServiceImpl,
    WxCpSchoolHealthServiceImpl, WxCpSchoolServiceImpl, WxCpSchoolUserServiceImpl,
    WxCpTagServiceImpl, WxCpTaskCardServiceImpl, WxCpUserServiceImpl,
};
use crate::api::{
    WxCpAgentService, WxCpAgentWorkBenchService, WxCpChatService, WxCpCorpGroupService,
    WxCpDepartmentService, WxCpExportService, WxCpExternalContactService, WxCpGroupRobotService,
    WxCpHrService, WxCpIntelligentRobotService, WxCpKfService, WxCpLivingService, WxCpMediaService,
    WxCpMeetingService, WxCpMenuService, WxCpMessageService, WxCpMsgAuditService,
    WxCpOAuth2Service, WxCpOaAgentService, WxCpOaCalendarService, WxCpOaMeetingRoomService,
    WxCpOaScheduleService, WxCpOaService, WxCpOaWeDocService, WxCpOaWeDriveService,
    WxCpSchoolHealthService, WxCpSchoolService, WxCpSchoolUserService, WxCpService, WxCpTagService,
    WxCpTaskCardService, WxCpUserService,
};
use crate::config::WxCpConfigStorage;
use crate::enums::url_core;

/// 子服务集合（对应 Java `WxCpService` 各 `getXxxService()` 返回的子服务
/// 字段，装配于 `BaseWxCpServiceImpl` 构造器）。
struct SubServices {
    department: Arc<dyn WxCpDepartmentService>,
    media: Arc<dyn WxCpMediaService>,
    menu: Arc<dyn WxCpMenuService>,
    oauth2: Arc<dyn WxCpOAuth2Service>,
    tag: Arc<dyn WxCpTagService>,
    user: Arc<dyn WxCpUserService>,
    external_contact: Arc<dyn WxCpExternalContactService>,
    chat: Arc<dyn WxCpChatService>,
    task_card: Arc<dyn WxCpTaskCardService>,
    agent: Arc<dyn WxCpAgentService>,
    message: Arc<dyn WxCpMessageService>,
    oa: Arc<dyn WxCpOaService>,
    school: Arc<dyn WxCpSchoolService>,
    school_user: Arc<dyn WxCpSchoolUserService>,
    school_health: Arc<dyn WxCpSchoolHealthService>,
    living: Arc<dyn WxCpLivingService>,
    oa_agent: Arc<dyn WxCpOaAgentService>,
    oa_we_drive: Arc<dyn WxCpOaWeDriveService>,
    oa_we_doc: Arc<dyn WxCpOaWeDocService>,
    msg_audit: Arc<dyn WxCpMsgAuditService>,
    oa_calendar: Arc<dyn WxCpOaCalendarService>,
    oa_meeting_room: Arc<dyn WxCpOaMeetingRoomService>,
    oa_schedule: Arc<dyn WxCpOaScheduleService>,
    group_robot: Arc<dyn WxCpGroupRobotService>,
    work_bench: Arc<dyn WxCpAgentWorkBenchService>,
    kf: Arc<dyn WxCpKfService>,
    export: Arc<dyn WxCpExportService>,
    meeting: Arc<dyn WxCpMeetingService>,
    corp_group: Arc<dyn WxCpCorpGroupService>,
    intelligent_robot: Arc<dyn WxCpIntelligentRobotService>,
    hr: Arc<dyn WxCpHrService>,
}

/// 企业微信服务实现（reqwest HTTP 后端）。
pub struct WxCpServiceImpl {
    client: reqwest::Client,
    config_storage: Mutex<Arc<dyn WxCpConfigStorage>>,
    retry_sleep_millis: Mutex<i32>,
    max_retry_times: Mutex<i32>,
    session_manager: Mutex<Arc<dyn WxSessionManager>>,
    sub_services: OnceLock<SubServices>,
}

impl WxCpServiceImpl {
    /// 构建服务（对应 Java 无参构造 + `setWxCpConfigStorage` 装配；子服务
    /// 以 `Weak<dyn WxCpService>` 注入打破循环引用，对应 Java Base
    /// 构造器中的 31 个子服务字段）。
    ///
    /// # 参数
    /// - `config`：企业微信配置存储
    pub fn new_arc(config: Arc<dyn WxCpConfigStorage>) -> Arc<Self> {
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config_storage: Mutex::new(config),
            retry_sleep_millis: Mutex::new(1000),
            max_retry_times: Mutex::new(5),
            // 对应 Java BaseWxCpServiceImpl 默认装配 StandardSessionManager
            session_manager: Mutex::new(Arc::new(StandardSessionManager::new())),
            sub_services: OnceLock::new(),
        });
        // 先转 Arc<dyn WxCpService> 再降级为 Weak<dyn WxCpService>
        // （对应 Java 各子服务构造器注入 `this`，Rust 以弱引用打破循环）
        let dyn_arc: Arc<dyn WxCpService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc.sub_services.set(SubServices {
            department: Arc::new(WxCpDepartmentServiceImpl::new(weak.clone())),
            media: Arc::new(WxCpMediaServiceImpl::new(weak.clone())),
            menu: Arc::new(WxCpMenuServiceImpl::new(weak.clone())),
            oauth2: Arc::new(WxCpOAuth2ServiceImpl::new(weak.clone())),
            tag: Arc::new(WxCpTagServiceImpl::new(weak.clone())),
            user: Arc::new(WxCpUserServiceImpl::new(weak.clone())),
            external_contact: Arc::new(WxCpExternalContactServiceImpl::new(weak.clone())),
            chat: Arc::new(WxCpChatServiceImpl::new(weak.clone())),
            task_card: Arc::new(WxCpTaskCardServiceImpl::new(weak.clone())),
            agent: Arc::new(WxCpAgentServiceImpl::new(weak.clone())),
            message: Arc::new(WxCpMessageServiceImpl::new(weak.clone())),
            oa: Arc::new(WxCpOaServiceImpl::new(weak.clone())),
            school: Arc::new(WxCpSchoolServiceImpl::new(weak.clone())),
            school_user: Arc::new(WxCpSchoolUserServiceImpl::new(weak.clone())),
            school_health: Arc::new(WxCpSchoolHealthServiceImpl::new(weak.clone())),
            living: Arc::new(WxCpLivingServiceImpl::new(weak.clone())),
            oa_agent: Arc::new(WxCpOaAgentServiceImpl::new(weak.clone())),
            oa_we_drive: Arc::new(WxCpOaWeDriveServiceImpl::new(weak.clone())),
            oa_we_doc: Arc::new(WxCpOaWeDocServiceImpl::new(weak.clone())),
            msg_audit: Arc::new(WxCpMsgAuditServiceImpl::new(weak.clone())),
            oa_calendar: Arc::new(WxCpOaCalendarServiceImpl::new(weak.clone())),
            oa_meeting_room: Arc::new(WxCpOaMeetingRoomServiceImpl::new(weak.clone())),
            oa_schedule: Arc::new(WxCpOaScheduleServiceImpl::new(weak.clone())),
            group_robot: Arc::new(WxCpGroupRobotServiceImpl::new(weak.clone())),
            work_bench: Arc::new(WxCpAgentWorkBenchServiceImpl::new(weak.clone())),
            kf: Arc::new(WxCpKfServiceImpl::new(weak.clone())),
            export: Arc::new(WxCpExportServiceImpl::new(weak.clone())),
            meeting: Arc::new(WxCpMeetingServiceImpl::new(weak.clone())),
            corp_group: Arc::new(WxCpCorpGroupServiceImpl::new(weak.clone())),
            intelligent_robot: Arc::new(WxCpIntelligentRobotServiceImpl::new(weak.clone())),
            hr: Arc::new(WxCpHrServiceImpl::new(weak.clone())),
        });
        arc
    }

    /// 子服务集合。
    fn services(&self) -> &SubServices {
        self.sub_services.get().expect("子服务已在构建时安装")
    }

    /// 执行 token 获取请求（对应 Java `WxCpServiceImpl.getAccessToken` 内
    /// 的 `HttpGet` + `ApacheBasicResponseHandler`；Rust 以 reqwest GET
    /// 表达，代理配置暂不注入——reqwest 客户端在 `new_arc` 时统一构建）。
    async fn do_get_token_request(&self, url: &str) -> Result<String, WxErrorException> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        let body = resp
            .text()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        Ok(body)
    }

    /// 解析 token 响应并更新配置缓存（对应 Java `getAccessToken` 内
    /// `WxError.fromJson` + `WxAccessToken.fromJson` +
    /// `updateAccessToken`；Rust 以 serde_json 表达，ADAPTED）。
    ///
    /// 返回刷新后的 access_token。
    fn extract_and_update_access_token(
        &self,
        config: &dyn WxCpConfigStorage,
        result_content: &str,
    ) -> Result<String, WxErrorException> {
        let error =
            wx_rust_common::error::WxError::from_json_with_type(result_content, Some(WxType::Cp));
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
        config
            .access_token()
            .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"))
    }

    /// 拼接 token 获取地址（对应 Java
    /// `String.format(configStorage.getApiUrl(GET_TOKEN), corpId, corpSecret)`：
    /// `%s` 依次替换为 corpid/corpsecret）。
    fn build_get_token_url(config: &dyn WxCpConfigStorage, secret: &str) -> String {
        config
            .api_url(url_core::GET_TOKEN)
            .replacen("%s", config.app_id(), 1)
            .replacen("%s", secret, 1)
    }
}

#[async_trait]
impl WxCpService for WxCpServiceImpl {
    fn wx_cp_config_storage(&self) -> Arc<dyn WxCpConfigStorage> {
        self.config_storage.lock().unwrap().clone()
    }

    fn set_wx_cp_config_storage(&self, config: Arc<dyn WxCpConfigStorage>) {
        *self.config_storage.lock().unwrap() = config;
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn retry_sleep_millis(&self) -> i32 {
        *self.retry_sleep_millis.lock().unwrap()
    }

    fn max_retry_times(&self) -> i32 {
        *self.max_retry_times.lock().unwrap()
    }

    fn set_retry_sleep_millis(&self, retry_sleep_millis: i32) {
        *self.retry_sleep_millis.lock().unwrap() = retry_sleep_millis;
    }

    fn set_max_retry_times(&self, max_retry_times: i32) {
        *self.max_retry_times.lock().unwrap() = max_retry_times;
    }

    fn session_manager(&self) -> Option<Arc<dyn WxSessionManager>> {
        Some(self.session_manager.lock().unwrap().clone())
    }

    fn set_session_manager(&self, session_manager: Arc<dyn WxSessionManager>) {
        *self.session_manager.lock().unwrap() = session_manager;
    }

    async fn get_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_config_storage();
        if !config.is_access_token_expired() && !force_refresh {
            return config
                .access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
        }
        let lock = config.access_token_lock();
        let _guard = lock.lock().await;
        // 拿到锁之后，再次判断一下最新的 token 是否过期，避免重刷
        if !config.is_access_token_expired() && !force_refresh {
            return config
                .access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
        }
        let url = Self::build_get_token_url(config.as_ref(), config.secret());
        let result_content = self.do_get_token_request(&url).await?;
        self.extract_and_update_access_token(config.as_ref(), &result_content)
    }

    async fn get_contact_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_config_storage();
        if !config.is_contact_access_token_expired() && !force_refresh {
            return config
                .contact_access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "通讯录同步 access token 为空"));
        }
        let lock = config.contact_access_token_lock();
        let _guard = lock.lock().await;
        // 拿到锁之后，再次判断一下最新的 token 是否过期，避免重刷
        if !config.is_contact_access_token_expired() && !force_refresh {
            return config
                .contact_access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "通讯录同步 access token 为空"));
        }
        // 使用通讯录同步 secret 获取 access_token
        let contact_secret = config.contact_secret().unwrap_or_default();
        if contact_secret.trim().is_empty() {
            // Java `new WxErrorException("通讯录同步secret未配置")`：默认错误码 -99
            return Err(WxErrorException::from_code(-99, "通讯录同步secret未配置"));
        }
        let url = Self::build_get_token_url(config.as_ref(), &contact_secret);
        let result_content = self.do_get_token_request(&url).await?;
        // 解析并更新到通讯录同步 access token 缓存
        let error =
            wx_rust_common::error::WxError::from_json_with_type(&result_content, Some(WxType::Cp));
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        let json: serde_json::Value = serde_json::from_str(&result_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let access_token = json
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WxErrorException::from_code(-99, "access_token 字段缺失"))?
            .to_string();
        let expires_in = json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        config.update_contact_access_token(&access_token, expires_in);
        config
            .contact_access_token()
            .ok_or_else(|| WxErrorException::from_code(-99, "通讯录同步 access token 为空"))
    }

    async fn get_msg_audit_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_config_storage();
        if !config.is_msg_audit_access_token_expired() && !force_refresh {
            return config
                .msg_audit_access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "会话存档 access token 为空"));
        }
        let lock = config.msg_audit_access_token_lock();
        let _guard = lock.lock().await;
        // 拿到锁之后，再次判断一下最新的 token 是否过期，避免重刷
        if !config.is_msg_audit_access_token_expired() && !force_refresh {
            return config
                .msg_audit_access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "会话存档 access token 为空"));
        }
        // 使用会话存档 secret 获取 access_token
        let msg_audit_secret = config.msg_audit_secret().unwrap_or_default();
        if msg_audit_secret.trim().is_empty() {
            // Java `new WxErrorException("会话存档secret未配置")`：默认错误码 -99
            return Err(WxErrorException::from_code(-99, "会话存档secret未配置"));
        }
        let url = Self::build_get_token_url(config.as_ref(), &msg_audit_secret);
        let result_content = self.do_get_token_request(&url).await?;
        // 解析并更新到会话存档 access token 缓存
        let error =
            wx_rust_common::error::WxError::from_json_with_type(&result_content, Some(WxType::Cp));
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        let json: serde_json::Value = serde_json::from_str(&result_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let access_token = json
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WxErrorException::from_code(-99, "access_token 字段缺失"))?
            .to_string();
        let expires_in = json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        config.update_msg_audit_access_token(&access_token, expires_in);
        config
            .msg_audit_access_token()
            .ok_or_else(|| WxErrorException::from_code(-99, "会话存档 access token 为空"))
    }

    async fn get_jsapi_ticket_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_config_storage();
        if force_refresh {
            config.expire_ticket(TicketType::Jsapi);
        }
        if config.is_ticket_expired(TicketType::Jsapi) {
            let lock = config.ticket_lock(TicketType::Jsapi);
            let _guard = lock.lock().await;
            // 拿到锁之后，再次判断一下最新的 ticket 是否过期，避免重刷
            if config.is_ticket_expired(TicketType::Jsapi) {
                let url = config.api_url(url_core::GET_JSAPI_TICKET);
                // Java `this.get(url, null)`：走标准执行引擎（自动带 token）
                let response_content = self.get(&url, "").await?;
                let json: serde_json::Value = serde_json::from_str(&response_content)
                    .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                let ticket = json
                    .get("ticket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| WxErrorException::from_code(-99, "ticket 字段缺失"))?
                    .to_string();
                let expires_in =
                    json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                config.update_ticket(TicketType::Jsapi, &ticket, expires_in);
            }
        }
        config
            .ticket(TicketType::Jsapi)
            .ok_or_else(|| WxErrorException::from_code(-99, "jsapi ticket 为空"))
    }

    async fn get_agent_jsapi_ticket_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_config_storage();
        if force_refresh {
            config.expire_agent_jsapi_ticket();
        }
        if config.is_agent_jsapi_ticket_expired() {
            let lock = config.agent_jsapi_ticket_lock();
            let _guard = lock.lock().await;
            // 拿到锁之后，再次判断一下最新的 ticket 是否过期，避免重刷
            if config.is_agent_jsapi_ticket_expired() {
                let url = config.api_url(url_core::GET_AGENT_CONFIG_TICKET);
                // Java `this.get(url, null)`：走标准执行引擎（自动带 token）
                let response_content = self.get(&url, "").await?;
                let json: serde_json::Value = serde_json::from_str(&response_content)
                    .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                let ticket = json
                    .get("ticket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| WxErrorException::from_code(-99, "ticket 字段缺失"))?
                    .to_string();
                let expires_in =
                    json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                config.update_agent_jsapi_ticket(&ticket, expires_in);
            }
        }
        config
            .agent_jsapi_ticket()
            .ok_or_else(|| WxErrorException::from_code(-99, "agent jsapi ticket 为空"))
    }

    // ---- 子服务 getter（对应 Java `WxCpService.getXxxService()`，覆写
    // trait 默认 `None`，返回装配后的实例） ----

    fn department_service(&self) -> Option<Arc<dyn WxCpDepartmentService>> {
        Some(self.services().department.clone())
    }

    fn media_service(&self) -> Option<Arc<dyn WxCpMediaService>> {
        Some(self.services().media.clone())
    }

    fn menu_service(&self) -> Option<Arc<dyn WxCpMenuService>> {
        Some(self.services().menu.clone())
    }

    fn oauth2_service(&self) -> Option<Arc<dyn WxCpOAuth2Service>> {
        Some(self.services().oauth2.clone())
    }

    fn tag_service(&self) -> Option<Arc<dyn WxCpTagService>> {
        Some(self.services().tag.clone())
    }

    fn user_service(&self) -> Option<Arc<dyn WxCpUserService>> {
        Some(self.services().user.clone())
    }

    fn external_contact_service(&self) -> Option<Arc<dyn WxCpExternalContactService>> {
        Some(self.services().external_contact.clone())
    }

    fn chat_service(&self) -> Option<Arc<dyn WxCpChatService>> {
        Some(self.services().chat.clone())
    }

    fn task_card_service(&self) -> Option<Arc<dyn WxCpTaskCardService>> {
        Some(self.services().task_card.clone())
    }

    fn agent_service(&self) -> Option<Arc<dyn WxCpAgentService>> {
        Some(self.services().agent.clone())
    }

    fn message_service(&self) -> Option<Arc<dyn WxCpMessageService>> {
        Some(self.services().message.clone())
    }

    fn oa_service(&self) -> Option<Arc<dyn WxCpOaService>> {
        Some(self.services().oa.clone())
    }

    fn school_service(&self) -> Option<Arc<dyn WxCpSchoolService>> {
        Some(self.services().school.clone())
    }

    fn school_user_service(&self) -> Option<Arc<dyn WxCpSchoolUserService>> {
        Some(self.services().school_user.clone())
    }

    fn school_health_service(&self) -> Option<Arc<dyn WxCpSchoolHealthService>> {
        Some(self.services().school_health.clone())
    }

    fn living_service(&self) -> Option<Arc<dyn WxCpLivingService>> {
        Some(self.services().living.clone())
    }

    fn oa_agent_service(&self) -> Option<Arc<dyn WxCpOaAgentService>> {
        Some(self.services().oa_agent.clone())
    }

    fn oa_we_drive_service(&self) -> Option<Arc<dyn WxCpOaWeDriveService>> {
        Some(self.services().oa_we_drive.clone())
    }

    fn oa_we_doc_service(&self) -> Option<Arc<dyn WxCpOaWeDocService>> {
        Some(self.services().oa_we_doc.clone())
    }

    fn msg_audit_service(&self) -> Option<Arc<dyn WxCpMsgAuditService>> {
        Some(self.services().msg_audit.clone())
    }

    fn oa_calendar_service(&self) -> Option<Arc<dyn WxCpOaCalendarService>> {
        Some(self.services().oa_calendar.clone())
    }

    fn oa_meeting_room_service(&self) -> Option<Arc<dyn WxCpOaMeetingRoomService>> {
        Some(self.services().oa_meeting_room.clone())
    }

    fn oa_schedule_service(&self) -> Option<Arc<dyn WxCpOaScheduleService>> {
        Some(self.services().oa_schedule.clone())
    }

    fn group_robot_service(&self) -> Option<Arc<dyn WxCpGroupRobotService>> {
        Some(self.services().group_robot.clone())
    }

    fn work_bench_service(&self) -> Option<Arc<dyn WxCpAgentWorkBenchService>> {
        Some(self.services().work_bench.clone())
    }

    fn kf_service(&self) -> Option<Arc<dyn WxCpKfService>> {
        Some(self.services().kf.clone())
    }

    fn export_service(&self) -> Option<Arc<dyn WxCpExportService>> {
        Some(self.services().export.clone())
    }

    fn meeting_service(&self) -> Option<Arc<dyn WxCpMeetingService>> {
        Some(self.services().meeting.clone())
    }

    fn corp_group_service(&self) -> Option<Arc<dyn WxCpCorpGroupService>> {
        Some(self.services().corp_group.clone())
    }

    fn intelligent_robot_service(&self) -> Option<Arc<dyn WxCpIntelligentRobotService>> {
        Some(self.services().intelligent_robot.clone())
    }

    fn hr_service(&self) -> Option<Arc<dyn WxCpHrService>> {
        Some(self.services().hr.clone())
    }
}
