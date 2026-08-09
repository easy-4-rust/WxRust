//! 腾讯企点配置存储。
//!
//! 对应 Java `me.chanjar.weixin.qidian.config` 包（`WxQidianConfigStorage`
//! 接口 + `WxQidianDefaultConfigImpl` 内存实现；Redis/Redisson 外部存储
//! 实现为 `PLATFORM_NA`，见台账）。

pub mod r#impl;
pub mod wx_qidian_config_storage;

pub use wx_qidian_config_storage::WxQidianConfigStorage;
