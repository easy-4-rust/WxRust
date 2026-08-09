//! 腾讯企点服务实现。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api.impl` 包（HTTP 多后端适配类
//! `WxQidianServiceHttpClientImpl`/`HttpComponentsImpl`/`JoddHttpImpl`/
//! `OkHttpImpl` 为 `PLATFORM_NA`，reqwest 统一承载，见台账）。

pub mod base_wx_qidian_service_impl;
pub mod wx_qidian_call_data_service_impl;
pub mod wx_qidian_dial_service_impl;
pub mod wx_qidian_service_impl;

pub use base_wx_qidian_service_impl::{execute_internal, execute_with_retry};
pub use wx_qidian_call_data_service_impl::WxQidianCallDataServiceImpl;
pub use wx_qidian_dial_service_impl::WxQidianDialServiceImpl;
pub use wx_qidian_service_impl::WxQidianServiceImpl;
