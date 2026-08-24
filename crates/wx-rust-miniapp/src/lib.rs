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

/// 同步门面（feature="sync"）：`WxMaServiceBlocking` 以全局 current_thread
/// runtime 逐调用驱动 async 门面，供纯同步上下文使用。runtime 阻塞驱动
/// 仅允许出现在本模块（CI 以 scripts 门禁脚本强制）。
#[cfg(feature = "sync")]
pub mod blocking;
