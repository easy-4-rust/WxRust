//! 基础实现（对应 Java `me.chanjar.weixin.qidian.api.impl.BaseWxQidianServiceImpl`）。
//!
//! Java 的 `BaseWxQidianServiceImpl` 承载执行引擎（指数退避重试 + token
//! 自动单次刷新）。Rust 中 trait 无法携带泛型方法（破坏 dyn 兼容），故将
//! 执行引擎抽为本模块的泛型自由函数，由门面 trait 的 `get`/`post` 默认
//! 实现调用——同一语义、同一文件映射（与 mp/miniapp 模块同一约定）。

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::RequestExecutor;

use crate::api::WxQidianService;

/// 执行请求（对应 Java `BaseWxQidianServiceImpl.execute`）。
///
/// 策略：错误码 -1（系统繁忙）时指数退避重试（`retrySleepMillis << n`，
/// 最多 `maxRetryTimes` 次）；其余错误直接上抛。重试超限返回
/// `WxErrorException::Runtime`（对应 Java `WxRuntimeException`）。
pub async fn execute_with_retry<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: E,
) -> Result<T, WxErrorException>
where
    S: WxQidianService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    let max_retry_times = svc.max_retry_times();
    let retry_sleep_millis = svc.retry_sleep_millis();

    let mut retry_times = 0;
    loop {
        match execute_internal(svc, executor, uri, &data, false).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // -1 系统繁忙，指数退避后重试
                if e.error_code() == Some(-1) {
                    if retry_times + 1 > max_retry_times {
                        return Err(WxErrorException::Runtime(
                            wx_rust_common::error::WxRuntimeError::new(
                                "微信服务端异常，超出重试次数",
                            ),
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
    Err(WxErrorException::Runtime(
        wx_rust_common::error::WxRuntimeError::new("微信服务端异常，超出重试次数"),
    ))
}

/// 执行内部请求（对应 Java `executeInternal`）。
///
/// token 注入 + access_token 过期（40001/40014/42001）时强制过期并
/// 自动单次刷新（`do_not_auto_refresh` 防无限递归）。
pub async fn execute_internal<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: &E,
    do_not_auto_refresh: bool,
) -> Result<T, WxErrorException>
where
    S: WxQidianService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    if uri.contains("access_token=") {
        return Err(WxErrorException::from_code(
            -99,
            format!("uri参数中不允许有access_token: {uri}"),
        ));
    }

    let config = svc.config_storage();
    let access_token = svc.get_access_token().await?;
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
            .execute(&uri_with_access_token, data.clone(), WxType::Mp)
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
