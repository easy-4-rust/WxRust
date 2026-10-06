//! 微信小店配置存储。
//!
//! 对应 Java `com.binarywang.wxjava.store.config` 包。

pub mod r#impl;
pub mod wx_store_config;
pub mod wx_store_host_config;

pub use wx_store_config::{DEFAULT_ACCESS_TOKEN_URL, DEFAULT_API_HOST_URL, WxStoreConfig};

pub use wx_store_host_config::{API_DEFAULT_HOST_URL, WxStoreHostConfig};
