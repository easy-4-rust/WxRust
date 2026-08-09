//! 企业微信第三方应用（服务商）模块。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp` 包：`service`（`WxCpTpService`
//! 门面 + 14 个子服务接口/实现）与 `message`（服务商推送消息路由族
//! `WxCpTpMessageRouter` 等）。

pub mod message;
pub mod service;
