//! `CircuitBreaker` per-host 熔断器测试。
//!
//! RUST_OBLIGATION：Closed（连续失败达阈值）→ Open（before 拒绝，
//! open_duration 过后进 HalfOpen）→ HalfOpen（探测成功复位 Closed /
//! 失败重回 Open）；多 host 状态彼此独立；管线可选接入
//! （`breaker: None` 行为与现状完全一致，熔断期间零 transport 调用）。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use wx_rust_common::circuit::CircuitBreaker;
use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::http::{MockTransport, TransportBody, TransportResponse};
use wx_rust_common::pipeline::{PipelineContext, execute_pipeline};

const HOST_A: &str = "api.a.invalid";
const HOST_B: &str = "api.b.invalid";

/// 阈值 2：两次失败后第三次 before 拒绝（错误码 -99、文案含「熔断器开启」）。
#[tokio::test]
async fn opens_after_threshold_and_rejects() {
    let cb = CircuitBreaker::new(2, Duration::from_millis(100));
    cb.after(HOST_A, false).await;
    // 阈值未达：仍放行
    assert!(cb.before(HOST_A).await.is_ok());
    cb.after(HOST_A, false).await;
    // 达阈值 → Open
    let err = cb.before(HOST_A).await.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("熔断器开启"));
    assert!(err.to_string().contains(HOST_A));
}

/// Open 经 open_duration 后进入 HalfOpen 放行一次探测；探测成功 →
/// Closed（后续 before 通过、失败计数清零）。
#[tokio::test]
async fn half_open_allows_probe_then_closes() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(100));
    cb.after(HOST_A, false).await;
    assert!(cb.before(HOST_A).await.is_err());
    tokio::time::sleep(Duration::from_millis(110)).await;
    // HalfOpen：放行探测
    assert!(cb.before(HOST_A).await.is_ok());
    // 探测成功 → Closed
    cb.after(HOST_A, true).await;
    assert!(cb.before(HOST_A).await.is_ok());
    // 计数已从零重新累计：阈值 1 下 Closed 态单次失败再次熔断
    cb.after(HOST_A, false).await;
    assert!(cb.before(HOST_A).await.is_err());
}

/// HalfOpen 探测失败 → 重回 Open（记新 open_at，需再等 open_duration）。
#[tokio::test]
async fn half_open_probe_failure_reopens() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(100));
    cb.after(HOST_A, false).await;
    tokio::time::sleep(Duration::from_millis(110)).await;
    assert!(cb.before(HOST_A).await.is_ok());
    // 探测失败 → 重新 Open
    cb.after(HOST_A, false).await;
    assert!(cb.before(HOST_A).await.is_err());
}

/// 多 host 状态彼此独立：host A 熔断不影响 host B。
#[tokio::test]
async fn hosts_are_isolated() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(100));
    cb.after(HOST_A, false).await;
    assert!(cb.before(HOST_A).await.is_err());
    // host B 不受影响：放行且可正常复位
    assert!(cb.before(HOST_B).await.is_ok());
    cb.after(HOST_B, true).await;
    assert!(cb.before(HOST_B).await.is_ok());
    // host A 依旧熔断
    assert!(cb.before(HOST_A).await.is_err());
}

/// 管线接入：breaker=Some 且已熔断 → transport 零调用、返回熔断错误。
///
/// 注：管线以 `scheme://host[:port]` 为熔断键（uri
/// `https://mock.local/get` → `https://mock.local`）。
#[tokio::test]
async fn pipeline_respects_breaker() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(100));
    cb.after("https://mock.local", false).await;

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
        uri: "https://mock.local/get".into(),
        body: TransportBody::None,
        replay_on_token_invalid: true,
        breaker: Some(&cb),
    };
    let res: Result<i32, WxErrorException> =
        execute_pipeline(ctx, WxType::MiniApp, |_| Ok(0i32), None).await;
    let err = res.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("熔断器开启"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

/// breaker=None（默认关闭）：行为与现状完全一致（回归）。
#[tokio::test]
async fn pipeline_without_breaker_unchanged() {
    let calls = Arc::new(AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: br#"{"errcode":0,"data":42}"#.to_vec(),
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
