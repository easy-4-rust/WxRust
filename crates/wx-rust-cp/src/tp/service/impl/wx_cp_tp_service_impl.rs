//! 企业微信第三方应用服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpServiceImpl`
//! （继承 `WxCpTpServiceApacheHttpClientImpl` → `BaseWxCpTpServiceImpl`）：
//! 组合门面 trait 的默认实现 + 配置存储持有 + 执行参数
//! （`retrySleepMillis`/`maxRetryTimes`）+ `StandardSessionManager` +
//! 12 个子服务装配（对应 Java Base 构造器字段，以 `Weak<dyn WxCpTpService>`
//! 注入打破循环引用）。
//!
//! suite access token 的获取为双检锁模式（对应 Java
//! `WxCpTpServiceOkHttpImpl.getSuiteAccessToken(boolean)`：先判断过期，
//! 再拿锁后二次判断避免重刷），锁由 `WxCpTpConfigStorage::suite_access_token_lock`
//! 提供；HTTP 执行经 `execute_with_retry_tp` 引擎（自动带 suite token）。

use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};

use crate::config::WxCpTpConfigStorage;
use crate::enums::url_tp;
use crate::tp::service::r#impl::{
    WxCpTpContactServiceImpl, WxCpTpCustomizedServiceImpl, WxCpTpDepartmentServiceImpl,
    WxCpTpEditionServiceImpl, WxCpTpIdConvertServiceImpl, WxCpTpLicenseServiceImpl,
    WxCpTpMediaServiceImpl, WxCpTpMessageServiceImpl, WxCpTpOAServiceImpl, WxCpTpOAuth2ServiceImpl,
    WxCpTpOrderServiceImpl, WxCpTpTagServiceImpl, WxCpTpUserServiceImpl,
};
use crate::tp::service::{
    WxCpTpContactService, WxCpTpCustomizedService, WxCpTpDepartmentService, WxCpTpEditionService,
    WxCpTpIdConvertService, WxCpTpLicenseService, WxCpTpMediaService, WxCpTpMessageService,
    WxCpTpOAService, WxCpTpOAuth2Service, WxCpTpOrderService, WxCpTpService, WxCpTpTagService,
    WxCpTpUserService,
};

/// 子服务集合（对应 Java `BaseWxCpTpServiceImpl` 构造器的 12 个子服务
/// 字段）。
struct TpSubServices {
    contact: Arc<dyn WxCpTpContactService>,
    department: Arc<dyn WxCpTpDepartmentService>,
    media: Arc<dyn WxCpTpMediaService>,
    oa: Arc<dyn WxCpTpOAService>,
    user: Arc<dyn WxCpTpUserService>,
    order: Arc<dyn WxCpTpOrderService>,
    edition: Arc<dyn WxCpTpEditionService>,
    license: Arc<dyn WxCpTpLicenseService>,
    id_convert: Arc<dyn WxCpTpIdConvertService>,
    oauth2: Arc<dyn WxCpTpOAuth2Service>,
    customized: Arc<dyn WxCpTpCustomizedService>,
    message: Arc<dyn WxCpTpMessageService>,
    tag: Arc<dyn WxCpTpTagService>,
}

/// 企业微信第三方应用服务实现（reqwest HTTP 后端）。
pub struct WxCpTpServiceImpl {
    client: reqwest::Client,
    config_storage: Mutex<Arc<dyn WxCpTpConfigStorage>>,
    retry_sleep_millis: Mutex<i32>,
    max_retry_times: Mutex<i32>,
    session_manager: Arc<dyn WxSessionManager>,
    sub_services: OnceLock<TpSubServices>,
}

impl WxCpTpServiceImpl {
    /// 构建服务（对应 Java 无参构造 + `setWxCpTpConfigStorage` 装配；
    /// 子服务以 `Weak<dyn WxCpTpService>` 注入打破循环引用）。
    ///
    /// # 参数
    /// - `config`：第三方应用配置存储
    pub fn new_arc(config: Arc<dyn WxCpTpConfigStorage>) -> Arc<Self> {
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config_storage: Mutex::new(config),
            retry_sleep_millis: Mutex::new(1000),
            max_retry_times: Mutex::new(5),
            // 对应 Java Base 默认装配 StandardSessionManager
            session_manager: Arc::new(StandardSessionManager::new()),
            sub_services: OnceLock::new(),
        });
        // 先转 Arc<dyn WxCpTpService> 再降级为 Weak（对应 Java 子服务
        // 构造器注入 `this`，Rust 以弱引用打破循环）
        let dyn_arc: Arc<dyn WxCpTpService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc.sub_services.set(TpSubServices {
            contact: Arc::new(WxCpTpContactServiceImpl::new(weak.clone())),
            department: Arc::new(WxCpTpDepartmentServiceImpl::new(weak.clone())),
            media: Arc::new(WxCpTpMediaServiceImpl::new(weak.clone())),
            oa: Arc::new(WxCpTpOAServiceImpl::new(weak.clone())),
            user: Arc::new(WxCpTpUserServiceImpl::new(weak.clone())),
            order: Arc::new(WxCpTpOrderServiceImpl::new(weak.clone())),
            edition: Arc::new(WxCpTpEditionServiceImpl::new(weak.clone())),
            license: Arc::new(WxCpTpLicenseServiceImpl::new(weak.clone())),
            id_convert: Arc::new(WxCpTpIdConvertServiceImpl::new(weak.clone())),
            oauth2: Arc::new(WxCpTpOAuth2ServiceImpl::new(weak.clone())),
            customized: Arc::new(WxCpTpCustomizedServiceImpl::new(weak.clone())),
            message: Arc::new(WxCpTpMessageServiceImpl::new(weak.clone())),
            tag: Arc::new(WxCpTpTagServiceImpl::new(weak.clone())),
        });
        arc
    }

    /// 子服务集合。
    fn services(&self) -> &TpSubServices {
        self.sub_services.get().expect("子服务已在构建时安装")
    }
}

