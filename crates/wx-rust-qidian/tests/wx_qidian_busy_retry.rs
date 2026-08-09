//! 系统繁忙重试测试（镜像 Java `WxMpBusyRetryTest`）。
//!
//! Java 以覆写 `executeInternal` 恒抛 `WxErrorException("something")`
//! （错误码为 `DEFAULT_ERROR_CODE` = -1）模拟系统繁忙，断言重试
//! `maxRetryTimes` 次后抛 `WxRuntimeException`；本文件以恒返回错误码 -1
//! 的自定义执行器镜像同一语义，并验证线程池复用场景下重试次数不变。

mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::RequestExecutor;

use wx_rust_qidian::api::WxQidianService;
use wx_rust_qidian::api::r#impl::WxQidianServiceImpl;

/// 恒返回错误码 -1（系统繁忙）的测试执行器（对应 Java 覆写的
/// `executeInternal` 抛 `new WxErrorException("something")`）。
struct BusyExecutor {
    calls: Arc<AtomicUsize>,
}

impl BusyExecutor {
    fn new(calls: Arc<AtomicUsize>) -> Self {
        Self { calls }
    }
}

#[async_trait]
impl RequestExecutor<String, String> for BusyExecutor {
    async fn execute(
        &self,
        _uri: &str,
        _data: String,
        _wx_type: WxType,
    ) -> Result<String, WxErrorException> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        // 对应 Java `WxErrorException(String)`：错误码 DEFAULT_ERROR_CODE(-1)
        Err(WxErrorException::from_code(-1, "something"))
    }
}

/// 镜像 Java `WxMpBusyRetryTest.testRetry`：`maxRetryTimes=3`、
/// `retrySleepMillis=500`（本测试用 1ms 加速），execute 恒失败 → 重试
/// 3 次后抛出"超出重试次数"运行时错误（对应 Java `WxRuntimeException`）。
#[tokio::test]
async fn test_retry() {
    let service = build_busy_service();
    let calls = Arc::new(AtomicUsize::new(0));
    let executor = Arc::new(BusyExecutor::new(calls.clone()));

    // 预置有效 access_token，避免 token 请求（对应 Java 覆写 executeInternal
    // 不触网）
    let config = service.config_storage();
    config.update_access_token("pre-token", 7200);

    let result = wx_rust_qidian::api::r#impl::execute_with_retry(
        service.as_ref(),
        executor.as_ref(),
        "https://api.qidian.qq.com/cgi-bin/call/dial/getivrlist",
        String::new(),
    )
    .await;
    assert!(result.is_err(), "重试超限应返回错误");
    assert!(
        matches!(result, Err(WxErrorException::Runtime(_))),
        "对应 Java WxRuntimeException：{result:?}"
    );
    // 首次执行 + 3 次重试 = 4 次
    assert_eq!(
        calls.load(Ordering::SeqCst),
        4,
        "执行次数 = 1 + maxRetryTimes"
    );
}

/// 镜像 Java `WxMpBusyRetryTest.testRetryInThreadPool`：线程池中的线程
/// 复用场景下仍保证相同的重试次数（本文件以两次顺序执行断言同一语义）。
#[tokio::test]
async fn test_retry_in_tasks() {
    let service = build_busy_service();
    let calls = Arc::new(AtomicUsize::new(0));
    let executor = Arc::new(BusyExecutor::new(calls.clone()));
    let config = service.config_storage();
    config.update_access_token("pre-token", 7200);

    // 两个并发任务（对应 Java 单线程池提交两次）
    let mut handles = Vec::new();
    for _ in 0..2 {
        let svc = service.clone();
        let exec = executor.clone();
        handles.push(tokio::spawn(async move {
            let _ = wx_rust_qidian::api::r#impl::execute_with_retry(
                svc.as_ref(),
                exec.as_ref(),
                "https://api.qidian.qq.com/cgi-bin/call/dial/getivrlist",
                String::new(),
            )
            .await;
        }));
    }
    for handle in handles {
        let _ = handle.await;
    }
    // 两次执行各重试 3 次：总执行 8 次
    assert_eq!(
        calls.load(Ordering::SeqCst),
        8,
        "两次执行各自 1 + maxRetryTimes 次"
    );
}

/// 构建开启重试的服务（maxRetryTimes=3, retrySleepMillis=1）。
fn build_busy_service() -> Arc<WxQidianServiceImpl> {
    let mut config =
        wx_rust_qidian::config::r#impl::WxQidianDefaultConfig::new("default", "secret");
    config.set_token("token123");
    let service = WxQidianServiceImpl::new_arc(Arc::new(config));
    service.set_max_retry_times(3);
    service.set_retry_sleep_millis(1);
    service
}
