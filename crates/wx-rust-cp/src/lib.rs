//! WxRust 企业微信模块（对应 WxJava `weixin-java-cp`）。
//!
//! 覆盖 `me.chanjar.weixin.cp.*`：门面服务（access_token/通讯录同步
//! access_token/会话存档 access_token/jsapi_ticket/agent jsapi_ticket 双检锁
//! 缓存、GET/POST 执行引擎与 token 自动刷新）、配置存储、消息加解密、
//! API URL 常量等基础设施；成员/部门/标签/消息/外部联系人等业务子域由
//! 后续波次实现（子服务 trait 目前为空占位，门面 getter 默认返回 `None`）。
//!
//! Wave 0 骨架说明：
//! - `bean/` 仅含门面签名所需的占位 bean（`WxCpAgentJsapiSignature`、
//!   `WxCpMaJsCode2SessionResult`、`WxCpProviderToken`），完整 bean 随
//!   业务子域批次补齐；
//! - `message/`（WxCpMessageRouter 等）与 `api` 各业务子服务随 Wave 1+
//!   补齐。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod config;
pub mod constant;
pub mod corpgroup;
pub mod enums;
pub mod message;
pub mod tp;
pub mod util;
