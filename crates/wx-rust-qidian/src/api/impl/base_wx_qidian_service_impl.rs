//! 基础实现（对应 Java `me.chanjar.weixin.qidian.api.impl.BaseWxQidianServiceImpl`）。
//!
//! Java 的 `BaseWxQidianServiceImpl` 承载执行引擎（指数退避重试 + token
//! 自动单次刷新）。Rust 中 trait 无法携带泛型方法（破坏 dyn 兼容），故将
//! 执行引擎抽为本模块的泛型自由函数，由门面 trait 的 `get`/`post` 默认
//! 实现调用——同一语义、同一文件映射（与 mp/miniapp 模块同一约定）。
//!
//! 执行路径（统一管线收敛，RUST_OBLIGATION：管线单实现）：
//! - **SimpleGet/SimplePost 流量**（门面 `get_by_url`/`post_by_url` 及经
//!   其路由的调用）走 [`wx_rust_common::pipeline::execute_pipeline`] 统一
//!   管线（见 [`execute_get_via_pipeline`] / [`execute_post_via_pipeline`]）；
//! - 通用执行器路径 [`execute_with_retry`] / [`execute_internal`] 保留
//!   （qidian 侧 host config 差异由 URL 构造层
//!   `WxQidianApiUrl::get_url(config)` 承担，非执行期替换）。
//!
//! 两条路径共用 [`retry_loop`]（-1 系统繁忙指数退避重试，语义自原
//! `execute_with_retry` 原样抽出；重试超限错误保持 qidian 特有的
//! `WxErrorException::Runtime` 变体——与 mp/cp/miniapp 的 -99 收束不同，
//! 对应 Java `WxRuntimeException` 直抛）。

use std::sync::Arc;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::http::{ReqwestTransport, TransportBody};
use wx_rust_common::pipeline::{PipelineContext, execute_pipeline};
use wx_rust_common::util::http::RequestExecutor;

use crate::api::WxQidianService;

