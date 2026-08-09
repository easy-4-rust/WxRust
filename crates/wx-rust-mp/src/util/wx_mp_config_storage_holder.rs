//! 公众号配置存储 ThreadLocal 持有器。
//!
//! 对应 Java `me.chanjar.weixin.mp.util.WxMpConfigStorageHolder`：ThreadLocal
//! 记录当前 appid，默认 `"default"`。Rust 以 `thread_local!` 表达同一语义
//! （线程局部；线程池/异步多线程运行时中跨线程丢失，与 Java 线程池一致）。

use std::cell::RefCell;

thread_local! {
    /// 当前公众号 appid 标记（默认 `"default"`）。
    static MP_APPID: RefCell<String> = RefCell::new("default".to_string());
}

/// 读取当前 appid 标记；未设置时返回 `"default"`。
pub fn get() -> String {
    MP_APPID.with(|c| c.borrow().clone())
}

/// 设置当前 appid 标记。
pub fn set(label: impl Into<String>) {
    MP_APPID.with(|c| *c.borrow_mut() = label.into());
}

/// 清除当前 appid 标记。
pub fn remove() {
    MP_APPID.with(|c| *c.borrow_mut() = "default".to_string());
}
