//! 第三方应用基础实现（对应 Java
//! `me.chanjar.weixin.cp.tp.service.impl.BaseWxCpTpServiceImpl`）。
//!
//! Java 的 `BaseWxCpTpServiceImpl` 承载执行引擎（指数退避重试 +
//! suite_access_token 自动单次刷新 + `withoutSuiteAccessToken` 通道）。
//! Rust 中 trait 无法携带泛型方法（破坏 dyn 兼容），故将执行引擎抽为本
//! 模块的泛型自由函数，由 `WxCpTpService` trait 的 `get`/`post`/
//! `get_without_suite_token`/`post_without_suite_token` 默认实现调用——
//! 与 `api::r#impl::base_wx_cp_service_impl` 同一设计原则。
//!
//! 语义镜像 Java `execute`/`executeInternal`：
//! - 错误码 -1（系统繁忙）时指数退避重试
//!   （`retrySleepMillis << n`，最多 `maxRetryTimes` 次），重试超限抛
//!   `微信服务端异常，超出重试次数`（Java `WxRuntimeException` → Rust
//!   `WxErrorException::from_code(-99, ...)`，ADAPTED）；
//! - `executeInternal` 对 uri 含 `suite_access_token=` 抛
//!   `IllegalArgumentException`（Rust -99，ADAPTED）；
//! - 错误码 42009（suite_access_token 已过期）时强制过期并自动单次刷新
//!   后重试（`do_not_auto_refresh` 防无限递归，对应 Java 递归
//!   `execute(executor, uri, data)`）。

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::RequestExecutor;

use crate::tp::service::WxCpTpService;

/// 执行请求（对应 Java `BaseWxCpTpServiceImpl.execute`）。
pub async fn execute_with_retry_tp<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: E,
    without_suite_access_token: bool,
) -> Result<T, WxErrorException>
where
    S: WxCpTpService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    let max_retry_times = svc.max_retry_times();
    let retry_sleep_millis = svc.retry_sleep_millis();

    let mut retry_times = 0;
    loop {
        match execute_internal_tp(svc, executor, uri, &data, without_suite_access_token, false)
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
/// suite_access_token 注入 + 42009（suite_access_token 已过期）时强制
/// 过期并自动单次刷新后重试（`do_not_auto_refresh` 防无限递归）。
pub async fn execute_internal_tp<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: &E,
    without_suite_access_token: bool,
    do_not_auto_refresh: bool,
) -> Result<T, WxErrorException>
where
    S: WxCpTpService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    if uri.contains("suite_access_token=") {
        return Err(WxErrorException::from_code(
            -99,
            format!("uri参数中不允许有suite_access_token: {uri}"),
        ));
    }

    let config = svc.wx_cp_tp_config_storage();
    let mut do_not_auto_refresh = do_not_auto_refresh;
    loop {
        // 每次循环重算 uri（42009 刷新后携带新 token，对应 Java 递归
        // execute → executeInternal 重算 uriWithAccessToken）
        let uri_with_access_token = if !without_suite_access_token {
            let suite_access_token = svc.get_suite_access_token().await?;
            if uri.contains('?') {
                format!("{uri}&suite_access_token={suite_access_token}")
            } else {
                format!("{uri}?suite_access_token={suite_access_token}")
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
                // 42009 suite_access_token 已过期：强制过期 + 自动单次
                // 刷新后重试（对应 Java WxCpErrorMsgEnum.CODE_42009 分支）
                if code == Some(42009) {
                    config.expire_suite_access_token();
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
