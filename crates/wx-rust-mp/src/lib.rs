//! WxRust 公众号模块（对应 WxJava `weixin-java-mp`）。
//!
//! 覆盖 `me.chanjar.weixin.mp.*`：门面服务、消息路由、XML 消息收发加解密、
//! 菜单/模板消息/二维码/客服/用户等核心子域。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod builder;
pub mod config;
pub mod constant;
pub mod enums;
pub mod util;
