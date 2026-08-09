//! WxRust 腾讯企点模块（对应 WxJava `weixin-java-qidian`）。
//!
//! 覆盖 `me.chanjar.weixin.qidian.*`：门面服务 `WxQidianService`（复用
//! mp/cp 的 access_token/ticket 双检锁缓存与 GET/POST 执行引擎、多配置
//! 切换）、IVR 呼叫（`WxQidianDialService`）、通话数据
//! （`WxQidianCallDataService`）两大子域，以及企点专属的接口地址表
//! （`WxQidianApiUrl`，api.qidian.qq.com 主机前缀）与响应基类
//! `QidianResponse`。token 语义与 mp 模块一致（
//! `https://api.qidian.qq.com/cgi-bin/token`）。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod config;
pub mod enums;
pub mod util;
