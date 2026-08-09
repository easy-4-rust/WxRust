//! 智能对话配置存储。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.config` 包（`WxAispeechConfigStorage`
//! 接口 + `WxAispeechDefaultConfigImpl` 内存实现）。

pub mod r#impl;
pub mod wx_aispeech_config_storage;

pub use wx_aispeech_config_storage::WxAispeechConfigStorage;