#[async_trait]
impl WxCpTpService for WxCpTpServiceImpl {
    fn wx_cp_tp_config_storage(&self) -> Arc<dyn WxCpTpConfigStorage> {
        self.config_storage.lock().unwrap().clone()
    }

    fn set_wx_cp_tp_config_storage(&self, config: Arc<dyn WxCpTpConfigStorage>) {
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

    fn session_manager(&self) -> Arc<dyn WxSessionManager> {
        self.session_manager.clone()
    }

    /// 获取 suite_access_token（可强制刷新，线程安全，对应 Java
    /// `WxCpTpServiceOkHttpImpl.getSuiteAccessToken(boolean)`）。
    ///
    /// 双检锁（`suite_access_token_lock`）保证多线程同时刷新时只刷新
    /// 一次；请求体 `{suite_id, suite_secret, suite_ticket}` POST
    /// `/cgi-bin/service/get_suite_token`（不带 suite token）。
    async fn get_suite_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_tp_config_storage();
        if !config.is_suite_access_token_expired() && !force_refresh {
            return config
                .suite_access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "suite access token 为空"));
        }
        let lock = config.suite_access_token_lock();
        let _guard = lock.lock().await;
        // 拿到锁之后，再次判断一下最新的 token 是否过期，避免重刷
        if !config.is_suite_access_token_expired() && !force_refresh {
            return config
                .suite_access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "suite access token 为空"));
        }
        // 对应 Java jsonObject：suite_id/suite_secret/suite_ticket
        let suite_ticket = self.get_suite_ticket().await?;
        let body = serde_json::json!({
            "suite_id": config.suite_id(),
            "suite_secret": config.suite_secret(),
            "suite_ticket": suite_ticket,
        })
        .to_string();
        let url = config.api_url(url_tp::GET_SUITE_TOKEN);
        let result_content = self.post_without_suite_token(&url, &body, true).await?;

        // 对应 Java WxError.fromJson(resultContent, WxType.CP)
        let error = wx_rust_common::error::WxError::from_json_with_type(
            &result_content,
            Some(wx_rust_common::enums::WxType::Cp),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        let json: serde_json::Value = serde_json::from_str(&result_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let suite_access_token = json
            .get("suite_access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WxErrorException::from_code(-99, "suite_access_token 字段缺失"))?
            .to_string();
        let expires_in = json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        config.update_suite_access_token(&suite_access_token, expires_in);
        config
            .suite_access_token()
            .ok_or_else(|| WxErrorException::from_code(-99, "suite access token 为空"))
    }

    // ---- 子服务 getter（对应 Java `getWxCpTpXxxService()`） ----

    fn wx_cp_tp_contact_service(&self) -> Option<Arc<dyn WxCpTpContactService>> {
        Some(self.services().contact.clone())
    }

    fn wx_cp_tp_department_service(&self) -> Option<Arc<dyn WxCpTpDepartmentService>> {
        Some(self.services().department.clone())
    }

    fn wx_cp_tp_media_service(&self) -> Option<Arc<dyn WxCpTpMediaService>> {
        Some(self.services().media.clone())
    }

    fn wx_cp_tp_oa_service(&self) -> Option<Arc<dyn WxCpTpOAService>> {
        Some(self.services().oa.clone())
    }

    fn wx_cp_tp_user_service(&self) -> Option<Arc<dyn WxCpTpUserService>> {
        Some(self.services().user.clone())
    }

    fn wx_cp_tp_license_service(&self) -> Option<Arc<dyn WxCpTpLicenseService>> {
        Some(self.services().license.clone())
    }

    fn wx_cp_tp_message_service(&self) -> Option<Arc<dyn WxCpTpMessageService>> {
        Some(self.services().message.clone())
    }

    fn wx_cp_tp_order_service(&self) -> Option<Arc<dyn WxCpTpOrderService>> {
        Some(self.services().order.clone())
    }

    fn wx_cp_tp_edition_service(&self) -> Option<Arc<dyn WxCpTpEditionService>> {
        Some(self.services().edition.clone())
    }

    fn wx_cp_tp_id_convert_service(&self) -> Option<Arc<dyn WxCpTpIdConvertService>> {
        Some(self.services().id_convert.clone())
    }

    fn wx_cp_tp_o_auth2_service(&self) -> Option<Arc<dyn WxCpTpOAuth2Service>> {
        Some(self.services().oauth2.clone())
    }

    fn wx_cp_tp_customized_service(&self) -> Option<Arc<dyn WxCpTpCustomizedService>> {
        Some(self.services().customized.clone())
    }

    fn wx_cp_tp_tag_service(&self) -> Option<Arc<dyn WxCpTpTagService>> {
        Some(self.services().tag.clone())
    }
}
