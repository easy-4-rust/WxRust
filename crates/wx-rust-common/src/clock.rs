//! 可注入时钟抽象（WxClock）。
//!
//! 用途：token/ticket 过期判断的时间来源可注入——默认 [`SystemClock`]
//! 与直接读 `SystemTime` 逐字节一致；测试注入 [`FakeClock`] 手动推进
//! 时间即可验证过期翻转，零 sleep（计划 Task 8）。
//!
//! 单位约定：`now_ms` 返回 UNIX 纪元毫秒；config 侧过期存储为 UNIX 秒，
//! 换算 `now_ms() / 1000`（默认路径与 `SystemTime::as_secs()` 相等）。
//!
//! 注入方式：实例级注入（如 `WxDefaultConfig::set_clock`），不做进程级
//! 全局时钟——全局可变静态会波及所有配置实例并在测试并行时互相干扰。

use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

/// 时钟抽象（可注入）。
///
/// `Debug` 为超 trait：携带 clock 的配置结构（如 `WxDefaultConfig`）
/// 可继续 derive `Debug`。
pub trait WxClock: Send + Sync + Debug {
    /// 当前时间（UNIX 纪元毫秒）。
    fn now_ms(&self) -> i64;
}

/// 系统真实时钟（默认实现，行为与直接读 `SystemTime` 一致）。
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl WxClock for SystemClock {
    fn now_ms(&self) -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }
}

/// 手动推进的假时钟（仅测试使用）。
///
/// 内部 `Arc<AtomicI64>` 共享时间源：`clone()` 后各句柄推进同一时钟，
/// 测试句柄与注入 config 的实例互通。
#[derive(Debug, Clone)]
pub struct FakeClock(pub Arc<AtomicI64>);

impl FakeClock {
    /// 构建起始时间为 `start_ms`（UNIX 毫秒）的假时钟。
    pub fn new(start_ms: i64) -> Self {
        Self(Arc::new(AtomicI64::new(start_ms)))
    }

    /// 推进时间（毫秒；负值即回拨）。
    pub fn advance_ms(&self, delta: i64) {
        self.0.fetch_add(delta, Ordering::SeqCst);
    }
}

impl WxClock for FakeClock {
    fn now_ms(&self) -> i64 {
        self.0.load(Ordering::SeqCst)
    }
}
