//! 统一执行管线。
//!
//! RUST_OBLIGATION：管线单实现。Java 各平台 `BaseXxxServiceImpl`
//! 各持一份 `executeInternal`（token 注入 + errcode 校验 + token 失效
//! 单次重放）；WxRust 将其收敛为本模块的 [`execute_pipeline`] 单实现，
//! 各 crate 以 [`PipelineContext`] 注入差异点（transport、token、uri、
//! body），并以 `parse` 闭包承载成功应答的类型化提取。
//!
//! 语义对齐 `wx-rust-miniapp` 的 `execute_internal`（对应 Java
//! `BaseWxMaServiceImpl.executeInternal`）：
//!
//! 1. uri 含 `access_token=` 时前置报错（-99，文案与 miniapp 现实现一致）；
//! 2. 组装 URL 注入 token（已有 `?` 则 `&` 追加，否则 `?` 追加
//!    `access_token={token}`）；GET 通过
//!    `execute_pipeline_with_refresh_and_query` 在 Token 之后追加 queryParam；
//! 3. 经 [`HttpTransport`] 发送；传输错误（`error_code()` 为 `None`）
//!    直接上抛、不重放；HTTP status >= 300 在响应体解析前转为 Runtime
//!    response failure（对齐 Java HttpComponents ResponseHandler）；
//! 4. GET 空 Body 对齐 SimpleGet 的 Runtime failure；Text POST 空 Body
//!    对齐 SimplePost 的 `WxErrorException(-99, "无响应内容")`；Text POST
//!    响应以精确的 `<xml>` 前缀开头时原样返回，不进入 WxError JSON 解析；
//! 5. 其余非空 2xx 应答按 `WxError::from_json_with_type` 校验 errcode
//!    （与 `SimpleGet/SimplePostRequestExecutor::handle_response` 一致，
//!    `wx_type` 用于错误码翻译）；errcode 非 0 时保留完整 `WxError`
//!    （含原始报文 `json`）返回 `Err`；
//! 6. errcode ∈ [`crate::api::wx_consts::ACCESS_TOKEN_ERROR_CODES`]
//!    （40001/40014/42001）时先执行 `on_token_invalid`（各 crate 承接
//!    「加锁比对→置过期」，内部含 `.await` 故为 `BoxFuture`）——置过期
//!    恒执行；随后仅当 `replay_on_token_invalid`（对应 miniapp
//!    `auto_refresh_token()`）为真且未重放过时单次重放（`replayed`
//!    flag 防无限递归，对应 miniapp `do_not_auto_refresh`）；生产 Service
//!    通过 `execute_pipeline_with_refresh` 在失效后重新调用服务层
//!    `get_access_token()`，并以新 Token 重建 URL；兼容入口
//!    `execute_pipeline` 在未提供刷新回调时保留旧的同 Token 重放行为；
//! 7. errcode 为 0 时交 `parse` 闭包提取成功值（parse 自身的错误原样
//!    上抛，不吞不换）。
//!
//! # 熔断（可选，默认关闭）
//!
//! `ctx.breaker` 为 `Some` 时：发送前 `before(host)`（host 为从
//! `ctx.uri` 解析的 `scheme://host[:port]`；解析失败退回整段 uri 作键）
//! ——熔断开启则直接返回熔断错误、零 transport 调用；整个执行
//! （发送 + errcode 校验 + 重放 + parse）以最终结果调用
//! `after(host, ok)`（`Ok` → 复位、`Err`（含传输/parse 错误）→ 计
//! 失败）。`breaker: None`（默认）时行为与无熔断完全一致。
//!
//! # body → 请求方法映射规则
//!
//! - [`TransportBody::None`] → `GET`（无请求体）；
//! - [`TransportBody::Text`] → `POST`，体原样透传，并附加
//!   `Content-Type: application/json; charset=UTF-8`（对应 Java
//!   HttpComponents `SimplePostRequestExecutor` 的 UTF-8 `StringEntity`）；
//! - [`TransportBody::Bytes`] → `POST`，体原样透传，不推断文本 Content-Type。
//!
//! # 适用边界
//!
//! 本管线按「GET JSON / POST JSON 或精确 `<xml>` 响应 + errcode 校验」语义工作；二进制应答下载
//! （如二维码图片字节流）不适用，应走各自执行器或后续流式管线
//! （[`stream::execute_stream`]，Task 7 落地）。

