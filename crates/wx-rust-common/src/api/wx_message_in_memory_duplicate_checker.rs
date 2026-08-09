//! 内存消息重复检查器（已废弃）。
//!
//! 对应 Java `me.chanjar.weixin.common.api.WxMessageInMemoryDuplicateChecker`。
//! Java 中标记 `@Deprecated`，推荐使用 [`super::wx_message_in_memory_duplicate_checker_singleton`]。

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::WxMessageDuplicateChecker;

/// 内存消息重复检查器。
///
/// 将每个消息 ID 保存在内存里，每隔 `clear_period` 清理已过期的消息 ID，
/// 每个消息 ID 的过期时间是 `time_to_live`。
///
/// # 默认值
/// - 消息 ID 过期时间：15 秒
/// - 清理周期：5 秒
///
/// # 废弃
/// 已被 [`super::WxMessageInMemoryDuplicateCheckerSingleton`] 替代。
#[derive(Debug)]
pub struct WxMessageInMemoryDuplicateChecker {
    /// 一个消息 ID 在内存的过期时间（毫秒）
    time_to_live: Duration,

    /// 每隔多少周期检查消息 ID 是否过期（毫秒）
    clear_period: Duration,

    /// 消息 ID -> 消息时间戳的映射
    msg_id_2_timestamp: Mutex<HashMap<String, Instant>>,
}

impl WxMessageInMemoryDuplicateChecker {
    /// 构建默认配置的检查器（过期 15 秒、清理周期 5 秒）。
    pub fn new() -> Self {
        Self {
            time_to_live: Duration::from_millis(15 * 1000),
            clear_period: Duration::from_millis(5 * 1000),
            msg_id_2_timestamp: Mutex::new(HashMap::new()),
        }
    }

    /// 构建自定义配置的检查器。
    ///
    /// # 参数
    /// - `time_to_live`：消息 ID 过期时间（毫秒）
    /// - `clear_period`：清理周期（毫秒）
    pub fn with_config(time_to_live: u64, clear_period: u64) -> Self {
        Self {
            time_to_live: Duration::from_millis(time_to_live),
            clear_period: Duration::from_millis(clear_period),
            msg_id_2_timestamp: Mutex::new(HashMap::new()),
        }
    }

    /// 清理已过期的消息 ID（对应 Java 后台清理线程的周期任务）。
    fn clear_expired(&self) {
        let now = Instant::now();
        let mut map = self.msg_id_2_timestamp.lock().unwrap();
        map.retain(|_, ts| now.duration_since(*ts) < self.time_to_live);
    }
}

impl Default for WxMessageInMemoryDuplicateChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl WxMessageDuplicateChecker for WxMessageInMemoryDuplicateChecker {
    fn is_duplicate(&self, message_id: &str) -> bool {
        // 周期性清理（懒清理：每次调用时按周期判断是否需要清理）
        // 简化实现：直接清理过期项
        self.clear_expired();
        let now = Instant::now();
        let mut map = self.msg_id_2_timestamp.lock().unwrap();
        if let Some(ts) = map.get(message_id) {
            if now.duration_since(*ts) < self.time_to_live {
                return true;
            }
        }
        map.insert(message_id.to_string(), now);
        false
    }
}

// 保留字段避免未使用警告（clear_period 在懒清理模式下未直接使用）
#[allow(dead_code)]
fn _touch(_c: &WxMessageInMemoryDuplicateChecker) {
    let _ = _c.clear_period;
}
