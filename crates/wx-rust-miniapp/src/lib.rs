//! WxRust 小程序模块（对应 WxJava `weixin-java-miniapp`）。
//!
//! 覆盖 `cn.binarywang.wx.miniapp.*`：门面服务（access_token 双检锁缓存、
//! GET/POST 执行引擎与 token 自动刷新）、配置存储、消息加解密、URL 常量等
//! 基础设施；用户/消息/二维码等业务子域由后续波次实现。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod builder;
pub mod config;
pub mod constant;
pub mod enums;
pub mod message;
pub mod util;
