//! 内存消息重复检查器单例。
//!
//! 对应 Java `me.chanjar.weixin.common.api.WxMessageInMemoryDuplicateCheckerSingleton`。

use once_cell::sync::Lazy;

use super::{WxMessageDuplicateChecker, WxMessageInMemoryDuplicateChecker};

/// 内存消息重复检查器单例。
///
/// 使用 `Lazy` 实现进程内唯一实例（对应 Java 静态单例）。
pub struct WxMessageInMemoryDuplicateCheckerSingleton;

/// 全局唯一的内存重复检查器实例。
pub static INSTANCE: Lazy<WxMessageInMemoryDuplicateChecker> =
    Lazy::new(WxMessageInMemoryDuplicateChecker::new);

impl WxMessageInMemoryDuplicateCheckerSingleton {
    /// 返回全局唯一实例。
    pub fn get_instance() -> &'static WxMessageInMemoryDuplicateChecker {
        &INSTANCE
    }
}

impl WxMessageDuplicateChecker for WxMessageInMemoryDuplicateCheckerSingleton {
    fn is_duplicate(&self, message_id: &str) -> bool {
        INSTANCE.is_duplicate(message_id)
    }
}
