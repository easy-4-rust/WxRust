//! 开放平台（第三方平台）配置存储。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenConfigStorage` 包。

pub mod r#impl;
pub mod wx_open_config_storage;
pub mod wx_open_host_config;

pub use wx_open_config_storage::WxOpenConfigStorage;
pub use wx_open_host_config::{
    API_DEFAULT_HOST_URL, MP_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL, WxOpenHostConfig,
};
