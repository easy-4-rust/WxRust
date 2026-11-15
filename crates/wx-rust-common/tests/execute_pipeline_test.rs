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
use wx_rust_common::pipeline::{
    PipelineContext, execute_pipeline, execute_pipeline_with_refresh_and_query,
};

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

/// GET queryParam must be appended after access_token (Java executeInternal -> SimpleGetRequestExecutor).
#[tokio::test]
async fn get_query_param_is_appended_after_access_token() {
    let t = MockTransport::new(|req| {
        assert_eq!(req.method, TransportMethod::Get);
        assert_eq!(
            req.url,
            "https://mock.local/get?base=1&access_token=T1&trace=wp03&scene=get"
        );
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":0,"errmsg":"ok"}"#.to_vec(),
        })
    });
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/get?base=1".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: None,
    };
    let result: String = execute_pipeline_with_refresh_and_query(
        ctx,
        WxType::MiniApp,
        |response| Ok(String::from_utf8_lossy(&response.body).into_owned()),
        None,
        None,
        Some("trace=wp03&scene=get"),
    )
    .await
    .unwrap();
    assert_eq!(result, r#"{"errcode":0,"errmsg":"ok"}"#);
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

/// SimplePostRequestExecutor returns a response whose body starts with <xml> unchanged.
#[tokio::test]
async fn post_xml_response_is_returned_raw_without_wx_error_json_parse() {
    let xml = "<xml><return_code>SUCCESS</return_code><return_msg>ok</return_msg></xml>";
    let t = MockTransport::new(move |req| {
        assert_eq!(req.method, TransportMethod::Post);
        assert_eq!(
            req.headers,
            vec![(
                "Content-Type".to_string(),
                "application/json; charset=UTF-8".to_string()
            )]
        );
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: xml.as_bytes().to_vec(),
        })
    });
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/post".into(),
        body: TransportBody::Text(r#"{"probe":"xml-response"}"#.into()),
        replay_on_token_invalid: true,
        breaker: None,
    };
    let result: String = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |response| Ok(String::from_utf8_lossy(&response.body).into_owned()),
        None,
    )
    .await
    .unwrap();
    assert_eq!(result, xml);
}

/// Java SimplePostRequestExecutor maps an empty response body to WxErrorException(-99, "无响应内容").
#[tokio::test]
async fn empty_post_response_returns_wx_error_minus_99() {
    let t = MockTransport::new(|req| {
        assert_eq!(req.method, TransportMethod::Post);
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: Vec::new(),
        })
    });
    let ctx = PipelineContext {
        transport: &t,
        access_token: "T1".into(),
        uri: "https://mock.local/post".into(),
        body: TransportBody::Text(r#"{"probe":"empty-post"}"#.into()),
        replay_on_token_invalid: true,
        breaker: None,
    };
    let result: Result<String, WxErrorException> = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |response| Ok(String::from_utf8_lossy(&response.body).into_owned()),
        None,
    )
    .await;
    let error = result.unwrap_err();
    assert_eq!(error.error_code(), Some(-99));
    assert_eq!(
        error
            .wx_error()
            .and_then(|value| value.error_msg.as_deref()),
        Some("无响应内容")
    );
}

/// Non-2xx HTTP responses are runtime transport failures, not successful JSON payloads.
#[tokio::test]
async fn non_2xx_status_is_rejected_before_errcode_parse() {
    let t = MockTransport::new(|_| {
        Ok(TransportResponse {
            status: 503,
            headers: vec![],
            body: br#"{"errcode":0,"errmsg":"upstream unavailable"}"#.to_vec(),
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
    let result: Result<String, WxErrorException> = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |_| panic!("non-2xx response must not reach the JSON success parser"),
        None,
    )
    .await;
    assert!(matches!(result, Err(WxErrorException::Runtime(_))));
}

#[tokio::test]
async fn empty_http_body_is_a_runtime_response_failure() {
    let t = MockTransport::new(|_| {
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: Vec::new(),
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
    let result: Result<String, WxErrorException> = execute_pipeline(
        ctx,
        WxType::MiniApp,
        |_| panic!("empty response must not reach the JSON success parser"),
        None,
    )
    .await;
    assert!(matches!(result, Err(WxErrorException::Runtime(_))));
}

/// Text body 对应 Java SimplePostRequestExecutor：原样 POST + JSON UTF-8 Content-Type。
#[tokio::test]
async fn text_body_maps_to_json_post_with_utf8_content_type() {
    let t = MockTransport::new(|req| {
        assert_eq!(req.method, TransportMethod::Post);
        assert_eq!(req.body, TransportBody::Text("{\"k\":1}".into()));
        assert_eq!(req.url, "https://mock.local/post?access_token=T1");
        assert_eq!(
            req.headers,
            vec![(
                "Content-Type".to_string(),
                "application/json; charset=UTF-8".to_string()
            )]
        );
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

/// Token 失效后必须刷新并使用新 Token 重建 URL（对齐 Java executeInternal）。
#[tokio::test]
async fn token_invalid_refreshes_token_before_single_replay() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&calls);
    let transport = MockTransport::new(move |req| {
        let n = c.fetch_add(1, Ordering::SeqCst);
        if n == 0 {
            assert_eq!(req.url, "https://mock.local/get?access_token=T1");
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: br#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_vec(),
            })
        } else {
            assert_eq!(n, 1, "Token invalid replay must happen only once");
            assert_eq!(req.url, "https://mock.local/get?access_token=T2");
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: br#"{"errcode":0,"errmsg":"ok","data":42}"#.to_vec(),
            })
        }
    });
    let invalidated = Arc::new(AtomicBool::new(false));
    let refresh_calls = Arc::new(AtomicUsize::new(0));
    let refresh_counter = Arc::clone(&refresh_calls);
    let refresh_access_token = move || -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<String, WxErrorException>> + Send>,
    > {
        let counter = Arc::clone(&refresh_counter);
        Box::pin(async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok("T2".to_string())
        })
    };
    let ctx = PipelineContext {
        transport: &transport,
        access_token: "T1".into(),
        uri: "https://mock.local/get".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: None,
    };

    let result: i32 = wx_rust_common::pipeline::execute_pipeline_with_refresh(
        ctx,
        WxType::MiniApp,
        |resp| {
            let value: serde_json::Value = serde_json::from_slice(&resp.body)?;
            Ok(value["data"].as_i64().unwrap_or_default() as i32)
        },
        Some(&flag_setter(&invalidated)),
        Some(&refresh_access_token),
    )
    .await
    .unwrap();

    assert_eq!(result, 42);
    assert!(invalidated.load(Ordering::SeqCst));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(refresh_calls.load(Ordering::SeqCst), 1);
}