pub mod stream;

use std::future::Future;
use std::pin::Pin;

use crate::enums::WxType;
use crate::error::{WxError, WxErrorError, WxErrorException, WxRuntimeError};

type RefreshAccessTokenFuture<'a> =
    Pin<Box<dyn Future<Output = Result<String, WxErrorException>> + Send + 'a>>;
type RefreshAccessToken<'a> = dyn Fn() -> RefreshAccessTokenFuture<'a> + Sync + 'a;
use crate::http::{
    HttpTransport, TransportBody, TransportMethod, TransportRequest, TransportResponse,
};

/// 管线侧最小熔断接口（避免与具体熔断实现耦合）。
///
/// [`crate::circuit::CircuitBreaker`] 实现本 trait；其他实现（如空操作、
/// 自定义窗口策略）亦可注入 [`PipelineContext::breaker`]。
#[async_trait::async_trait]
pub trait CircuitBreakerLike: Send + Sync {
    /// 请求发送前调用：熔断开启时应返回 `Err` 阻止发送。
    async fn before(&self, host: &str) -> Result<(), WxErrorException>;
    /// 请求结束后调用：`ok = true` 复位，`ok = false` 计失败。
    async fn after(&self, host: &str, ok: bool);
}

/// 从 uri 解析熔断键 `scheme://host[:port]`。
///
/// 解析失败（或无 host 段）时退回整段 uri 作键——保持熔断行为确定性，
/// 不影响请求本身。
fn build_access_token_url(uri: &str, access_token: &str) -> String {
    if uri.contains('?') {
        format!("{uri}&access_token={access_token}")
    } else {
        format!("{uri}?access_token={access_token}")
    }
}

fn build_access_token_url_with_query(
    uri: &str,
    access_token: &str,
    query_param: Option<&str>,
) -> String {
    let url = build_access_token_url(uri, access_token);
    match query_param.filter(|query| !query.is_empty()) {
        Some(query) if url.contains('?') => format!("{url}&{query}"),
        Some(query) => format!("{url}?{query}"),
        None => url,
    }
}

fn breaker_host_key(uri: &str) -> String {
    match url::Url::parse(uri) {
        Ok(parsed) => match (parsed.scheme(), parsed.host_str(), parsed.port()) {
            (scheme, Some(host), Some(port)) => format!("{scheme}://{host}:{port}"),
            (scheme, Some(host), None) => format!("{scheme}://{host}"),
            _ => uri.to_string(),
        },
        Err(_) => uri.to_string(),
    }
}

/// 统一执行管线上下文：注入一次执行所需的差异点。
pub struct PipelineContext<'a> {
    /// 传输实现（生产 [`crate::http::ReqwestTransport`]，测试
    /// [`crate::http::MockTransport`]）
    pub transport: &'a dyn HttpTransport,
    /// 首次请求使用的 access_token；生产 Service 重放前会通过刷新回调替换为新 Token。
    pub access_token: String,
    /// 请求 uri（不含 access_token；与 Java「uri 不允许带 access_token」
    /// 一致的前置校验）
    pub uri: String,
    /// 请求体（按模块文档的映射规则决定 GET/POST）
    pub body: TransportBody,
    /// token 失效时是否重放一次（对应 miniapp 配置
    /// `auto_refresh_token()`；为 false 时仅执行 `on_token_invalid`
    /// 置过期、不重放——对齐 miniapp `execute_internal` 的
    /// 「lock→compare→expire 恒执行、重放仅当 auto_refresh」语义）
    pub replay_on_token_invalid: bool,
    /// 可选 per-host 熔断器（默认 `None` 关闭，行为与现状完全一致；
    /// `Some` 时按模块文档「熔断」节接入）
    pub breaker: Option<&'a dyn CircuitBreakerLike>,
}

