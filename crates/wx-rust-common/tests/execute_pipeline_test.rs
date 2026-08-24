//! `execute_pipeline` 统一执行管线测试。
//!
//! RUST_OBLIGATION：管线单实现——token 注入（URL 追加 `access_token`）、
//! errcode 校验（`WxError::from_json_with_type`）、token 失效
//! （40001/40014/42001）单次重放语义与 miniapp `execute_internal`
//! （对应 Java `BaseWxMaServiceImpl.executeInternal`）一致；
//! 以 MockTransport 零网络验证。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use futures_util::FutureExt;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::http::{MockTransport, TransportBody, TransportMethod, TransportResponse};
use wx_rust_common::pipeline::{PipelineContext, execute_pipeline};

/// 构造「置位」回调：返回设置指定 `AtomicBool` 的 `BoxFuture`。
///
/// （`BoxFuture<'static>` 需拥有数据，故经 `Arc` clone 进 async 块——
/// 对计划测试片段中栈上 `AtomicBool` 的最小可编译适配。）
fn flag_setter(
    flag: &Arc<AtomicBool>,
) -> impl Fn() -> futures_util::future::BoxFuture<'static, ()> {
    let flag = Arc::clone(flag);
    move || {
        let flag = Arc::clone(&flag);
        async move {
            flag.store(true, Ordering::SeqCst);
        }
        .boxed()
    }
}

/// 正常返回：errcode 0 → parse 提取字段成功；URL 注入 token、
/// `TransportBody::None` → GET 映射；transport 恰调用 1 次、不重放。
#[tokio::test]
async fn ok_response_parses_without_replay() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |req| {
        c.fetch_add(1, Ordering::SeqCst);
        // token 注入 + `?` 追加 + None→GET 映射
        assert_eq!(req.url, "https://mock.local/get?access_token=T1");
        assert_eq!(req.method, TransportMethod::Get);
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":0,"errmsg":"ok","data":42}"#.to_vec(),
        })
    });
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/get".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: None,
    };
    let r: i32 = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |resp| {
            let v: serde_json::Value = serde_json::from_slice(&resp.body)?;
            Ok(v["data"].as_i64().unwrap_or(0) as i32)
        },
        None,
    )
    .await
    .unwrap();
    assert_eq!(r, 42);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

/// 计划 verbatim：首次 40001 → on_token_invalid 触发（置过期）→
/// 单次重放成功；transport 恰调用 2 次。
#[tokio::test]
async fn token_invalid_replays_exactly_once() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |_| {
        let n = c.fetch_add(1, Ordering::SeqCst);
        let body = if n == 0 {
            r#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok","data":42}"#.to_string()
        };
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: body.into_bytes(),
        })
    });
    let expired = Arc::new(AtomicBool::new(false));
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/get".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: None,
    };
    let r: i32 = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |resp| {
            let v: serde_json::Value = serde_json::from_slice(&resp.body)?;
            Ok(v["data"].as_i64().unwrap_or(0) as i32)
        },
        Some(&flag_setter(&expired)),
    )
    .await
    .unwrap();
    assert_eq!(r, 42);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert!(expired.load(Ordering::SeqCst));
}

/// uri 含 access_token → 发送前立即报错（文案含「不允许」，错误码 -99），
/// transport 零调用。
#[tokio::test]
async fn uri_with_access_token_rejected_before_send() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":0}"#.to_vec(),
        })
    });
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/get?access_token=LEAK".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: None,
    };
    let res: Result<i32, WxErrorException> =
        execute_pipeline(ctx, WxType::MiniApp, |_| Ok(0i32), None).await;
    let err = res.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("不允许"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

/// 非 token 错误码（45009）→ 直接返回错误，不重放、on_token_invalid 不触发。
#[tokio::test]
async fn non_token_error_code_returns_without_replay() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":45009,"errmsg":"api daily quota exceeded"}"#.to_vec(),
        })
    });
    let invoked = Arc::new(AtomicBool::new(false));
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/get".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: None,
    };
    let res: Result<i32, WxErrorException> = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |_| Ok(0i32),
        Some(&flag_setter(&invoked)),
    )
    .await;
    let err = res.unwrap_err();
    assert_eq!(err.error_code(), Some(45009));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(!invoked.load(Ordering::SeqCst));
}

/// 重放仍失败（恒 40001）→ 返回该错误且仅执行两次（`replayed` flag
/// 防无限重放，对应 miniapp `do_not_auto_refresh` 语义）。
#[tokio::test]
async fn token_invalid_replay_still_fails_returns_error() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_vec(),
        })
    });
    let expired = Arc::new(AtomicBool::new(false));
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/get".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: None,
    };
    let res: Result<i32, WxErrorException> = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |_| Ok(0i32),
        Some(&flag_setter(&expired)),
    )
    .await;
    let err = res.unwrap_err();
    assert_eq!(err.error_code(), Some(40001));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert!(expired.load(Ordering::SeqCst));
}

/// `replay_on_token_invalid = false`（对应 miniapp `auto_refresh_token()`
/// 为 false）：errcode 命中 token 失效码时仍执行 on_token_invalid（置过期），
/// 但不重放——transport 恰调用 1 次，返回原 errcode 错误。
#[tokio::test]
async fn replay_disabled_still_expires_token_without_replay() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_vec(),
        })
    });
    let expired = Arc::new(AtomicBool::new(false));
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/get".into(),
        body: TransportBody::None,
        replay_on_token_invalid: false,
        breaker: None,
    };
    let res: Result<i32, WxErrorException> = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |_| Ok(0i32),
        Some(&flag_setter(&expired)),
    )
    .await;
    let err = res.unwrap_err();
    assert_eq!(err.error_code(), Some(40001));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(expired.load(Ordering::SeqCst));
}

/// body 映射规则：`TransportBody::Text` → 原始 POST（体原样透传），
/// 对应 SimplePostRequestExecutor 的 POST 文本体语义。
#[tokio::test]
async fn text_body_maps_to_raw_post() {
    let t = MockTransport::new(|req| {
        assert_eq!(req.method, TransportMethod::Post);
        assert_eq!(req.body, TransportBody::Text("{\"k\":1}".into()));
        assert_eq!(req.url, "https://mock.local/post?access_token=T1");
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":0}"#.to_vec(),
        })
    });
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/post".into(),
        body: TransportBody::Text("{\"k\":1}".into()),
        replay_on_token_invalid: true,
        breaker: None,
    };
    let r: i32 = execute_pipeline(ctx, WxType::MiniApp, |_| Ok(7i32), None)
        .await
        .unwrap();
    assert_eq!(r, 7);
}
