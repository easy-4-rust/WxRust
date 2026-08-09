//! 视频号小店配置存储。
//!
//! 对应 Java `me.chanjar.weixin.channel.config` 包。

pub mod r#impl;
pub mod wx_channel_config;
pub mod wx_channel_host_config;

pub use wx_channel_config::{DEFAULT_ACCESS_TOKEN_URL, DEFAULT_API_HOST_URL, WxChannelConfig};
pub use wx_channel_host_config::{API_DEFAULT_HOST_URL, WxChannelHostConfig};
