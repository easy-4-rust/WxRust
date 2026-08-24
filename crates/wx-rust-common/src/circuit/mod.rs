//! per-host 熔断器（零第三方依赖自写，不引 tower 等）。
//!
//! RUST_OBLIGATION：对应计划 Task 6——「过载有熔断」。状态机：
//!
//! - **Closed**：正常放行；`after(host, false)` 连续失败计数 +1，
//!   达 `failure_threshold` → 转 **Open** 并记 `open_at`；
//! - **Open**：`before` 直接拒绝（错误码 -99、文案
//!   「熔断器开启：&lt;host&gt;」）；`open_at` 起 `open_duration` 过后
//!   转 **HalfOpen**；
//! - **HalfOpen**：`before` 放行探测请求——`after(host, true)` →
//!   Closed 且失败计数清零；`after(host, false)` → 重回 Open 并记新
//!   `open_at`（需再等完整 `open_duration`）。
//!
//! 多 host 状态彼此独立（[`dashmap::DashMap`] 按键分片存储）；整体以
//! 单个 [`tokio::sync::Mutex`] 串行化状态迁移（迁移均为纯内存操作、
//! 无内嵌 `.await`，锁仅跨自身 `lock().await` 持有——满足项目
//! 「不跨 `.await` 持有 std 锁；新增锁一律 `tokio::sync`」约束）。
//!
//! 熔断器为可选件：管线经 [`crate::pipeline::CircuitBreakerLike`]
//! 最小 trait 消费（本类型实现该 trait），`breaker: None` 时管线行为
//! 与无熔断时完全一致。

use std::time::{Duration, Instant};

use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::Mutex;

use crate::error::WxErrorException;
use crate::pipeline::CircuitBreakerLike;

/// 单 host 的熔断相位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    /// 闭合：正常放行，累计连续失败。
    Closed,
    /// 熔断开启：拒绝放行；`open_at` 起 `open_duration` 后可探测。
    Open {
        /// 进入 Open 的时刻。
        open_at: Instant,
    },
    /// 半开：放行探测请求，结果决定回 Closed 或重回 Open。
    HalfOpen,
}

/// 单 host 的熔断状态（相位 + 连续失败计数）。
#[derive(Debug, Clone)]
struct BreakerState {
    phase: Phase,
    consecutive_failures: u32,
}

impl Default for BreakerState {
    fn default() -> Self {
        Self {
            phase: Phase::Closed,
            consecutive_failures: 0,
        }
    }
}

/// per-host 熔断器。
///
/// 构造后经 [`CircuitBreakerLike`]（`before` / `after`）驱动：请求前
/// `before`、请求结束后以最终结果 `after(host, ok)`。用法见
/// `tests/circuit_breaker_test.rs`。
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_threshold: u32,
    open_duration: Duration,
    /// 各 host 独立状态；外层 tokio Mutex 串行化状态迁移。
    hosts: Mutex<DashMap<String, BreakerState>>,
}

impl CircuitBreaker {
    /// 构造熔断器。
    ///
    /// - `failure_threshold`：Closed 态连续失败达该值 → Open；
    /// - `open_duration`：Open 态持续时间，过后进入 HalfOpen 可探测。
    pub fn new(failure_threshold: u32, open_duration: Duration) -> Self {
        Self {
            failure_threshold,
            open_duration,
            hosts: Mutex::new(DashMap::new()),
        }
    }

    /// 请求前调用：Open 且未到探测窗口时拒绝。
    ///
    /// 拒绝错误：`from_code(-99, "熔断器开启：<host>")`。
    pub async fn before(&self, host: &str) -> Result<(), WxErrorException> {
        let hosts = self.hosts.lock().await;
        let mut state = hosts.entry(host.to_string()).or_default();
        match state.phase {
            Phase::Closed => Ok(()),
            Phase::Open { open_at } => {
                if open_at.elapsed() >= self.open_duration {
                    // 冷却期满 → HalfOpen，放行本次探测
                    state.phase = Phase::HalfOpen;
                    Ok(())
                } else {
                    Err(WxErrorException::from_code(
                        -99,
                        format!("熔断器开启：{host}"),
                    ))
                }
            }
            Phase::HalfOpen => Ok(()),
        }
    }

    /// 请求后调用：`ok` 为最终结果（成功复位、失败计数/重开）。
    ///
    /// 首次接触的 host 视为 Closed 起步——`after` 可独立于 `before`
    /// 驱动状态机（计数/熔断）。
    pub async fn after(&self, host: &str, ok: bool) {
        let hosts = self.hosts.lock().await;
        let mut state = hosts.entry(host.to_string()).or_default();
        if ok {
            // 成功复位（Closed 态清零计数；HalfOpen 探测成功 → Closed）
            state.phase = Phase::Closed;
            state.consecutive_failures = 0;
            return;
        }
        match state.phase {
            Phase::Closed => {
                state.consecutive_failures += 1;
                if state.consecutive_failures >= self.failure_threshold {
                    state.phase = Phase::Open {
                        open_at: Instant::now(),
                    };
                    state.consecutive_failures = 0;
                }
            }
            Phase::HalfOpen => {
                // 探测失败 → 重回 Open，记新 open_at（完整冷却重来）
                state.phase = Phase::Open {
                    open_at: Instant::now(),
                };
                state.consecutive_failures = 0;
            }
            Phase::Open { .. } => {
                // 已开启：维持原 open_at（不因后续失败延长冷却）
            }
        }
    }
}

#[async_trait]
impl CircuitBreakerLike for CircuitBreaker {
    async fn before(&self, host: &str) -> Result<(), WxErrorException> {
        CircuitBreaker::before(self, host).await
    }

    async fn after(&self, host: &str, ok: bool) {
        CircuitBreaker::after(self, host, ok).await
    }
}
