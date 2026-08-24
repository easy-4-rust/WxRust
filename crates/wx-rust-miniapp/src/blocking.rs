//! feature="sync" 同步门面（`WxMaServiceBlocking`）。
//!
//! 对应 Java WxJava 同步调用习惯：为无法引入 async runtime 的调用方
//! （CLI、FFI、遗留同步代码）提供与 async 门面同参的同步方法。实现方式为
//! 惰性全局 current_thread tokio runtime（`OnceLock`，进程单例）+
//! `block_on` 逐调用驱动——**`block_on` 仅允许出现在本文件**
//! （CI 门禁 `scripts/check_block_on.sh`），async 路径行为零改动。
//!
//! 设计约束：
//! - 门面**不实现** `WxMaService` 等 async trait——类型上杜绝将其投入
//!   async 上下文造成的误用（例如作为子服务注入）。
//! - 首批暴露 3 个高频方法（业务 API + token）：`js_code_to_session`、
//!   `get_phone_number`、`get_access_token_sync`（对应 async 版
//!   `get_access_token`），签名与 async 版一致仅去掉 async。
//! - 不得在 tokio runtime 线程内调用（`block_on` 重入会 panic）；
//!   同步门面只在纯同步上下文使用。

use std::future::Future;
use std::sync::{Arc, OnceLock};

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::r#impl::WxMaServiceImpl;
use crate::bean::{WxMaJscode2SessionResult, WxMaPhoneNumberInfo};

/// 全局 current_thread runtime（惰性单例）。
///
/// current_thread 足够：`block_on` 期间会驱动本 runtime 上全部已 spawn 的
/// 任务（含调用前经 [`block_on`] 起动的 mock/辅助任务），无需多 worker。
static RUNTIME: OnceLock<Arc<tokio::runtime::Runtime>> = OnceLock::new();

fn runtime() -> Arc<tokio::runtime::Runtime> {
    RUNTIME
        .get_or_init(|| {
            Arc::new(
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("构建 current_thread tokio runtime 失败"),
            )
        })
        .clone()
}

/// 在同步门面的全局 runtime 上执行 future。
///
/// 供同步上下文完成必要的 async 起动工作（如测试中起动 mock 服务器），
/// 保证其任务与门面方法共享同一 runtime（`block_on` 驱动期间一并执行）。
/// 不得在 tokio runtime 线程内调用（重入 panic）。
pub fn block_on<F: Future>(future: F) -> F::Output {
    runtime().block_on(future)
}

/// 小程序服务同步门面。
///
/// 包装 [`WxMaServiceImpl`]，将 async 方法逐个映射为同步版本；不实现任何
/// async trait（见模块文档）。克隆开销低（`Arc` 字段）。
pub struct WxMaServiceBlocking {
    inner: Arc<WxMaServiceImpl>,
    rt: Arc<tokio::runtime::Runtime>,
}

impl WxMaServiceBlocking {
    /// 构建同步门面（runtime 为全局惰性单例，首个调用时创建）。
    pub fn new(inner: Arc<WxMaServiceImpl>) -> Self {
        Self {
            inner,
            rt: runtime(),
        }
    }

    /// 获取登录后的 session 信息（对应 async `js_code_to_session`）。
    ///
    /// GET `/sns/jscode2session`，token 自动获取注入；响应解析为
    /// `WxMaJscode2SessionResult`。
    pub fn js_code_to_session(
        &self,
        js_code: &str,
    ) -> Result<WxMaJscode2SessionResult, WxErrorException> {
        self.rt.block_on(WxMaService::js_code_to_session(
            self.inner.as_ref(),
            js_code,
        ))
    }

    /// 通过 code 获取手机号（对应 async `get_phone_number`）。
    ///
    /// POST `/wxa/business/getuserphonenumber`，请求体 `{"code": ...}`；
    /// 响应含 `phone_info` 时解析返回，否则返回 `None`（Java null 语义）。
    pub fn get_phone_number(
        &self,
        code: &str,
    ) -> Result<Option<WxMaPhoneNumberInfo>, WxErrorException> {
        self.rt
            .block_on(WxMaService::get_phone_number(self.inner.as_ref(), code))
    }

    /// 获取 access_token（对应 async `get_access_token`，不强制刷新）。
    ///
    /// 双检锁 + 缓存语义与 async 版一致；token 过期时经 HTTP 刷新并写回配置。
    pub fn get_access_token_sync(&self) -> Result<String, WxErrorException> {
        self.rt
            .block_on(WxMaService::get_access_token(self.inner.as_ref()))
    }
}