/// -1 系统繁忙指数退避重试循环（自原 `execute_with_retry` 原样抽出，
/// 供执行器路径与统一管线路径共用——逐行语义不变）。
///
/// 策略：错误码 -1（系统繁忙）时指数退避重试（`retry_sleep_millis << n`，
/// 最多 `max_retry_times` 次）；其余错误直接上抛；超次数后以
/// `WxErrorException::Runtime`（对应 Java `WxRuntimeException`）收束。
async fn retry_loop<T, F, Fut>(
    max_retry_times: i32,
    retry_sleep_millis: i32,
    mut f: F,
) -> Result<T, WxErrorException>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, WxErrorException>>,
{
    let mut retry_times = 0;
    loop {
        match f().await {
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

/// 执行请求（对应 Java `BaseWxQidianServiceImpl.execute`）。
///
/// 策略：错误码 -1（系统繁忙）时指数退避重试（`retrySleepMillis << n`，
/// 最多 `maxRetryTimes` 次）；其余错误直接上抛。重试超限返回
/// `WxErrorException::Runtime`（对应 Java `WxRuntimeException`）。
///
/// 通用执行器入口（保留供任意 `RequestExecutor` 使用）；
/// SimpleGet/SimplePost 语义的 `get_by_url`/`post_by_url` 门面流量已改走
/// [`execute_get_via_pipeline`] / [`execute_post_via_pipeline`]。
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
    retry_loop(max_retry_times, retry_sleep_millis, || {
        execute_internal(svc, executor, uri, &data, false)
    })
    .await
}

/// GET 门面经统一管线的执行入口（供 `WxQidianService::get_by_url` 委托，
/// 对应 Java `get(String, String)` + `SimpleGetRequestExecutor`）。
///
/// 与原「`SimpleGetRequestExecutor` + `execute_with_retry`」路径等价：
/// - 原执行器的 query 拼接语义内联于此（data 非空时按 uri 是否已含
///   `?` 以 `&`/`?` 追加；data 为空不追加）；
/// - -1 系统繁忙指数退避重试语义共用 [`retry_loop`]（重试超限保持
///   qidian 特有的 `Runtime` 变体错误）。
///
/// 已知字节级差异（参数集合不变、HTTP 语义等价）：query 与 access_token
/// 的追加顺序由原「token 在前、query 在后」变为「query 在前、token 由
/// 管线追加在后」（管线在组装 URL 时注入 token，无法在其后再追加参数）。
pub async fn execute_get_via_pipeline<S>(
    svc: &S,
    uri: &str,
    query_param: &str,
) -> Result<String, WxErrorException>
where
    S: WxQidianService + ?Sized,
{
    let max_retry_times = svc.max_retry_times();
    let retry_sleep_millis = svc.retry_sleep_millis();
    // SimpleGetRequestExecutor::execute 的 query 拼接（原样内联）
    let uri_with_query = if query_param.is_empty() {
        uri.to_string()
    } else if uri.contains('?') {
        format!("{uri}&{query_param}")
    } else {
        format!("{uri}?{query_param}")
    };
    retry_loop(max_retry_times, retry_sleep_millis, || {
        execute_via_pipeline(svc, &uri_with_query, TransportBody::None)
    })
    .await
}

/// POST 门面经统一管线的执行入口（供 `WxQidianService::post_by_url`
/// 委托，对应 Java `post(String, String)` + `SimplePostRequestExecutor`）。
///
/// 与原「`SimplePostRequestExecutor` + `execute_with_retry`」路径等价：
/// POST 文本体原样透传（无 Content-Type，与 `SimplePostRequestExecutor`
/// 的 `.body(data)` 一致）；-1 重试语义共用 [`retry_loop`]。
pub async fn execute_post_via_pipeline<S>(
    svc: &S,
    uri: &str,
    post_data: &str,
) -> Result<String, WxErrorException>
where
    S: WxQidianService + ?Sized,
{
    let max_retry_times = svc.max_retry_times();
    let retry_sleep_millis = svc.retry_sleep_millis();
    retry_loop(max_retry_times, retry_sleep_millis, || {
        execute_via_pipeline(svc, uri, TransportBody::Text(post_data.to_string()))
    })
    .await
}

/// 单次经统一管线执行（[`execute_internal`] 的管线化形态，仅承接
/// SimpleGet/SimplePost 语义的 JSON errcode 流量）。
///
/// qidian 侧差异点保留于本封装：
/// - uri 前置校验时序与 [`execute_internal`] 一致（拒绝携带 token 的
///   uri 发生在取 token 之前——拒绝路径零副作用；管线内部会再校验一次）；
/// - host config 差异由 URL 构造层（`WxQidianApiUrl::get_url(config)`）
///   承担，执行期无域名替换；
/// - errcode 翻译沿用 qidian 现实现的 `WxType::Mp`（企点与公众号共用
///   错误码表，与原 `execute_internal` 一致）；
/// - `on_token_invalid` 原样承接「加锁比对→置过期」块（恒执行；是否
///   重放由 `replay_on_token_invalid` = `auto_refresh_token()` 决定，
///   对齐 [`execute_internal`] 的「lock→compare→expire 恒执行、重放仅当
///   auto_refresh_token」语义）。
async fn execute_via_pipeline<S>(
    svc: &S,
    uri: &str,
    body: TransportBody,
) -> Result<String, WxErrorException>
where
    S: WxQidianService + ?Sized,
{
    if uri.contains("access_token=") {
        return Err(WxErrorException::from_code(
            -99,
            format!("uri参数中不允许有access_token: {uri}"),
        ));
    }

    let config = svc.config_storage();
    let access_token = svc.get_access_token().await?;

    // 复用服务层 reqwest::Client（Client 内部 Arc 句柄，克隆廉价——与原
    // get_by_url/post_by_url 每次调用构造 SimpleGet/SimplePost 执行器
    // 同级的开销方式；qidian 的 http_client() 即按值返回克隆句柄）
    let transport = ReqwestTransport::new(svc.http_client());

    // token 失效回调：原样承接 execute_internal 的「加锁比对→置过期」块
    //（返回类型即管线签名中的 `futures_util::future::BoxFuture<'static, ()>`
    // ——本 crate 不直接依赖 futures-util，以 std 类型结构性等价表达）
    let token_snapshot = access_token.clone();
    let config_for_invalidation = Arc::clone(&config);
    let on_token_invalid =
        move || -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
            let config = Arc::clone(&config_for_invalidation);
            let token = token_snapshot.clone();
            Box::pin(async move {
                // 强制设置 access token 过期，下一次请求里就会刷新
                let lock = config.access_token_lock();
                let _guard = lock.lock().await;
                if config.access_token().as_deref() == Some(token.as_str()) {
                    config.expire_access_token();
                }
            })
        };

    execute_pipeline(
        PipelineContext {
            transport: &transport,
            access_token,
            uri: uri.to_string(),
            body,
            replay_on_token_invalid: config.auto_refresh_token(),
        },
        // 与原 execute_internal 相同：qidian 的 errcode 翻译走 Mp 表
        WxType::Mp,
        // errcode 校验后的原始应答文本（与 SimpleGet/SimplePost 的
        // Ok(response_content) 对应；reqwest text() 对 UTF-8 应答与
        // from_utf8_lossy 逐字节一致）
        |resp| Ok(String::from_utf8_lossy(&resp.body).into_owned()),
        Some(&on_token_invalid),
    )
    .await
}

/// 执行内部请求（对应 Java `executeInternal`；执行器路径的内部实现）。
///
/// 保留供任意 `RequestExecutor` 流量（经 [`execute_with_retry`] 进入）；
/// SimpleGet/SimplePost 语义流量已收敛至 [`execute_via_pipeline`]
/// （同一 token 注入 + errcode + 单次重放语义的统一管线实现）。
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
