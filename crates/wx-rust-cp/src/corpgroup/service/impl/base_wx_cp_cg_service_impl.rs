//! 企业互联基础实现（对应 Java
//! `me.chanjar.weixin.cp.corpgroup.service.impl.BaseWxCpCgServiceImpl`）。
//!
//! Java 的 `BaseWxCpCgServiceImpl` 承载执行引擎（指数退避重试 +
//! corp_access_token 42009 自动单次刷新 + `withoutCorpAccessToken` 通道）
//! 与互联企业服务装配（`new WxCpLinkedCorpServiceImpl(this)`）。Rust 中
//! trait 无法携带泛型方法（破坏 dyn 兼容），执行引擎抽为本模块的泛型
//! 自由函数（与 `base_wx_cp_tp_service_impl` 同一设计原则），由
//! `WxCpCgService` trait 的 get/post 默认实现调用；具体实现
//! `WxCpCgServiceImpl`（对应 Java HTTP 后端实现，Java 各 HTTP 后端
//! 以 PLATFORM_NA 归类，Rust reqwest 单一后端）亦位于本文件。

use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::RequestExecutor;

use crate::api::WxCpService;
use crate::bean::WxCpCorpGroupCorpGetTokenReq;
use crate::config::WxCpCorpGroupConfigStorage;
use crate::corpgroup::service::r#impl::WxCpLinkedCorpServiceImpl;
use crate::corpgroup::service::{WxCpCgService, WxCpLinkedCorpService};

/// 执行请求（对应 Java `BaseWxCpCgServiceImpl.execute`）。
///
/// 策略：错误码 -1（系统繁忙）时指数退避重试（`retrySleepMillis << n`，
/// 最多 `maxRetryTimes` 次）；其余错误直接上抛；重试超限抛
/// `微信服务端异常，超出重试次数`（Java `WxRuntimeException` → Rust
/// `WxErrorException::from_code(-99, ...)`，ADAPTED）。
pub async fn execute_with_retry_cg<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: E,
    without_corp_access_token: bool,
    req: &WxCpCorpGroupCorpGetTokenReq,
) -> Result<T, WxErrorException>
where
    S: WxCpCgService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    let max_retry_times = svc.max_retry_times();
    let retry_sleep_millis = svc.retry_sleep_millis();

    let mut retry_times = 0;
    loop {
        match execute_internal_cg(
            svc,
            executor,
            uri,
            &data,
            without_corp_access_token,
            false,
            req,
        )
        .await
        {
            Ok(result) => return Ok(result),
            Err(e) => {
                // -1 系统繁忙，retrySleepMillis * 2^retryTimes 后重试
                if e.error_code() == Some(-1) {
                    if retry_times + 1 > max_retry_times {
                        return Err(WxErrorException::from_code(
                            -99,
                            "微信服务端异常，超出重试次数",
                        ));
                    }
                    let sleep_millis = retry_sleep_millis * (1 << retry_times);
                    tokio::time::sleep(std::time::Duration::from_millis(sleep_millis as u64)).await;
                } else {
                    return Err(e);
                }
            }
        }
        retry_times += 1;
        if retry_times > max_retry_times {
            break;
        }
    }
    Err(WxErrorException::from_code(
        -99,
        "微信服务端异常，超出重试次数",
    ))
}

