//! 视频号小店基础实现（对应 Java `me.chanjar.weixin.channel.api.impl.BaseWxChannelServiceImpl`）。
//!
//! Java 的 `BaseWxChannelServiceImpl` 承载执行引擎（指数退避重试 + token 自动
//! 单次刷新 + 自定义 apiHost 替换）。Rust 中 trait 无法携带泛型方法（破坏 dyn
//! 兼容），故将执行引擎抽为本模块的泛型自由函数，由门面 trait 的 `get`/`post`/
//! `upload` 默认实现调用——同一语义、同一文件映射。
//!
//! 与 mp/miniapp 的差异（按 Java 语义）：
//! - Java `execute(RequestExecutor, String, E)` → 本模块 `execute`
//!   （重试开关同 Java `execute0`，printResult=true 语义以注释记录，Wave 0
//!   不落日志，`ADAPTED`）；
//! - Java `executeWithoutLog` → 本模块 `execute_without_log`；
//! - 自定义域名替换为 `apiHostUrl` 语义（Java `executeInternal` 中
//!   `uri.replace("https://api.weixin.qq.com", config.getApiHostUrl())`）。

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::RequestExecutor;

use crate::api::WxChannelService;
use crate::config::DEFAULT_API_HOST_URL;

/// 执行请求（对应 Java `BaseWxChannelServiceImpl.execute`）。
///
/// 策略：错误码 -1（系统繁忙）时指数退避重试（`retrySleepMillis << n`，
/// 最多 `maxRetryTimes` 次）；其余错误直接上抛。
pub async fn execute<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: E,
) -> Result<T, WxErrorException>
where
    S: WxChannelService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    execute0(svc, executor, uri, data).await
}

/// 执行请求（对应 Java `BaseWxChannelServiceImpl.executeWithoutLog`；
/// Java 仅差一个结果日志开关，Rust 侧 Wave 0 均不落日志，语义一致）。
pub async fn execute_without_log<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: E,
) -> Result<T, WxErrorException>
where
    S: WxChannelService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    execute0(svc, executor, uri, data).await
}

/// 执行请求（对应 Java `execute0(RequestExecutor, String, E, boolean printResult)`）。
///
/// 重试循环：`retryTimes + 1 > maxRetryTimes` 时上抛
/// “微信服务端异常，超出重试次数！”；错误码 -1（系统繁忙）指数退避后重试，
/// 其余错误直接上抛。
async fn execute0<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: E,
) -> Result<T, WxErrorException>
where
    S: WxChannelService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    let config = svc.wx_channel_config();
    let max_retry_times = config.max_retry_times();
    let retry_sleep_millis = config.retry_sleep_millis();

    let mut retry_times = 0;
    loop {
        match execute_internal(svc, executor, uri, &data, false).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // 最后一次重试失败后，直接抛出异常，不再等待
                if retry_times + 1 > max_retry_times {
                    return Err(WxErrorException::from_code(
                        e.error_code().unwrap_or(-99),
                        "微信服务端异常，超出重试次数！",
                    ));
                }
                // -1 系统繁忙，retrySleepMillis << retryTimes 后重试
                if e.error_code() == Some(-1) {
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
/// token 注入 + 自定义 apiHost 域名替换（`https://api.weixin.qq.com` →
/// 配置的 `apiHostUrl`）+ access_token 过期（40001/40014/42001）时强制过期并
/// 自动单次刷新（`do_not_auto_refresh` 防无限递归）。
pub async fn execute_internal<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: &E,
    do_not_auto_refresh: bool,
) -> Result<T, WxErrorException>
where
    S: WxChannelService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    if uri.contains("access_token=") {
        return Err(WxErrorException::from_code(
            -99,
            format!("uri参数中不允许有access_token: {uri}"),
        ));
    }

    let config = svc.wx_channel_config();
    let access_token = svc.get_access_token().await?;

    // Java: 配置了自定义 apiHostUrl 时替换默认 https://api.weixin.qq.com
    let uri = match config.api_host_url() {
        Some(api_host_url) if !api_host_url.is_empty() => {
            uri.replace(DEFAULT_API_HOST_URL, &api_host_url)
        }
        _ => uri.to_string(),
    };

    let uri_with_access_token = if uri.contains('?') {
        format!("{uri}&access_token={access_token}")
    } else {
        format!("{uri}?access_token={access_token}")
    };

    // Java 以递归实现单次 token 自动刷新；Rust 以 flag 循环表达
    // 同一语义（doNotAutoRefresh=true 后不再自动重试，防无限递归）
    let mut do_not_auto_refresh = do_not_auto_refresh;
    loop {
        match executor
            .execute(&uri_with_access_token, data.clone(), WxType::Channel)
            .await
        {
            Ok(result) => return Ok(result),
            Err(e) => {
                let code = e.error_code();
                if let Some(code) = code {
                    if wx_rust_common::api::wx_consts::ACCESS_TOKEN_ERROR_CODES.contains(&code) {
                        // 强制设置 access token 过期，下一次请求里就会刷新
                        let lock = config.access_token_lock();
                        let _guard = lock.lock().await;
                        if config.access_token().as_deref() == Some(access_token.as_str()) {
                            config.expire_access_token();
                        }
                        if config.auto_refresh_token() && !do_not_auto_refresh {
                            // 下一次不再自动重试（对齐 Java 注释语义）
                            do_not_auto_refresh = true;
                            continue;
                        }
                    }
                    if code != 0 {
                        return Err(e);
                    }
                    // Java 对错误码 0 的异常返回 null；标准执行器对 errcode!=0
                    // 已抛错，此处仅理论可达（执行器不抛错误码 0 的异常），
                    // 以 Err 表达该边缘路径（ADAPTED：无 null 类型）。
                    return Err(e);
                }
                return Err(e);
            }
        }
    }
}
