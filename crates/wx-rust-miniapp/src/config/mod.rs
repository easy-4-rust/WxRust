//! 小程序配置存储。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.config` 包。

pub mod r#impl;
pub mod wx_ma_config;
pub mod wx_ma_host_config;

pub use wx_ma_config::{CLOUD_RUN_API_HOST_URL, DEFAULT_API_HOST_URL, WxMaConfig};
pub use wx_ma_host_config::{
    API_DEFAULT_HOST_URL, MP_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL, WxMaHostConfig,
};
