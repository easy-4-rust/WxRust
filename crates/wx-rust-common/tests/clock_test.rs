//! WxClock 时钟注入测试。
//!
//! RUST_OBLIGATION：可注入时间源——token 过期判断改经 `WxClock` 后，
//! 默认 `SystemClock` 行为逐字节不变；测试注入 `FakeClock` 手动推进时间，
//! 过期翻转验证零 sleep（计划 Task 8）。

use std::sync::Arc;

use wx_rust_common::clock::{FakeClock, SystemClock, WxClock};
use wx_rust_common::config::{WxConfigStorage, WxDefaultConfig};

// ==================== RUST_OBLIGATION ====================

// --- 时钟义务：SystemClock 单调合理且为 UNIX 毫秒量级 ---
// Java 语义：System.currentTimeMillis() 为 UNIX 毫秒；两次调用差 >= 0。
#[test]
fn system_clock_now_ms_monotonic_and_epoch_scale() {
    let clock = SystemClock;
    let t1 = clock.now_ms();
    let t2 = clock.now_ms();
    assert!(t2 >= t1, "两次调用不回拨：t1={t1} t2={t2}");
    // UNIX 毫秒量级（约 1.7e12，容差 1.0e12 ~ 3.0e12 防时钟异常误报）
    assert!(
        (1_000_000_000_000..3_000_000_000_000).contains(&t1),
        "UNIX 毫秒量级：{t1}"
    );
}

// --- 时钟义务：FakeClock advance 后 now_ms 反映推进 ---
// 同一 Arc 时间源 clone 后互通（测试句柄推进，注入侧可见）。
#[test]
fn fake_clock_advance_changes_now_ms() {
    let fake = FakeClock::new(1_700_000_000_000);
    assert_eq!(fake.now_ms(), 1_700_000_000_000);
    fake.advance_ms(500);
    assert_eq!(fake.now_ms(), 1_700_000_000_500);
    // clone 共享同一时间源
    let handle = fake.clone();
    handle.advance_ms(1_000);
    assert_eq!(fake.now_ms(), 1_700_000_001_500);
}

// --- 时钟义务：token 过期翻转零 sleep（端到端） ---
// Java 语义：expires_at = now + expires_in；读取时 now >= expires_at 即过期。
// 注入 FakeClock 后 advance 越过 expires_in 即翻转，无需真实等待。
#[test]
fn fake_clock_token_expiry_flips_without_sleep() {
    let fake = FakeClock::new(0);
    let config = WxDefaultConfig::new("appid", "secret");
    assert!(config.set_clock(Arc::new(fake.clone())), "首次注入生效");

    config.update_access_token("token-A", 7200);
    assert!(!config.is_access_token_expired(), "未 advance 时不过期");

    fake.advance_ms((7200 + 1) * 1000);
    assert!(
        config.is_access_token_expired(),
        "advance 越过 expires_in 后翻转"
    );
    // 二次注入在首次读取时间后不再生效（OnceLock 语义）
    assert!(!config.set_clock(Arc::new(FakeClock::new(0))));
}

// --- 时钟义务：默认路径 SystemClock 行为不变 ---
// 未注入时钟的配置仍按真实系统时间判断（与改造前逐字节一致）。
#[test]
fn default_config_uses_system_clock_unchanged() {
    let config = WxDefaultConfig::new("appid", "secret");
    // 未设置 token：视为过期（原语义）
    assert!(config.is_access_token_expired());
    config.update_access_token("token-B", 7200);
    assert!(!config.is_access_token_expired(), "默认时钟下 7200s 未过期");
}