/// 执行并处理 token 失效重放。
///
/// 语义与 miniapp base impl 的 `execute_internal` 一致：首次执行；
/// errcode ∈ [`crate::api::wx_consts::ACCESS_TOKEN_ERROR_CODES`] 且
/// `on_token_invalid`（加锁比对→置过期，内部含 `.await`，故返回
/// `BoxFuture`）执行后，`replay_on_token_invalid` 为真时重放一次
/// （`replayed` flag 防无限递归）。
///
/// # 参数
/// - `ctx`：执行上下文（transport / token / uri / body / 重放开关 / 可选熔断器）
/// - `wx_type`：微信平台类型（errcode 错误信息翻译表选择）
/// - `parse`：errcode 为 0 时的成功应答提取函数
/// - `on_token_invalid`：token 失效回调（`None` 时既不置过期也不重放）
///
/// # 返回
/// `parse` 提取的成功值；或 errcode 非 0 的业务错误 / 传输错误。
/// 兼容入口：保留旧的同 Token 重放行为，供无刷新上下文的调用使用。
pub async fn execute_pipeline<T, F>(
    ctx: PipelineContext<'_>,
    wx_type: WxType,
    parse: F,
    on_token_invalid: Option<&(dyn Fn() -> futures_util::future::BoxFuture<'static, ()> + Sync)>,
) -> Result<T, WxErrorException>
where
    F: Fn(TransportResponse) -> Result<T, WxErrorException>,
{
    execute_pipeline_inner(ctx, wx_type, parse, on_token_invalid, None, None).await
}

/// 与 Java executeInternal 一致：Token 失效后重新获取 Token，再重放一次。
pub async fn execute_pipeline_with_refresh<'ctx, 'refresh, T, F>(
    ctx: PipelineContext<'ctx>,
    wx_type: WxType,
    parse: F,
    on_token_invalid: Option<&(dyn Fn() -> futures_util::future::BoxFuture<'static, ()> + Sync)>,
    refresh_access_token: Option<&RefreshAccessToken<'refresh>>,
) -> Result<T, WxErrorException>
where
    F: Fn(TransportResponse) -> Result<T, WxErrorException>,
{
    execute_pipeline_inner(
        ctx,
        wx_type,
        parse,
        on_token_invalid,
        refresh_access_token,
        None,
    )
    .await
}

/// 与 Java executeInternal + SimpleGetRequestExecutor 一致：先注入 Token，再追加 queryParam。
///
/// Java 的 BaseWxMaServiceImpl.executeInternal 先构造 uriWithAccessToken，
/// 随后 SimpleGetRequestExecutor.execute 再追加 queryParam。
pub async fn execute_pipeline_with_refresh_and_query<'ctx, 'refresh, T, F>(
    ctx: PipelineContext<'ctx>,
    wx_type: WxType,
    parse: F,
    on_token_invalid: Option<&(dyn Fn() -> futures_util::future::BoxFuture<'static, ()> + Sync)>,
    refresh_access_token: Option<&RefreshAccessToken<'refresh>>,
    query_param: Option<&str>,
) -> Result<T, WxErrorException>
where
    F: Fn(TransportResponse) -> Result<T, WxErrorException>,
{
    execute_pipeline_inner(
        ctx,
        wx_type,
        parse,
        on_token_invalid,
        refresh_access_token,
        query_param,
    )
    .await
}

async fn execute_pipeline_inner<'ctx, 'refresh, T, F>(
    ctx: PipelineContext<'ctx>,
    wx_type: WxType,
    parse: F,
    on_token_invalid: Option<&(dyn Fn() -> futures_util::future::BoxFuture<'static, ()> + Sync)>,
    refresh_access_token: Option<&RefreshAccessToken<'refresh>>,
    query_param: Option<&str>,
) -> Result<T, WxErrorException>
where
    F: Fn(TransportResponse) -> Result<T, WxErrorException>,
{
    // 注：回调 trait 对象需 `+ Sync`——`&dyn Fn` 跨 `.await` 持有，服务的
    // async_trait 门面要求整体 future 为 Send（现有实现闭包均为 Sync，
    // 纯编译期约束加强，不改变运行时语义）
    // 与 Java/miniapp 一致的前置校验：uri 不允许自带 access_token
    if ctx.uri.contains("access_token=") {
        return Err(WxErrorException::from_code(
            -99,
            format!("uri参数中不允许有access_token: {}", ctx.uri),
        ));
    }

    // 熔断（默认关闭）：before 拒绝则零 transport 调用直接返回；
    // host 键只解析一次，after 沿用同一键
    let breaker_key = ctx.breaker.is_some().then(|| breaker_host_key(&ctx.uri));
    if let (Some(breaker), Some(key)) = (ctx.breaker, breaker_key.as_deref()) {
        breaker.before(key).await?;
    }

    // 首次请求使用服务层 Token；刷新后重新构造 URL。
    let mut active_token = ctx.access_token.clone();
    let mut url = build_access_token_url_with_query(&ctx.uri, &active_token, query_param);

    // body → 方法映射（规则见模块文档）
    let method = match &ctx.body {
        TransportBody::None => TransportMethod::Get,
        TransportBody::Text(_) | TransportBody::Bytes(_) => TransportMethod::Post,
    };

    // Java HttpComponents SimplePostRequestExecutor uses
    // StringEntity(data, ContentType.APPLICATION_JSON.withCharset(UTF_8)).
    // Keep binary bodies free of an inferred text/JSON content type.
    let headers = match &ctx.body {
        TransportBody::Text(_) => vec![(
            "Content-Type".to_string(),
            "application/json; charset=UTF-8".to_string(),
        )],
        TransportBody::None | TransportBody::Bytes(_) => vec![],
    };

    // Java 以递归实现单次 token 自动刷新；Rust 以 flag 循环表达同一
    // 语义（replayed=true 后不再重放，防无限递归）
    let mut replayed = false;
    let outcome = loop {
        // 传输错误（error_code 为 None）直接上抛，不重放
        let resp = match ctx
            .transport
            .send(TransportRequest {
                method: method.clone(),
                url: url.clone(),
                headers: headers.clone(),
                body: ctx.body.clone(),
            })
            .await
        {
            Ok(resp) => resp,
            Err(e) => break Err(e),
        };

        // Apache AbstractHttpClientResponseHandler rejects HTTP status >= 300
        // before parsing the response entity; BaseWxMaServiceImpl wraps the
        // resulting IOException as WxRuntimeException. Match that boundary.
        if resp.status >= 300 {
            break Err(WxErrorException::Runtime(WxRuntimeError::new(format!(
                "HTTP response status: {}",
                resp.status
            ))));
        }
        // Java SimplePostRequestExecutor rejects an empty body as
        // WxErrorException(-99, "无响应内容"); SimpleGet has no such guard and
        // dereferences the null Gson result, yielding a runtime exception.
        let simple_post = matches!(&ctx.body, TransportBody::Text(_));
        if resp.body.is_empty() {
            if simple_post {
                break Err(WxErrorException::from_code(-99, "无响应内容"));
            }
            break Err(WxErrorException::Runtime(WxRuntimeError::new(
                "HTTP response body is empty",
            )));
        }

        let body_text = String::from_utf8_lossy(&resp.body);
        // Java SimplePostRequestExecutor returns an exact "<xml>" prefix as-is
        // before attempting WxError JSON parsing; GET does not have this branch.
        if simple_post && body_text.starts_with("<xml>") {
            break parse(resp);
        }
        // 与标准执行器 handle_response 一致的 errcode 校验
        let wx_error = WxError::from_json_with_type(&body_text, Some(wx_type));
        if wx_error.error_code != 0 {
            if crate::api::wx_consts::ACCESS_TOKEN_ERROR_CODES.contains(&wx_error.error_code)
                && let Some(on_token_invalid) = on_token_invalid
            {
                // 先按 Java 语义加锁比对并使旧 Token 过期。
                on_token_invalid().await;
                if ctx.replay_on_token_invalid && !replayed {
                    // Java 递归进入 executeInternal 后会再次调用 getAccessToken(false)。
                    if let Some(refresh) = refresh_access_token {
                        match refresh().await {
                            Ok(token) => active_token = token,
                            Err(error) => break Err(error),
                        }
                    }
                    url = build_access_token_url_with_query(&ctx.uri, &active_token, query_param);
                    replayed = true;
                    continue;
                }
            }
            // 保留完整 WxError（含原始报文 json）供上层回解析
            break Err(WxErrorException::Wx(WxErrorError::new(wx_error)));
        }
        break parse(resp);
    };

    // 熔断收口：以最终结果（含传输 / parse / 业务错误）计入 after——
    // 重放成功视为 ok（与单次成功请求等价）
    if let (Some(breaker), Some(key)) = (ctx.breaker, breaker_key.as_deref()) {
        breaker.after(key, outcome.is_ok()).await;
    }
    outcome
}
