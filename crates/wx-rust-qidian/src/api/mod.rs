//! 腾讯企点 API。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api` 包：门面服务 `WxQidianService`
//! （复用 mp/cp 模式的 access_token/ticket 执行引擎）与
//! `WxQidianDialService` / `WxQidianCallDataService` 两个子域。

pub mod r#impl;
pub mod wx_qidian_call_data_service;
pub mod wx_qidian_dial_service;
pub mod wx_qidian_service;

pub use wx_qidian_call_data_service::WxQidianCallDataService;
pub use wx_qidian_dial_service::WxQidianDialService;
pub use wx_qidian_service::WxQidianService;
