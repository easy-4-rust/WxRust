//! 腾讯企点配置存储 ThreadLocal 持有器。
//!
//! 对应 Java `me.chanjar.weixin.qidian.util.WxQidianConfigStorageHolder`：
//! ThreadLocal 记录当前 mpId，默认 `"default"`。Rust 以 `thread_local!`
//! 表达同一语义（线程局部；异步多线程运行时中跨线程丢失，与 Java 线程池
//! 一致）。

use std::cell::RefCell;

thread_local! {
    /// 当前企点 mpId 标记（默认 `"default"`）。
    static MP_ID: RefCell<String> = RefCell::new("default".to_string());
}

/// 企点配置持有器（对应 Java `WxQidianConfigStorageHolder`）。
pub struct WxQidianConfigStorageHolder;

impl WxQidianConfigStorageHolder {
    /// 读取当前 mpId 标记（对应 Java `get()`）。
    pub fn get() -> String {
        MP_ID.with(|c| c.borrow().clone())
    }

    /// 设置当前 mpId 标记（对应 Java `set(String)`）。
    pub fn set(label: impl Into<String>) {
        MP_ID.with(|c| *c.borrow_mut() = label.into());
    }

    /// 清除当前 mpId 标记（对应 Java `remove()`，需使用者按业务时机手动
    /// 调用）。
    pub fn remove() {
        MP_ID.with(|c| *c.borrow_mut() = "default".to_string());
    }
}
