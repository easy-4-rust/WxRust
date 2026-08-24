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
//!
//! 执行路径（统一管线收敛，RUST_OBLIGATION：管线单实现）：
//! - **SimplePost 流量**（门面 `post` 及经其路由的 `post_json`/
//!   `post_to_json` 等调用）走
//!   [`wx_rust_common::pipeline::execute_pipeline`] 统一管线（见
//!   [`execute_post_via_pipeline`]）；
//! - **GET 门面与 multipart 上传（CommonUpload）及通用执行器流量**
//!   （经 [`execute`] / [`execute_without_log`] 进入）不走管线：GET 的
//!   Java 字节序「token 在前、query 在后」被测试
//!   `get_appends_token_and_query` 冻结，管线注入 token 恒在 URL 末尾、
//!   无法复现（按「宁可少接也不改语义」保留原执行器路径）；multipart
//!   则为非 JSON errcode 语义。均保留原路径 [`execute_internal`]。
//!
//! 两条重试路径共用 [`retry_loop`]（-1 系统繁忙指数退避重试，语义自原
//! `execute0` 原样抽出——channel 特有：重试超限错误保留原错误码并以
//! 「！」文案收束）；channel 侧差异点（`api_host_url` 域名替换、token
//! 失效「加锁比对→置过期」闭包、`auto_refresh_token` 重放开关）保留在
//! 本模块。

use std::sync::Arc;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::http::{ReqwestTransport, TransportBody};
use wx_rust_common::pipeline::{PipelineContext, execute_pipeline};
use wx_rust_common::util::http::RequestExecutor;

use crate::api::WxChannelService;
use crate::config::DEFAULT_API_HOST_URL;

/// -1 系统繁忙指数退避重试循环（自原 `execute0` 原样抽出，供执行器路径
/// 与统一管线路径共用——逐行语义不变）。
///
/// channel 特有语义（与其他 crate 不同，原样保留）：
/// - 重试超限时错误**保留原错误码**（`e.error_code().unwrap_or(-99)`），
///   文案带感叹号「微信服务端异常，超出重试次数！」；
/// - 循环末尾的兜臂错误为 -99「微信服务端异常，超出重试次数」（无感叹号，
///   对应 Java 原结构，实际由顶部检查先行拦截）。
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

/// 执行请求（对应 Java `BaseWxChannelServiceImpl.execute`）。
///
/// 策略：错误码 -1（系统繁忙）时指数退避重试（`retrySleepMillis << n`，
/// 最多 `maxRetryTimes` 次）；其余错误直接上抛。
///
/// 通用执行器入口（保留供 CommonUpload multipart 等任意 `RequestExecutor`
/// 使用）；SimpleGet/SimplePost 语义的 `get`/`post` 门面流量已改走
/// [`execute_get_via_pipeline`] / [`execute_post_via_pipeline`]。
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
    retry_loop(max_retry_times, retry_sleep_millis, || {
        execute_internal(svc, executor, uri, &data, false)
    })
    .await
}

/// POST 门面经统一管线的执行入口（供 `WxChannelService::post` 委托，对应
/// Java `post(url, postData)` + `SimplePostRequestExecutor`）。
///
/// 与原「`SimplePostRequestExecutor` + `execute`」路径等价：POST 文本体
/// 原样透传（无 Content-Type，与 `SimplePostRequestExecutor` 的
/// `.body(data)` 一致）；-1 重试语义共用 [`retry_loop`]；POST 无 query
/// 拼接，URL 字节序与原路径完全一致（`?access_token=..` 追加在末尾）。
///
/// 注：channel 的 **GET 门面不接管线**——测试 `get_appends_token_and_query`
/// 冻结了 Java 字节序「token 在前、query 在后」，管线注入 token 恒在
/// URL 末尾、无法复现该顺序（按「宁可少接也不改语义」保留原执行器路径）。
pub async fn execute_post_via_pipeline<S>(
    svc: &S,
    uri: &str,
    post_data: &str,
) -> Result<String, WxErrorException>
where
    S: WxChannelService + ?Sized,
{
    let config = svc.wx_channel_config();
    let max_retry_times = config.max_retry_times();
    let retry_sleep_millis = config.retry_sleep_millis();
    retry_loop(max_retry_times, retry_sleep_millis, || {
        execute_via_pipeline(svc, uri, TransportBody::Text(post_data.to_string()))
    })
    .await
}

/// 单次经统一管线执行（[`execute_internal`] 的管线化形态，仅承接
/// SimpleGet/SimplePost 语义的 JSON errcode 流量）。
///
/// channel 侧差异点保留于本封装：
/// - uri 前置校验时序与 [`execute_internal`] 一致（拒绝携带 token 的
///   uri 发生在取 token 之前——拒绝路径零副作用；管线内部会再校验一次）；
/// - `api_host_url` 自定义域名替换（`DEFAULT_API_HOST_URL` → 配置域名，
///   channel 特有语义，Some 且非空时替换）；
/// - `on_token_invalid` 原样承接「加锁比对→置过期」块（恒执行；是否
///   重放由 `replay_on_token_invalid` = `auto_refresh_token()` 决定，
///   对齐 [`execute_internal`] 的「lock→compare→expire 恒执行、重放仅当
///   auto_refresh_token」语义——channel 与 miniapp 同构，重放沿用同一
///   token 与同一 URL）。
async fn execute_via_pipeline<S>(
    svc: &S,
    uri: &str,
    body: TransportBody,
) -> Result<String, WxErrorException>
where
    S: WxChannelService + ?Sized,
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

    // 复用服务层 reqwest::Client（Client 内部 Arc 句柄，克隆廉价——与原
    // get/post 每次调用构造 SimpleGet/SimplePost 执行器同级的开销方式）
    let transport = ReqwestTransport::new(svc.http_client().clone());

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
            uri,
            body,
            replay_on_token_invalid: config.auto_refresh_token(),
        },
        WxType::Channel,
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
/// 保留供非管线流量（CommonUpload multipart 等，经 [`execute`] /
/// [`execute_without_log`] 进入）；SimpleGet/SimplePost 语义流量已收敛至
/// [`execute_via_pipeline`]（同一 token 注入 + 域名替换 + errcode +
/// 单次重放语义的统一管线实现）。
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
