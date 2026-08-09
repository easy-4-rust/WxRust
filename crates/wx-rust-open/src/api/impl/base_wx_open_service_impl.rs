//! 开放平台（第三方平台）基础实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenServiceAbstractImpl` +
//! `WxOpenComponentServiceImpl` 的 token 引擎。
//!
//! Java 的 `WxOpenServiceAbstractImpl` 承载裸执行（`execute` 直调执行器），
//! `WxOpenComponentServiceImpl` 承载 component_access_token 双检锁刷新链
//! （`getComponentAccessToken`）与带 token 注入的 `get`/`post`。Rust 中
//! trait 无法携带泛型方法（破坏 dyn 兼容），故将执行引擎抽为本模块的
//! 泛型自由函数，由门面 trait 与组件服务实现的默认实现调用——同一语义、
//! 同一文件映射。
//!
//! 刷新链（对应 Java `getComponentAccessToken`）：
//! `component_verify_ticket`（推送）→ POST `/cgi-bin/component/api_component_token`
//! （component_appid + component_appsecret + component_verify_ticket）→
//! `WxOpenComponentAccessToken` → 缓存（预留 200 秒提前过期）。

use wx_rust_common::api::wx_consts::ACCESS_TOKEN_ERROR_CODES;
use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, WxErrorException};
use wx_rust_common::util::http::{RequestExecutor, SimplePostRequestExecutor};

use crate::api::WxOpenService;
use crate::bean::WxOpenComponentAccessToken;
use crate::enums::url_core::api_component_token_url;

/// 构建带 token 注入的 uri（对应 Java `uri + (uri.contains("?") ? "&" : "?")
/// + accessTokenKey + "=" + componentAccessToken`）。
fn build_uri_with_token(uri: &str, access_token_key: &str, token: &str) -> String {
    if uri.contains('?') {
        format!("{uri}&{access_token_key}={token}")
    } else {
        format!("{uri}?{access_token_key}={token}")
    }
}

