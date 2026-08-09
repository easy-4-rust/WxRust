//! WxRust 视频号小店模块（对应 WxJava `weixin-java-channel`）。
//!
//! 覆盖 `me.chanjar.weixin.channel.*`：门面服务（access_token 双检锁缓存、
//! GET/POST 执行引擎与 token 自动刷新）、配置存储、URL/枚举常量等基础设施；
//! 商品/订单/售后/资金/联盟/合作等业务子域方法已在门面 trait 冻结签名
//! （Wave 0 B0 冻结，默认 `Err(-99)` 占位），真实实现由后续波次按子域补齐。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod config;
pub mod constant;
pub mod enums;
pub mod message;
pub mod util;