/// 执行内部请求（对应 Java `executeInternal`）。
///
/// corp access token 注入 + 42009（corp_access_token 已过期）时强制过期
/// 并自动单次刷新后重试（`do_not_auto_refresh` 防无限递归）。
pub async fn execute_internal_cg<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: &E,
    without_corp_access_token: bool,
    do_not_auto_refresh: bool,
    req: &WxCpCorpGroupCorpGetTokenReq,
) -> Result<T, WxErrorException>
where
    S: WxCpCgService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    if uri.contains("access_token=") {
        return Err(WxErrorException::from_code(
            -99,
            format!("uri参数中不允许有access_token: {uri}"),
        ));
    }

    let config = svc.wx_cp_corp_group_config_storage();
    let mut do_not_auto_refresh = do_not_auto_refresh;
    loop {
        // 每次循环重算 uri（42009 刷新后携带新 token，对应 Java 递归
        // execute → executeInternal 重算 uriWithAccessToken）
        let uri_with_access_token = if !without_corp_access_token {
            let corp_access_token = svc
                .get_corp_access_token(&req.corp_id, Some(req.agent_id), Some(req.business_type))
                .await?;
            if uri.contains('?') {
                format!("{uri}&access_token={corp_access_token}")
            } else {
                format!("{uri}?access_token={corp_access_token}")
            }
        } else {
            uri.to_string()
        };

        match executor
            .execute(&uri_with_access_token, data.clone(), WxType::Cp)
            .await
        {
            Ok(result) => return Ok(result),
            Err(e) => {
                let code = e.error_code();
                // 42009 corp_access_token 已过期：强制过期 + 自动单次
                // 刷新后重试（对应 Java CODE_42009 分支）
                if code == Some(42009) {
                    config.expire_corp_access_token(&req.corp_id, Some(req.agent_id));
                    if config.auto_refresh_token() && !do_not_auto_refresh {
                        do_not_auto_refresh = true;
                        continue;
                    }
                }
                if let Some(code) = code {
                    if code != 0 {
                        return Err(e);
                    }
                    // Java 对错误码 0 的异常返回 null；标准执行器对
                    // errcode!=0 已抛错，此处仅理论可达（ADAPTED）
                    return Err(e);
                }
                return Err(e);
            }
        }
    }
}

/// 企业互联集团服务实现（reqwest HTTP 后端；对应 Java
/// `WxCpCgServiceApacheHttpClientImpl`/`WxCpCgServiceHttpComponentsImpl`
/// 的 Rust 单一后端承载，Java HTTP 后端变体以 PLATFORM_NA 归类）。
pub struct WxCpCgServiceImpl {
    client: reqwest::Client,
    config_storage: Mutex<Arc<dyn WxCpCorpGroupConfigStorage>>,
    wx_cp_service: Mutex<Option<Arc<dyn WxCpService>>>,
    retry_sleep_millis: Mutex<i32>,
    max_retry_times: Mutex<i32>,
    linked_corp: OnceLock<Arc<dyn WxCpLinkedCorpService>>,
}

impl WxCpCgServiceImpl {
    /// 构建服务（对应 Java 构造 + `setWxCpCorpGroupConfigStorage`/
    /// `setWxCpService` 装配；互联企业服务以 `Weak<dyn WxCpCgService>`
    /// 注入打破循环引用，对应 Java `new WxCpLinkedCorpServiceImpl(this)`）。
    ///
    /// # 参数
    /// - `config`：企业互联配置存储
    /// - `wx_cp_service`：企业微信服务（用于获取本企业 token）
    pub fn new_arc(
        config: Arc<dyn WxCpCorpGroupConfigStorage>,
        wx_cp_service: Arc<dyn WxCpService>,
    ) -> Arc<Self> {
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config_storage: Mutex::new(config),
            wx_cp_service: Mutex::new(Some(wx_cp_service)),
            retry_sleep_millis: Mutex::new(1000),
            max_retry_times: Mutex::new(5),
            linked_corp: OnceLock::new(),
        });
        let dyn_arc: Arc<dyn WxCpCgService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc
            .linked_corp
            .set(Arc::new(WxCpLinkedCorpServiceImpl::new(weak)));
        arc
    }
}

#[async_trait]
impl WxCpCgService for WxCpCgServiceImpl {
    fn wx_cp_corp_group_config_storage(&self) -> Arc<dyn WxCpCorpGroupConfigStorage> {
        self.config_storage.lock().unwrap().clone()
    }

    fn set_wx_cp_corp_group_config_storage(&self, config: Arc<dyn WxCpCorpGroupConfigStorage>) {
        *self.config_storage.lock().unwrap() = config;
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn set_wx_cp_service(&self, service: Arc<dyn WxCpService>) {
        *self.wx_cp_service.lock().unwrap() = Some(service);
    }

    fn wx_cp_service(&self) -> Option<Arc<dyn WxCpService>> {
        self.wx_cp_service.lock().unwrap().clone()
    }

    fn linked_corp_service(&self) -> Arc<dyn WxCpLinkedCorpService> {
        self.linked_corp
            .get()
            .expect("互联企业服务已在构建时安装")
            .clone()
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
}