/// 执行请求（对应 Java `BaseWxMpServiceImpl.executeWithRetry` 语义 +
/// `WxOpenServiceAbstractImpl.execute`）。
///
/// 策略：错误码 -1（系统繁忙）时指数退避重试（`retrySleepMillis << n`，
/// 最多 `maxRetryTimes` 次）；其余错误直接上抛。
pub async fn execute_with_retry<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: E,
    access_token_key: &str,
) -> Result<T, WxErrorException>
where
    S: WxOpenService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    let config = svc.wx_open_config_storage();
    let max_retry_times = config.max_retry_times();
    let retry_sleep_millis = config.retry_sleep_millis();

    let mut retry_times = 0;
    loop {
        match execute_internal(svc, executor, uri, &data, access_token_key, false).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // -1 系统繁忙，指数退避后重试
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

/// 执行内部请求（对应 Java `WxOpenComponentServiceImpl.get/post` 的
/// token 注入 + 自动刷新语义，与 mp `executeInternal` 同一模式）。
///
/// component_access_token 注入 + 过期（40001/40014/42001）时强制过期并
/// 自动单次刷新（`do_not_auto_refresh` 防无限递归）。
pub async fn execute_internal<S, T, E>(
    svc: &S,
    executor: &dyn RequestExecutor<T, E>,
    uri: &str,
    data: &E,
    access_token_key: &str,
    do_not_auto_refresh: bool,
) -> Result<T, WxErrorException>
where
    S: WxOpenService + ?Sized,
    T: Send,
    E: Send + Clone,
{
    if uri.contains(&format!("{access_token_key}=")) {
        return Err(WxErrorException::from_code(
            -99,
            format!("uri参数中不允许有{access_token_key}: {uri}"),
        ));
    }

    let config = svc.wx_open_config_storage();
    let component_access_token = svc.get_component_access_token(false).await?;
    let mut component_access_token = component_access_token;
    let mut uri_with_token = build_uri_with_token(uri, access_token_key, &component_access_token);

    // Java 以递归实现单次 token 自动刷新（递归内重新 getComponentAccessToken
    // 并重建 uri）；Rust 以 flag + 循环表达同一语义（doNotAutoRefresh=true
    // 后不再自动重试，防无限递归）
    let mut do_not_auto_refresh = do_not_auto_refresh;
    loop {
        match executor
            .execute(&uri_with_token, data.clone(), WxType::Open)
            .await
        {
            Ok(result) => return Ok(result),
            Err(e) => {
                if let Some(code) = e.error_code() {
                    if ACCESS_TOKEN_ERROR_CODES.contains(&code) {
                        // 强制设置 component access token 过期，下一次请求里就会刷新
                        // （镜像 Java：expire 持锁、刷新前释放锁——Java post()
                        // 中 lock.lock() → expire → unlock() 后递归重试，防自锁）
                        {
                            let lock = config.component_access_token_lock();
                            let _guard = lock.lock().await;
                            if config.component_access_token().as_deref()
                                == Some(component_access_token.as_str())
                            {
                                config.expire_component_access_token();
                            }
                        }
                        if config.auto_refresh_token() && !do_not_auto_refresh {
                            // 对齐 Java 递归语义：重新获取 token 并重建 uri 后重试
                            do_not_auto_refresh = true;
                            component_access_token = svc.get_component_access_token(false).await?;
                            uri_with_token = build_uri_with_token(
                                uri,
                                access_token_key,
                                &component_access_token,
                            );
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

/// 获取 component_access_token（对应 Java
/// `WxOpenComponentServiceImpl.getComponentAccessToken(boolean)`）。
///
/// 双检锁：未过期且非强制刷新时直接返回；否则持锁后二次检查，以
/// component_appid/component_appsecret/component_verify_ticket POST
/// `api_component_token` 刷新并缓存。与 mp/ma 的 access_token 双检锁
/// 同一模式（tryLock(100ms) 轮询 + 3 秒超时）。
pub async fn get_component_access_token_with_lock<S>(
    svc: &S,
    force_refresh: bool,
) -> Result<String, WxErrorException>
where
    S: WxOpenService + ?Sized,
{
    let config = svc.wx_open_config_storage();
    if !force_refresh && !config.is_component_access_token_expired() {
        return config
            .component_access_token()
            .ok_or_else(|| WxErrorException::from_code(-99, "component access token 为空"));
    }

    let lock = config.component_access_token_lock();
    let timeout_at = std::time::Instant::now() + std::time::Duration::from_millis(3000);
    // 对应 mp tryLock(100ms) 轮询：guard 必须持有到刷新完成（双检锁）
    let _guard = loop {
        if !force_refresh && !config.is_component_access_token_expired() {
            return config
                .component_access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "component access token 为空"));
        }
        match lock.try_lock() {
            Ok(guard) => break guard,
            Err(_) => {
                if std::time::Instant::now() > timeout_at {
                    return Err(WxErrorException::from_code(
                        -99,
                        "获取componentAccessToken超时：获取时间超时",
                    ));
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    };

    // Java `jsonObject.addProperty("component_appid", ...)` 等三个字段
    let body = serde_json::json!({
        "component_appid": config.component_app_id().unwrap_or_default(),
        "component_appsecret": config.component_app_secret().unwrap_or_default(),
        "component_verify_ticket": config.component_verify_ticket().unwrap_or_default(),
    });
    // 刷新请求必须为裸请求（Java 直调 `getWxOpenService().post(...)`），
    // 不能经过带注入的执行引擎（否则递归取 token）
    let executor = SimplePostRequestExecutor::new(svc.http_client().clone());
    let uri = api_component_token_url(config.as_ref());
    let response = executor
        .execute(&uri, body.to_string(), WxType::Open)
        .await?;

    let component_access_token = extract_component_access_token(&response)?;
    config.update_component_access_token(&component_access_token);
    Ok(config
        .component_access_token()
        .unwrap_or_else(|| component_access_token.component_access_token().to_string()))
}

/// 解析 component_access_token 响应（对应 Java
/// `WxOpenComponentAccessToken.fromJson` + `parseErrorResponse` 语义）。
///
/// errcode != 0 时抛业务错误；成功时解析 `component_access_token`/`expires_in`。
pub fn extract_component_access_token(
    result_content: &str,
) -> Result<WxOpenComponentAccessToken, WxErrorException> {
    let error = WxError::from_json_with_type(result_content, Some(WxType::Open));
    if error.error_code != 0 {
        return Err(WxErrorException::from_code(
            error.error_code,
            error.error_msg.unwrap_or_default(),
        ));
    }
    WxOpenComponentAccessToken::from_json(result_content)
        .map_err(|e| WxErrorException::Serde(e.to_string()))
}

/// 归一化 errcode 为字符串（对应 Java Gson 宽松类型转换语义）。
///
/// 微信接口 `errcode` 为数字（如 `0`），Java 各 bean 的 `String errcode`
/// 经 Gson 自动转字符串（`"0"`）；Rust serde 严格，数字无法反序列化
/// 进 String 字段（生成的 bean 与 Java 字段类型一致，冻结），故在
/// bean 解析前将数字 errcode 归一化为字符串。与组件实现
/// （`WxOpenComponentServiceImpl::normalize_errcode`）同一语义，供
/// Ma*/Minishop 子域服务解析共用（ADAPTED）。
pub fn normalize_errcode(json: &str) -> Result<String, WxErrorException> {
    let mut value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| WxErrorException::Serde(e.to_string()))?;
    if let Some(errcode) = value.get("errcode") {
        if errcode.is_number() {
            value["errcode"] = serde_json::Value::String(errcode.to_string());
        }
    }
    serde_json::to_string(&value).map_err(|e| WxErrorException::Serde(e.to_string()))
}
