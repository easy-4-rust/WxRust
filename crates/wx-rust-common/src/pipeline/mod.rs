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
//!    `access_token={token}`）；
//! 3. 经 [`HttpTransport`] 发送；传输错误（`error_code()` 为 `None`）
//!    直接上抛、不重放；
//! 4. 应答按 `WxError::from_json_with_type` 校验 errcode（与
//!    `SimpleGet/SimplePostRequestExecutor::handle_response` 一致，
//!    `wx_type` 用于错误码翻译）；errcode 非 0 时保留完整 `WxError`
//!    （含原始报文 `json`）返回 `Err`；
//! 5. errcode ∈ [`crate::api::wx_consts::ACCESS_TOKEN_ERROR_CODES`]
//!    （40001/40014/42001）时先执行 `on_token_invalid`（各 crate 承接
//!    「加锁比对→置过期」，内部含 `.await` 故为 `BoxFuture`）——置过期
//!    恒执行；随后仅当 `replay_on_token_invalid`（对应 miniapp
//!    `auto_refresh_token()`）为真且未重放过时单次重放（`replayed`
//!    flag 防无限递归，对应 miniapp `do_not_auto_refresh`）；重放沿用
//!    同一 URL 与同一 token——token 刷新由服务层在下一次独立请求时发生
//!    （与 miniapp 现实现一致：重放循环使用同一 `uri_with_access_token`）；
//! 6. errcode 为 0 时交 `parse` 闭包提取成功值（parse 自身的错误原样
//!    上抛，不吞不换）。
//!
//! # body → 请求方法映射规则
//!
//! - [`TransportBody::None`] → `GET`（无请求体）；
//! - [`TransportBody::Text`] / [`TransportBody::Bytes`] → `POST`，
//!   体原样透传（对应 `SimplePostRequestExecutor` 的 POST 文本体语义）。
//!
//! # 适用边界
//!
//! 本管线按「JSON 报文 + errcode 校验」语义工作；二进制应答下载
//! （如二维码图片字节流）不适用，应走各自执行器或后续流式管线。

use crate::enums::WxType;
use crate::error::{WxError, WxErrorError, WxErrorException};
use crate::http::{
    HttpTransport, TransportBody, TransportMethod, TransportRequest, TransportResponse,
};

/// 统一执行管线上下文：注入一次执行所需的差异点。
pub struct PipelineContext<'a> {
    /// 传输实现（生产 [`crate::http::ReqwestTransport`]，测试
    /// [`crate::http::MockTransport`]）
    pub transport: &'a dyn HttpTransport,
    /// 当前有效 access_token（由服务层取得；重放沿用同一 token）
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
/// - `ctx`：执行上下文（transport / token / uri / body / 重放开关）
/// - `wx_type`：微信平台类型（errcode 错误信息翻译表选择）
/// - `parse`：errcode 为 0 时的成功应答提取函数
/// - `on_token_invalid`：token 失效回调（`None` 时既不置过期也不重放）
///
/// # 返回
/// `parse` 提取的成功值；或 errcode 非 0 的业务错误 / 传输错误。
pub async fn execute_pipeline<T, F>(
    ctx: PipelineContext<'_>,
    wx_type: WxType,
    parse: F,
    on_token_invalid: Option<&(dyn Fn() -> futures_util::future::BoxFuture<'static, ()> + Sync)>,
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

    // token 注入（? 已存在则 & 追加）——循环外组装一次，重放沿用同一 URL
    let url = if ctx.uri.contains('?') {
        format!("{}&access_token={}", ctx.uri, ctx.access_token)
    } else {
        format!("{}?access_token={}", ctx.uri, ctx.access_token)
    };

    // body → 方法映射（规则见模块文档）
    let method = match &ctx.body {
        TransportBody::None => TransportMethod::Get,
        TransportBody::Text(_) | TransportBody::Bytes(_) => TransportMethod::Post,
    };

    // Java 以递归实现单次 token 自动刷新；Rust 以 flag 循环表达同一
    // 语义（replayed=true 后不再重放，防无限递归）
    let mut replayed = false;
    loop {
        // 传输错误（error_code 为 None）直接上抛，不重放
        let resp = ctx
            .transport
            .send(TransportRequest {
                method: method.clone(),
                url: url.clone(),
                headers: vec![],
                body: ctx.body.clone(),
            })
            .await?;

        // 与标准执行器 handle_response 一致的 errcode 校验
        let body_text = String::from_utf8_lossy(&resp.body);
        let wx_error = WxError::from_json_with_type(&body_text, Some(wx_type));
        if wx_error.error_code != 0 {
            if crate::api::wx_consts::ACCESS_TOKEN_ERROR_CODES.contains(&wx_error.error_code) {
                if let Some(on_token_invalid) = on_token_invalid {
                    // 强制设置 access token 过期，下一次请求里就会刷新
                    // （重放沿用同一 token，与 miniapp 现实现一致）。
                    // 置过期恒执行；重放仅在 replay_on_token_invalid 为真
                    // 且未重放过时发生（对齐 miniapp「auto_refresh_token()
                    // && !do_not_auto_refresh」语义）
                    on_token_invalid().await;
                    if ctx.replay_on_token_invalid && !replayed {
                        replayed = true;
                        continue;
                    }
                }
            }
            // 保留完整 WxError（含原始报文 json）供上层回解析
            return Err(WxErrorException::Wx(WxErrorError::new(wx_error)));
        }
        return parse(resp);
    }
}
