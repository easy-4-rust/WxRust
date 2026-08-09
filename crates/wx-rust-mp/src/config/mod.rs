//! 公众号配置存储。
//!
//! 对应 Java `me.chanjar.weixin.mp.config` 包。

pub mod r#impl;
pub mod wx_mp_config_storage;
pub mod wx_mp_host_config;

pub use wx_mp_config_storage::WxMpConfigStorage;
pub use wx_mp_host_config::{
    API_DEFAULT_HOST_URL, MP_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL, WxMpHostConfig,
};
