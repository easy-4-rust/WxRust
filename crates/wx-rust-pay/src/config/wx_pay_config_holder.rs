//! 微信支付多商户配置策略持有器。
//!
//! 对应 Java `com.github.binarywang.wxpay.config.WxPayConfigHolder`：
//! `ThreadLocal<String>`（初始 `"default"`）+ `get`/`set`/`remove` 三个
//! 静态方法——多商户场景下按线程选择配置策略（label）。
//!
//! Java 的 `remove()` 清除线程本地值（下次 `get()` 回落初始值
//! `"default"`）；Rust `thread_local!` 无按 key 删除形态，以重置回
//! `"default"` 表达同一语义。

use std::cell::RefCell;

thread_local! {
    /// 当前线程的配置策略标签（对应 Java
    /// `ThreadLocal.withInitial(() -> "default")`）。
    static LABEL: RefCell<String> = RefCell::new("default".to_string());
}

/// 初始策略标签（对应 Java lambda `() -> "default"`）。
pub const DEFAULT_LABEL: &str = "default";

/// 获取当前线程的微信支付配置策略（对应 Java `WxPayConfigHolder.get()`）。
pub fn get() -> String {
    LABEL.with(|l| l.borrow().clone())
}

/// 设置当前线程的微信支付配置策略（对应 Java
/// `WxPayConfigHolder.set(String label)`）。
pub fn set(label: &str) {
    LABEL.with(|l| *l.borrow_mut() = label.to_string());
}

/// 清除当前线程的配置策略（对应 Java `WxPayConfigHolder.remove()`；
/// 用户需在合适位置手动触发，SDK 无法判断调用时机）。
pub fn remove() {
    LABEL.with(|l| *l.borrow_mut() = DEFAULT_LABEL.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_local_label_lifecycle() {
        // 初始值 default
        assert_eq!(get(), "default");
        set("mch-001");
        assert_eq!(get(), "mch-001");
        remove();
        assert_eq!(get(), "default");
    }

    #[test]
    fn labels_are_thread_local() {
        set("main-label");
        let spawned = std::thread::spawn(|| {
            // 新线程回落初始值，且互不影响
            let initial = get();
            set("worker-label");
            (initial, get())
        })
        .join()
        .unwrap();
        assert_eq!(spawned, ("default".to_string(), "worker-label".to_string()));
        assert_eq!(get(), "main-label");
        remove();
    }
}
